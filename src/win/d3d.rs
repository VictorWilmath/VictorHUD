//! Direct3D 11 device on the headset's GPU, plus the textures overlays render into.

use anyhow::Context;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_UNKNOWN, D3D_FEATURE_LEVEL_11_0};
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT, DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_SAMPLE_DESC};
use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory1, IDXGIAdapter, IDXGIAdapter1, IDXGIFactory1};
use windows::core::Interface;

#[derive(Clone)]
pub struct Gpu {
    pub device: ID3D11Device,
    pub context: ID3D11DeviceContext,
}

impl Gpu {
    /// Creates a device on DXGI adapter `index` (from `IVRSystem::GetDXGIOutputInfo`), falling
    /// back to the default adapter.
    pub fn new(adapter_index: i32) -> anyhow::Result<Self> {
        let adapter = if adapter_index >= 0 { adapter_by_index(adapter_index as u32).ok() } else { None };
        match adapter {
            Some(adapter) => Self::on_adapter(&adapter),
            None => Self::create(None),
        }
    }

    pub fn on_adapter(adapter: &IDXGIAdapter1) -> anyhow::Result<Self> {
        Self::create(Some(adapter.cast::<IDXGIAdapter>()?))
    }

    fn create(adapter: Option<IDXGIAdapter>) -> anyhow::Result<Self> {
        let mut device = None;
        let mut context = None;
        let driver = if adapter.is_some() { D3D_DRIVER_TYPE_UNKNOWN } else { D3D_DRIVER_TYPE_HARDWARE };
        unsafe {
            D3D11CreateDevice(
                adapter.as_ref(),
                driver,
                HMODULE::default(),
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                Some(&[D3D_FEATURE_LEVEL_11_0]),
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                Some(&mut context),
            )
        }
        .context("creating a Direct3D 11 device (is your GPU driver up to date?)")?;
        let device: ID3D11Device = device.context("no D3D11 device")?;
        let context = context.context("no D3D11 context")?;
        Ok(Self { device, context })
    }

    pub fn texture(
        &self,
        width: u32,
        height: u32,
        format: DXGI_FORMAT,
        bind: D3D11_BIND_FLAG,
        misc: u32,
    ) -> anyhow::Result<ID3D11Texture2D> {
        let desc = D3D11_TEXTURE2D_DESC {
            Width: width.max(1),
            Height: height.max(1),
            MipLevels: 1,
            ArraySize: 1,
            Format: format,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: bind.0 as u32,
            CPUAccessFlags: 0,
            MiscFlags: misc,
        };
        let mut texture = None;
        unsafe { self.device.CreateTexture2D(&desc, None, Some(&mut texture)) }.context("creating a texture")?;
        texture.context("no texture")
    }

    pub fn shader_view(&self, texture: &ID3D11Texture2D) -> anyhow::Result<ID3D11ShaderResourceView> {
        let mut view = None;
        unsafe { self.device.CreateShaderResourceView(texture, None, Some(&mut view)) }?;
        view.context("no shader resource view")
    }
}

pub fn adapter_by_index(index: u32) -> anyhow::Result<IDXGIAdapter1> {
    let factory: IDXGIFactory1 = unsafe { CreateDXGIFactory1() }?;
    Ok(unsafe { factory.EnumAdapters1(index) }?)
}

/// A texture an overlay displays. It is shared so the SteamVR compositor can read it.
pub struct RenderTarget {
    pub texture: ID3D11Texture2D,
    pub view: ID3D11RenderTargetView,
    pub size: [u32; 2],
}

impl RenderTarget {
    pub fn new(gpu: &Gpu, width: u32, height: u32) -> anyhow::Result<Self> {
        let texture = gpu.texture(
            width,
            height,
            DXGI_FORMAT_R8G8B8A8_UNORM,
            D3D11_BIND_FLAG(D3D11_BIND_RENDER_TARGET.0 | D3D11_BIND_SHADER_RESOURCE.0),
            D3D11_RESOURCE_MISC_SHARED.0 as u32,
        )?;
        let mut view = None;
        unsafe { gpu.device.CreateRenderTargetView(&texture, None, Some(&mut view)) }?;
        Ok(Self { texture, view: view.context("no render target view")?, size: [width.max(1), height.max(1)] })
    }

    /// The raw pointer OpenVR's `Texture_t::handle` expects for D3D11.
    pub fn raw(&self) -> *mut std::ffi::c_void {
        self.texture.as_raw()
    }
}
