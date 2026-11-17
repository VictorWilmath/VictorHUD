//! egui renderer for Direct3D 11. Renders into an overlay's texture in gamma space with
//! premultiplied alpha, which is what SteamVR expects with `ColorSpace_Gamma` and
//! `VROverlayFlags_IsPremultiplied`.

use std::collections::HashMap;
use std::rc::Rc;

use anyhow::{Context, bail};
use egui::epaint::{ClippedPrimitive, Primitive, Vertex};
use egui::{ImageData, TextureFilter, TextureId, TexturesDelta};
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Direct3D::Fxc::{D3DCOMPILE_OPTIMIZATION_LEVEL3, D3DCompile};
use windows::Win32::Graphics::Direct3D::{D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST, ID3DBlob};
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::core::{PCSTR, s};

use super::d3d::{Gpu, RenderTarget};

const SHADER: &str = r#"
cbuffer Constants : register(b0) { float2 screen_size; float opaque; float pad; };
struct VsIn { float2 pos : POSITION; float2 uv : TEXCOORD0; float4 color : COLOR0; };
struct VsOut { float4 pos : SV_POSITION; float2 uv : TEXCOORD0; float4 color : COLOR0; };
VsOut vs_main(VsIn input) {
    VsOut output;
    output.pos = float4(input.pos.x / screen_size.x * 2.0 - 1.0, 1.0 - input.pos.y / screen_size.y * 2.0, 0.0, 1.0);
    output.uv = input.uv;
    output.color = input.color;
    return output;
}
Texture2D tex : register(t0);
SamplerState samp : register(s0);
float4 ps_main(VsOut input) : SV_TARGET {
    float4 texel = tex.Sample(samp, input.uv);
    if (opaque > 0.5) texel.a = 1.0;
    return input.color * texel;
}
"#;

#[repr(C)]
struct Constants {
    screen_size: [f32; 2],
    opaque: f32,
    pad: f32,
}

/// Shaders and fixed state, shared by every panel.
pub struct Pipeline {
    gpu: Gpu,
    vertex_shader: ID3D11VertexShader,
    pixel_shader: ID3D11PixelShader,
    layout: ID3D11InputLayout,
    linear: ID3D11SamplerState,
    nearest: ID3D11SamplerState,
    blend: ID3D11BlendState,
    raster: ID3D11RasterizerState,
    constants: ID3D11Buffer,
}

fn compile(entry: PCSTR, target: PCSTR) -> anyhow::Result<Vec<u8>> {
    let mut code: Option<ID3DBlob> = None;
    let mut errors: Option<ID3DBlob> = None;
    let result = unsafe {
        D3DCompile(
            SHADER.as_ptr().cast(),
            SHADER.len(),
            s!("victorhud.hlsl"),
            None,
            None,
            entry,
            target,
            D3DCOMPILE_OPTIMIZATION_LEVEL3,
            0,
            &mut code,
            Some(&mut errors),
        )
    };
    if let Err(err) = result {
        let detail = errors
            .map(|blob| unsafe {
                String::from_utf8_lossy(std::slice::from_raw_parts(
                    blob.GetBufferPointer() as *const u8,
                    blob.GetBufferSize(),
                ))
                .into_owned()
            })
            .unwrap_or_default();
        bail!("compiling the UI shader failed: {err} {detail}");
    }
    let blob = code.context("shader compiler returned nothing")?;
    Ok(unsafe { std::slice::from_raw_parts(blob.GetBufferPointer() as *const u8, blob.GetBufferSize()) }.to_vec())
}

impl Pipeline {
    pub fn new(gpu: &Gpu) -> anyhow::Result<Rc<Self>> {
        let device = &gpu.device;
        let vs_code = compile(s!("vs_main"), s!("vs_5_0"))?;
        let ps_code = compile(s!("ps_main"), s!("ps_5_0"))?;
        let mut vertex_shader = None;
        let mut pixel_shader = None;
        let mut layout = None;
        unsafe {
            device.CreateVertexShader(&vs_code, None, Some(&mut vertex_shader))?;
            device.CreatePixelShader(&ps_code, None, Some(&mut pixel_shader))?;
            let elements = [
                input_element(s!("POSITION"), DXGI_FORMAT_R32G32_FLOAT, 0),
                input_element(s!("TEXCOORD"), DXGI_FORMAT_R32G32_FLOAT, 8),
                input_element(s!("COLOR"), DXGI_FORMAT_R8G8B8A8_UNORM, 16),
            ];
            device.CreateInputLayout(&elements, &vs_code, Some(&mut layout))?;
        }

        let sampler = |filter| -> anyhow::Result<ID3D11SamplerState> {
            let desc = D3D11_SAMPLER_DESC {
                Filter: filter,
                AddressU: D3D11_TEXTURE_ADDRESS_CLAMP,
                AddressV: D3D11_TEXTURE_ADDRESS_CLAMP,
                AddressW: D3D11_TEXTURE_ADDRESS_CLAMP,
                MipLODBias: 0.0,
                MaxAnisotropy: 1,
                ComparisonFunc: D3D11_COMPARISON_ALWAYS,
                BorderColor: [0.0; 4],
                MinLOD: 0.0,
                MaxLOD: 0.0,
            };
            let mut state = None;
            unsafe { device.CreateSamplerState(&desc, Some(&mut state)) }?;
            state.context("no sampler")
        };

        let mut blend_desc = D3D11_BLEND_DESC::default();
        blend_desc.RenderTarget[0] = D3D11_RENDER_TARGET_BLEND_DESC {
            BlendEnable: true.into(),
            SrcBlend: D3D11_BLEND_ONE,
            DestBlend: D3D11_BLEND_INV_SRC_ALPHA,
            BlendOp: D3D11_BLEND_OP_ADD,
            SrcBlendAlpha: D3D11_BLEND_ONE,
            DestBlendAlpha: D3D11_BLEND_INV_SRC_ALPHA,
            BlendOpAlpha: D3D11_BLEND_OP_ADD,
            RenderTargetWriteMask: D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8,
        };
        let mut blend = None;
        unsafe { device.CreateBlendState(&blend_desc, Some(&mut blend)) }?;

        let raster_desc = D3D11_RASTERIZER_DESC {
            FillMode: D3D11_FILL_SOLID,
            CullMode: D3D11_CULL_NONE,
            ScissorEnable: true.into(),
            DepthClipEnable: false.into(),
            ..Default::default()
        };
        let mut raster = None;
        unsafe { device.CreateRasterizerState(&raster_desc, Some(&mut raster)) }?;

        let constants_desc = D3D11_BUFFER_DESC {
            ByteWidth: size_of::<Constants>() as u32,
            Usage: D3D11_USAGE_DYNAMIC,
            BindFlags: D3D11_BIND_CONSTANT_BUFFER.0 as u32,
            CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
            MiscFlags: 0,
            StructureByteStride: 0,
        };
        let mut constants = None;
        unsafe { device.CreateBuffer(&constants_desc, None, Some(&mut constants)) }?;

        Ok(Rc::new(Self {
            gpu: gpu.clone(),
            vertex_shader: vertex_shader.context("no vertex shader")?,
            pixel_shader: pixel_shader.context("no pixel shader")?,
            layout: layout.context("no input layout")?,
            linear: sampler(D3D11_FILTER_MIN_MAG_MIP_LINEAR)?,
            nearest: sampler(D3D11_FILTER_MIN_MAG_MIP_POINT)?,
            blend: blend.context("no blend state")?,
            raster: raster.context("no rasterizer state")?,
            constants: constants.context("no constant buffer")?,
        }))
    }
}

fn input_element(name: PCSTR, format: DXGI_FORMAT, offset: u32) -> D3D11_INPUT_ELEMENT_DESC {
    D3D11_INPUT_ELEMENT_DESC {
        SemanticName: name,
        SemanticIndex: 0,
        Format: format,
        InputSlot: 0,
        AlignedByteOffset: offset,
        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
        InstanceDataStepRate: 0,
    }
}

struct GpuTexture {
    texture: Option<ID3D11Texture2D>,
    view: ID3D11ShaderResourceView,
    nearest: bool,
    /// Ignore the texture's alpha channel (desktop captures often have alpha = 0).
    opaque: bool,
}

struct DynamicBuffer {
    buffer: ID3D11Buffer,
    capacity: usize,
}

/// Renders one egui context. Each panel owns one, since egui texture ids are per context.
pub struct Painter {
    pipeline: Rc<Pipeline>,
    textures: HashMap<TextureId, GpuTexture>,
    vertices: Option<DynamicBuffer>,
    indices: Option<DynamicBuffer>,
}

impl Painter {
    pub fn new(pipeline: Rc<Pipeline>) -> Self {
        Self { pipeline, textures: HashMap::new(), vertices: None, indices: None }
    }

    /// Shows an existing GPU texture (e.g. a captured desktop) as `TextureId::User(id)`.
    pub fn set_native_texture(&mut self, id: u64, view: ID3D11ShaderResourceView, opaque: bool) -> TextureId {
        let texture_id = TextureId::User(id);
        self.textures.insert(texture_id, GpuTexture { texture: None, view, nearest: false, opaque });
        texture_id
    }

    pub fn remove_native_texture(&mut self, id: u64) {
        self.textures.remove(&TextureId::User(id));
    }

    /// Uploads `delta.set` and empties it (egui asserts that every delta gets applied).
    pub fn update_textures(&mut self, delta: &mut TexturesDelta) -> anyhow::Result<()> {
        for (id, patches) in delta.set.drain() {
            for patch in patches {
                let ImageData::Color(image) = &patch.image;
                let pixels: &[u8] = bytemuck_color32(&image.pixels);
                let nearest = patch.options.magnification == TextureFilter::Nearest;
                match (patch.pos, self.textures.get(&id)) {
                    (Some([x, y]), Some(existing)) => {
                        let Some(texture) = &existing.texture else { continue };
                        let region = D3D11_BOX {
                            left: x as u32,
                            top: y as u32,
                            front: 0,
                            right: (x + image.size[0]) as u32,
                            bottom: (y + image.size[1]) as u32,
                            back: 1,
                        };
                        unsafe {
                            self.pipeline.gpu.context.UpdateSubresource(
                                texture,
                                0,
                                Some(&region),
                                pixels.as_ptr().cast(),
                                (image.size[0] * 4) as u32,
                                0,
                            )
                        };
                    }
                    _ => {
                        let gpu = &self.pipeline.gpu;
                        let texture = gpu.texture(
                            image.size[0] as u32,
                            image.size[1] as u32,
                            DXGI_FORMAT_R8G8B8A8_UNORM,
                            D3D11_BIND_SHADER_RESOURCE,
                            0,
                        )?;
                        unsafe {
                            gpu.context.UpdateSubresource(
                                &texture,
                                0,
                                None,
                                pixels.as_ptr().cast(),
                                (image.size[0] * 4) as u32,
                                0,
                            )
                        };
                        let view = gpu.shader_view(&texture)?;
                        self.textures.insert(id, GpuTexture { texture: Some(texture), view, nearest, opaque: false });
                    }
                }
            }
        }
        Ok(())
    }

    /// Releases `delta.free` and empties it. Call after painting.
    pub fn free_textures(&mut self, delta: &mut TexturesDelta) {
        for id in delta.free.drain() {
            self.textures.remove(&id);
        }
    }

    fn ensure_buffer(
        &self,
        slot: &mut Option<DynamicBuffer>,
        bytes: usize,
        bind: D3D11_BIND_FLAG,
    ) -> anyhow::Result<ID3D11Buffer> {
        if let Some(existing) = slot
            && existing.capacity >= bytes
        {
            return Ok(existing.buffer.clone());
        }
        let capacity = bytes.next_power_of_two().max(64 * 1024);
        let desc = D3D11_BUFFER_DESC {
            ByteWidth: capacity as u32,
            Usage: D3D11_USAGE_DYNAMIC,
            BindFlags: bind.0 as u32,
            CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
            MiscFlags: 0,
            StructureByteStride: 0,
        };
        let mut buffer = None;
        unsafe { self.pipeline.gpu.device.CreateBuffer(&desc, None, Some(&mut buffer)) }?;
        let buffer = buffer.context("no buffer")?;
        *slot = Some(DynamicBuffer { buffer: buffer.clone(), capacity });
        Ok(buffer)
    }

    fn write_buffer(&self, buffer: &ID3D11Buffer, data: &[u8]) -> anyhow::Result<()> {
        let context = &self.pipeline.gpu.context;
        let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
        unsafe {
            context.Map(buffer, 0, D3D11_MAP_WRITE_DISCARD, 0, Some(&mut mapped))?;
            std::ptr::copy_nonoverlapping(data.as_ptr(), mapped.pData as *mut u8, data.len());
            context.Unmap(buffer, 0);
        }
        Ok(())
    }

    fn set_constants(&self, screen_size: [f32; 2], opaque: bool) -> anyhow::Result<()> {
        let constants = Constants { screen_size, opaque: if opaque { 1.0 } else { 0.0 }, pad: 0.0 };
        let bytes = unsafe {
            std::slice::from_raw_parts((&constants as *const Constants).cast::<u8>(), size_of::<Constants>())
        };
        self.write_buffer(&self.pipeline.constants, bytes)
    }

    /// Clears `target` to transparent and draws `primitives` into it.
    pub fn paint(
        &mut self,
        target: &RenderTarget,
        pixels_per_point: f32,
        primitives: &[ClippedPrimitive],
    ) -> anyhow::Result<()> {
        let pipeline = self.pipeline.clone();
        let context = &pipeline.gpu.context;
        let [width, height] = target.size;
        let screen_points = [width as f32 / pixels_per_point, height as f32 / pixels_per_point];

        let mut vertex_bytes: Vec<u8> = Vec::new();
        let mut index_data: Vec<u32> = Vec::new();
        let mut draws = Vec::new();
        for primitive in primitives {
            let Primitive::Mesh(mesh) = &primitive.primitive else { continue };
            if mesh.indices.is_empty() || !self.textures.contains_key(&mesh.texture_id) {
                continue;
            }
            let base_vertex = vertex_bytes.len() / size_of::<Vertex>();
            vertex_bytes.extend_from_slice(vertices_as_bytes(&mesh.vertices));
            let first_index = index_data.len();
            index_data.extend_from_slice(&mesh.indices);
            draws.push((primitive.clip_rect, mesh.texture_id, first_index, mesh.indices.len(), base_vertex));
        }

        unsafe {
            context.OMSetRenderTargets(Some(&[Some(target.view.clone())]), None);
            context.ClearRenderTargetView(&target.view, &[0.0, 0.0, 0.0, 0.0]);
        }
        if draws.is_empty() {
            return Ok(());
        }

        let mut vertex_slot = self.vertices.take();
        let vertex_buffer = self.ensure_buffer(&mut vertex_slot, vertex_bytes.len(), D3D11_BIND_VERTEX_BUFFER)?;
        self.vertices = vertex_slot;
        let index_bytes = unsafe { std::slice::from_raw_parts(index_data.as_ptr().cast::<u8>(), index_data.len() * 4) };
        let mut index_slot = self.indices.take();
        let index_buffer = self.ensure_buffer(&mut index_slot, index_bytes.len(), D3D11_BIND_INDEX_BUFFER)?;
        self.indices = index_slot;
        self.write_buffer(&vertex_buffer, &vertex_bytes)?;
        self.write_buffer(&index_buffer, index_bytes)?;

        unsafe {
            context.RSSetViewports(Some(&[D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: width as f32,
                Height: height as f32,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            }]));
            context.RSSetState(&pipeline.raster);
            context.OMSetBlendState(&pipeline.blend, Some(&[0.0; 4]), u32::MAX);
            context.IASetInputLayout(&pipeline.layout);
            context.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            let stride = size_of::<Vertex>() as u32;
            let offset = 0u32;
            context.IASetVertexBuffers(0, 1, Some(&Some(vertex_buffer)), Some(&stride), Some(&offset));
            context.IASetIndexBuffer(&index_buffer, DXGI_FORMAT_R32_UINT, 0);
            context.VSSetShader(&pipeline.vertex_shader, None);
            context.PSSetShader(&pipeline.pixel_shader, None);
            context.VSSetConstantBuffers(0, Some(&[Some(pipeline.constants.clone())]));
            context.PSSetConstantBuffers(0, Some(&[Some(pipeline.constants.clone())]));
        }

        let mut current_opaque = None;
        for (clip, texture_id, first_index, count, base_vertex) in draws {
            let Some(texture) = self.textures.get(&texture_id) else { continue };
            let left = (clip.min.x * pixels_per_point).round().clamp(0.0, width as f32) as i32;
            let top = (clip.min.y * pixels_per_point).round().clamp(0.0, height as f32) as i32;
            let right = (clip.max.x * pixels_per_point).round().clamp(0.0, width as f32) as i32;
            let bottom = (clip.max.y * pixels_per_point).round().clamp(0.0, height as f32) as i32;
            if right <= left || bottom <= top {
                continue;
            }
            if current_opaque != Some(texture.opaque) {
                self.set_constants(screen_points, texture.opaque)?;
                current_opaque = Some(texture.opaque);
            }
            let sampler = if texture.nearest { &pipeline.nearest } else { &pipeline.linear };
            unsafe {
                context.RSSetScissorRects(Some(&[RECT { left, top, right, bottom }]));
                context.PSSetShaderResources(0, Some(&[Some(texture.view.clone())]));
                context.PSSetSamplers(0, Some(&[Some(sampler.clone())]));
                context.DrawIndexed(count as u32, first_index as u32, base_vertex as i32);
            }
        }
        unsafe { context.PSSetShaderResources(0, Some(&[None])) };
        Ok(())
    }
}

fn vertices_as_bytes(vertices: &[Vertex]) -> &[u8] {
    // `Vertex` is `#[repr(C)]`: pos (2×f32), uv (2×f32), color (4×u8).
    unsafe { std::slice::from_raw_parts(vertices.as_ptr().cast::<u8>(), std::mem::size_of_val(vertices)) }
}

fn bytemuck_color32(pixels: &[egui::Color32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(pixels.as_ptr().cast::<u8>(), std::mem::size_of_val(pixels)) }
}

const _: () = assert!(size_of::<Vertex>() == 20);

#[cfg(test)]
mod tests {
    use egui::{Color32, Pos2, Rect, pos2, vec2};

    use super::*;

    fn read_back(gpu: &Gpu, target: &RenderTarget) -> Vec<[u8; 4]> {
        let [width, height] = target.size;
        let desc = D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: D3D11_USAGE_STAGING,
            BindFlags: 0,
            CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32,
            MiscFlags: 0,
        };
        let mut staging = None;
        unsafe { gpu.device.CreateTexture2D(&desc, None, Some(&mut staging)) }.unwrap();
        let staging = staging.unwrap();
        let mut pixels = Vec::with_capacity((width * height) as usize);
        unsafe {
            gpu.context.CopyResource(&staging, &target.texture);
            let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
            gpu.context.Map(&staging, 0, D3D11_MAP_READ, 0, Some(&mut mapped)).unwrap();
            for y in 0..height {
                let row = (mapped.pData as *const u8).add((y * mapped.RowPitch) as usize);
                for x in 0..width as usize {
                    let p = row.add(x * 4);
                    pixels.push([*p, *p.add(1), *p.add(2), *p.add(3)]);
                }
            }
            gpu.context.Unmap(&staging, 0);
        }
        pixels
    }

    #[test]
    fn renders_egui_with_premultiplied_alpha() {
        let gpu = match Gpu::new(-1) {
            Ok(gpu) => gpu,
            Err(err) => {
                eprintln!("skipping: no Direct3D 11 device ({err:#})");
                return;
            }
        };
        let pipeline = Pipeline::new(&gpu).expect("shaders compile");
        let mut painter = Painter::new(pipeline);
        let target = RenderTarget::new(&gpu, 64, 32).unwrap();

        let ctx = egui::Context::default();
        let raw = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(64.0, 32.0))),
            ..Default::default()
        };
        let mut full = ctx.run_ui(raw, |ui| {
            let painter = ui.painter();
            painter.rect_filled(
                Rect::from_min_max(pos2(0.0, 0.0), pos2(16.0, 32.0)),
                0.0,
                Color32::from_rgb(255, 0, 0),
            );
            // Half-transparent green, which egui stores premultiplied.
            painter.rect_filled(
                Rect::from_min_max(pos2(16.0, 0.0), pos2(32.0, 32.0)),
                0.0,
                Color32::from_rgba_unmultiplied(0, 255, 0, 128),
            );
        });
        painter.update_textures(&mut full.textures_delta).unwrap();
        let primitives = ctx.tessellate(full.shapes, full.pixels_per_point);
        painter.paint(&target, full.pixels_per_point, &primitives).unwrap();
        painter.free_textures(&mut full.textures_delta);

        let pixels = read_back(&gpu, &target);
        let at = |x: usize, y: usize| pixels[y * 64 + x];
        let close = |a: [u8; 4], b: [u8; 4]| a.iter().zip(b).all(|(&a, b)| a.abs_diff(b) <= 2);
        assert!(close(at(8, 16), [255, 0, 0, 255]), "opaque red: {:?}", at(8, 16));
        assert!(close(at(24, 16), [0, 128, 0, 128]), "premultiplied green: {:?}", at(24, 16));
        assert_eq!(at(48, 16), [0, 0, 0, 0], "untouched pixels stay transparent");
    }
}
