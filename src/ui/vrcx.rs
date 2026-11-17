//! VRCX panel: current instance (from VRChat's log), friends, feed and recent worlds (from the
//! VRCX database, read-only).

use chrono::{DateTime, Local};
use egui::{Align, Layout, RichText, ScrollArea, Sense, Ui, vec2};

use super::icons::{self, Icon};
use super::theme;
use super::widgets;
use super::{PanelInfo, UiCtx};
use crate::services::vrcx::{FeedKind, VrcxSnapshot, VrcxStatus};
use crate::state::Action;
use crate::util::{format_ago, format_duration};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Tab {
    #[default]
    Instance,
    Friends,
    Feed,
    Worlds,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum FeedFilter {
    #[default]
    All,
    Locations,
    OnlineOffline,
    Status,
}

pub fn draw(ui: &mut Ui, ctx: &mut UiCtx, panel: &PanelInfo) {
    let palette = ctx.palette;
    let state = ctx.state;
    let tab_id = egui::Id::new(("vrcx-tab", panel.id.0));
    let mut tab: Tab = ui.data(|d| d.get_temp(tab_id)).unwrap_or_default();

    ui.horizontal(|ui| {
        let online = state.vrcx.online_friends().count();
        let players = state.vrchat.players.len();
        for (value, label) in [
            (Tab::Instance, if players > 0 { format!("Instance  {players}") } else { "Instance".into() }),
            (Tab::Friends, if online > 0 { format!("Friends  {online}") } else { "Friends".into() }),
            (Tab::Feed, "Feed".into()),
            (Tab::Worlds, "Worlds".into()),
        ] {
            if widgets::chip(ui, &palette, &label, tab == value).clicked() {
                tab = value;
            }
        }
    });
    ui.data_mut(|d| d.insert_temp(tab_id, tab));
    ui.add_space(4.0);

    match tab {
        Tab::Instance => instance_tab(ui, ctx),
        other => {
            if !vrcx_status_banner(ui, ctx, &state.vrcx) {
                return;
            }
            match other {
                Tab::Friends => friends_tab(ui, ctx),
                Tab::Feed => feed_tab(ui, ctx, panel),
                Tab::Worlds => worlds_tab(ui, ctx),
                Tab::Instance => unreachable!(),
            }
        }
    }
}

fn ago(now: DateTime<Local>, then: DateTime<Local>) -> String {
    format_ago((now - then).to_std().unwrap_or_default())
}

/// Returns whether the VRCX data is usable.
fn vrcx_status_banner(ui: &mut Ui, ctx: &UiCtx, snapshot: &VrcxSnapshot) -> bool {
    let palette = ctx.palette;
    match &snapshot.status {
        VrcxStatus::Ready => true,
        VrcxStatus::Starting => {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label(RichText::new("Reading the VRCX database…").color(palette.text_dim));
            });
            false
        }
        VrcxStatus::NotFound(path) => {
            widgets::empty_state(
                ui,
                &palette,
                Icon::Users,
                "VRCX database not found",
                &format!("Install VRCX and sign in once, or set its path in Settings.\nLooked for {}", path.display()),
            );
            false
        }
        VrcxStatus::Error(error) => {
            widgets::empty_state(ui, &palette, Icon::Warning, "Couldn't read VRCX", error);
            false
        }
    }
}

fn instance_tab(ui: &mut Ui, ctx: &mut UiCtx) {
    let palette = ctx.palette;
    let state = ctx.state;
    let vrchat = &state.vrchat;
    let Some(instance) = &vrchat.instance else {
        let (title, detail) = if vrchat.log_file.is_none() {
            ("VRChat isn't running", "Join a world in VRChat and its details show up here.")
        } else {
            ("Not in a world", "Waiting for VRChat to join an instance…")
        };
        widgets::empty_state(ui, &palette, Icon::Globe, title, detail);
        return;
    };

    theme::card_frame(&palette).show(ui, |ui| {
        ui.set_width(ui.available_width());
        let world = instance.world_name.as_deref().unwrap_or(&instance.world_id);
        ui.add(egui::Label::new(RichText::new(world).size(21.0).strong()).truncate());
        ui.horizontal(|ui| {
            widgets::pill(ui, instance.access.label(), palette.accent);
            widgets::pill(ui, &instance.region, palette.text_dim);
            widgets::pill(ui, &format!("#{}", instance.instance_name), palette.text_dim);
            let here = (ctx.now - instance.joined_at).to_std().unwrap_or_default();
            ui.label(RichText::new(format!("Here for {}", format_duration(here))).size(13.0).color(palette.text_faint));
        });
        ui.horizontal(|ui| {
            if ui.button("Open world page").clicked() {
                ctx.act(Action::OpenUrl(instance.world_url()));
            }
            if ui.button("Copy invite link").clicked() {
                ctx.act(Action::CopyToClipboard(instance.launch_url()));
            }
        });
    });

    ui.add_space(2.0);
    widgets::section_label(ui, &palette, &format!("In this instance  ·  {}", vrchat.players.len()));
    ScrollArea::vertical().id_salt("players").auto_shrink([false, false]).show(ui, |ui| {
        if vrchat.players.is_empty() {
            ui.label(RichText::new("Nobody else here yet").color(palette.text_faint));
        }
        let mut players: Vec<_> = vrchat.players.iter().collect();
        players.sort_by_key(|p| {
            let friend = p.user_id.as_deref().is_some_and(|id| state.vrcx.is_friend(id));
            (!friend, p.display_name.to_lowercase())
        });
        for player in players {
            let friend = player.user_id.as_deref().is_some_and(|id| state.vrcx.is_friend(id));
            let memo = player.user_id.as_deref().and_then(|id| state.vrcx.memos.get(id));
            ui.horizontal(|ui| {
                widgets::avatar(ui, &player.display_name, 32.0);
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&player.display_name).strong());
                        if friend {
                            widgets::pill(ui, "Friend", palette.good);
                        }
                        if vrchat.local_user.as_deref() == Some(player.display_name.as_str()) {
                            widgets::pill(ui, "You", palette.accent);
                        }
                    });
                    if let Some(memo) = memo {
                        ui.add(egui::Label::new(RichText::new(memo).size(12.5).color(palette.text_faint)).truncate());
                    }
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(RichText::new(ago(ctx.now, player.joined_at)).size(12.5).color(palette.text_faint));
                });
            });
        }
    });
}

fn friends_tab(ui: &mut Ui, ctx: &mut UiCtx) {
    let palette = ctx.palette;
    let state = ctx.state;
    let friends = &state.vrcx.friends;
    if friends.is_empty() {
        widgets::empty_state(ui, &palette, Icon::Users, "No friends found", "VRCX hasn't recorded any friends yet.");
        return;
    }
    let online: Vec<_> = friends.iter().filter(|f| f.likely_online).collect();
    let offline: Vec<_> = friends.iter().filter(|f| !f.likely_online).collect();
    ScrollArea::vertical().id_salt("friends").auto_shrink([false, false]).show(ui, |ui| {
        widgets::section_label(ui, &palette, &format!("Probably online  ·  {}", online.len()));
        if online.is_empty() {
            ui.label(RichText::new("Nobody seems to be online right now").color(palette.text_faint));
        }
        for friend in &online {
            friend_row(ui, ctx, friend, true);
        }
        ui.add_space(6.0);
        egui::CollapsingHeader::new(RichText::new(format!("Offline  ·  {}", offline.len())).color(palette.text_dim))
            .id_salt("offline-friends")
            .show(ui, |ui| {
                for friend in &offline {
                    friend_row(ui, ctx, friend, false);
                }
            });
    });
}

fn friend_row(ui: &mut Ui, ctx: &mut UiCtx, friend: &crate::services::vrcx::FriendEntry, online: bool) {
    let palette = ctx.palette;
    let memo = ctx.state.vrcx.memos.get(&friend.user_id);
    let response = ui
        .horizontal(|ui| {
            widgets::avatar(ui, &friend.display_name, 32.0);
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.horizontal(|ui| {
                    if online {
                        widgets::status_dot(ui, palette.good);
                    }
                    ui.label(RichText::new(&friend.display_name).strong());
                    if let Some(trust) = &friend.trust_level {
                        widgets::pill(ui, trust, palette.text_dim);
                    }
                });
                let detail = match (&friend.last_world, online) {
                    (Some(world), true) => world.clone(),
                    (None, true) => "Private world".to_string(),
                    (Some(world), false) => format!("Last seen in {world}"),
                    (None, false) => "Offline".to_string(),
                };
                ui.add(egui::Label::new(RichText::new(detail).size(12.5).color(palette.text_faint)).truncate());
                if let Some(memo) = memo {
                    ui.add(egui::Label::new(RichText::new(memo).size(12.0).italics().color(palette.text_faint)).truncate());
                }
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if let Some(seen) = friend.last_seen {
                    ui.label(RichText::new(ago(ctx.now, seen)).size(12.5).color(palette.text_faint));
                }
            });
        })
        .response;
    let response = ui.interact(response.rect, response.id.with("open"), Sense::click());
    if response.clicked() {
        ctx.act(Action::OpenUrl(format!("https://vrchat.com/home/user/{}", friend.user_id)));
    }
    response.on_hover_text("Open profile on vrchat.com");
}

fn feed_tab(ui: &mut Ui, ctx: &mut UiCtx, panel: &PanelInfo) {
    let palette = ctx.palette;
    let state = ctx.state;
    let filter_id = egui::Id::new(("vrcx-feed-filter", panel.id.0));
    let mut filter: FeedFilter = ui.data(|d| d.get_temp(filter_id)).unwrap_or_default();
    ui.horizontal(|ui| {
        for (value, label) in [
            (FeedFilter::All, "All"),
            (FeedFilter::Locations, "Locations"),
            (FeedFilter::OnlineOffline, "Online / offline"),
            (FeedFilter::Status, "Status"),
        ] {
            if widgets::chip(ui, &palette, label, filter == value).clicked() {
                filter = value;
            }
        }
    });
    ui.data_mut(|d| d.insert_temp(filter_id, filter));

    let entries: Vec<_> = state
        .vrcx
        .feed
        .iter()
        .filter(|entry| match filter {
            FeedFilter::All => true,
            FeedFilter::Locations => entry.kind == FeedKind::Location,
            FeedFilter::OnlineOffline => matches!(entry.kind, FeedKind::Online | FeedKind::Offline),
            FeedFilter::Status => entry.kind == FeedKind::Status,
        })
        .collect();
    ScrollArea::vertical().id_salt("feed").auto_shrink([false, false]).show(ui, |ui| {
        if entries.is_empty() {
            ui.label(RichText::new("No feed activity yet").color(palette.text_faint));
        }
        for entry in entries {
            let (color, icon) = match entry.kind {
                FeedKind::Location => (palette.accent, Icon::Globe),
                FeedKind::Online => (palette.good, Icon::Check),
                FeedKind::Offline => (palette.text_faint, Icon::Minus),
                FeedKind::Status => (palette.warn, Icon::Bell),
            };
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(vec2(28.0, 28.0), Sense::hover());
                ui.painter().circle_filled(rect.center(), 14.0, color.gamma_multiply(0.18));
                icons::paint(ui.painter(), rect.shrink(7.0), icon, color);
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&entry.display_name).strong());
                        ui.label(RichText::new(entry.kind.label()).size(12.5).color(palette.text_faint));
                    });
                    if !entry.detail.is_empty() {
                        ui.add(egui::Label::new(RichText::new(&entry.detail).size(13.0).color(palette.text_dim)).truncate());
                    }
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if let Some(at) = entry.created_at {
                        ui.label(RichText::new(ago(ctx.now, at)).size(12.5).color(palette.text_faint));
                    }
                });
            });
        }
    });
}

fn worlds_tab(ui: &mut Ui, ctx: &mut UiCtx) {
    let palette = ctx.palette;
    let state = ctx.state;
    if state.vrcx.recent_worlds.is_empty() {
        widgets::empty_state(ui, &palette, Icon::Globe, "No world history yet", "Worlds you visit appear here.");
        return;
    }
    ScrollArea::vertical().id_salt("worlds").auto_shrink([false, false]).show(ui, |ui| {
        for world in &state.vrcx.recent_worlds {
            theme::card_frame(&palette).inner_margin(egui::Margin::symmetric(12, 8)).show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    let (rect, _) = ui.allocate_exact_size(vec2(22.0, 22.0), Sense::hover());
                    icons::paint(ui.painter(), rect, Icon::Globe, palette.accent);
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;
                        ui.add(egui::Label::new(RichText::new(&world.world_name).strong()).truncate());
                        let mut detail = world.visited_at.map(|at| ago(ctx.now, at)).unwrap_or_default();
                        if let Some(spent) = world.time_spent {
                            detail = format!("{detail}  ·  stayed {}", format_duration(spent));
                        }
                        ui.label(RichText::new(detail).size(12.5).color(palette.text_faint));
                    });
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.button("Open").clicked() {
                            ctx.act(Action::OpenUrl(format!("https://vrchat.com/home/world/{}", world.world_id)));
                        }
                    });
                });
            });
        }
    });
}
