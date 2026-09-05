use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::Result;
use eframe::egui;

use crate::combo::{self, ComboTool};
use crate::ulp::{self, UlpTool};
use crate::config::{load_ui_lang, normalize_lmdb_path, save_lmdb_path, save_ui_lang};
use crate::db::{AppendStats, HashDb};
use crate::dump_batch::BatchLiveProgress;
use crate::i18n::{self, Lang};
use crate::job_control::JobControl;
use crate::merger::{self, MergeStats};
use crate::processor::{self, Progress};
use crate::regex_extract::{self, RegexExtractConfig, RegexExtractStats, PRESETS};
use crate::sql_columns::{self, SqlColumnsStats};
use crate::sql_extract::{self, ExtractStats};
use crate::tab_log::TabLog;
use crate::tab_results::{self, TabResults};

/// SG Checker legacy shell — cyber-dark tokens from archive `frontend/legacy/index.vue-legacy.html`
mod harmony {
    use eframe::egui::{self, Color32, Rounding, Stroke, Ui, Visuals};

    // RecehTok crypto dashboard tokens (Free Crypto Dashboard — RecehTok Dark)
    pub const BG: Color32 = Color32::from_rgb(9, 9, 15);
    pub const PANEL: Color32 = Color32::from_rgb(13, 13, 22);
    pub const CARD: Color32 = Color32::from_rgb(20, 20, 32);
    pub const ELEVATED: Color32 = Color32::from_rgb(28, 28, 44);
    pub const INPUT: Color32 = Color32::from_rgb(18, 18, 30);
    pub const BORDER: Color32 = Color32::from_rgb(38, 38, 56);
    pub const BORDER_STRONG: Color32 = Color32::from_rgb(52, 52, 74);
    pub const TEXT: Color32 = Color32::from_rgb(248, 248, 252);
    pub const SECONDARY: Color32 = Color32::from_rgb(152, 152, 176);
    pub const MUTED: Color32 = Color32::from_rgb(88, 88, 112);
    pub const PRIMARY: Color32 = Color32::from_rgb(123, 97, 255);
    pub const PRIMARY_LIGHT: Color32 = Color32::from_rgb(157, 138, 255);
    pub const ACCENT: Color32 = PRIMARY;
    pub const SUCCESS: Color32 = Color32::from_rgb(0, 227, 150);
    pub const SUCCESS_LIGHT: Color32 = Color32::from_rgb(52, 211, 153);
    pub const WARNING: Color32 = Color32::from_rgb(255, 184, 0);
    pub const WARNING_LIGHT: Color32 = Color32::from_rgb(251, 191, 36);
    pub const DANGER: Color32 = Color32::from_rgb(255, 71, 87);
    pub const DANGER_LIGHT: Color32 = Color32::from_rgb(248, 113, 113);

    pub const SKY: Color32 = Color32::from_rgb(0, 212, 255);
    pub const AMBER: Color32 = WARNING_LIGHT;
    pub const ROSE: Color32 = Color32::from_rgb(255, 107, 157);
    pub const ORANGE: Color32 = Color32::from_rgb(255, 140, 66);
    pub const CYAN: Color32 = SKY;
    pub const ACCENT_CYAN: Color32 = SKY;
    pub const VIOLET: Color32 = PRIMARY;
    pub const SLATE: Color32 = Color32::from_rgb(120, 120, 148);
    pub const LOG_BG: Color32 = Color32::from_rgb(6, 6, 10);
    pub const LOG_TEXT: Color32 = Color32::from_rgb(96, 96, 140);

    // Bottom-toolbar crypto-tinted buttons (glow-friendly muted fills)
    pub const TOOLBAR_START_FILL: Color32 = Color32::from_rgb(16, 48, 40);
    pub const TOOLBAR_START_STROKE: Color32 = Color32::from_rgb(0, 180, 120);
    pub const TOOLBAR_START_HOVER: Color32 = Color32::from_rgb(20, 64, 52);
    pub const TOOLBAR_PAUSE_FILL: Color32 = Color32::from_rgb(48, 36, 16);
    pub const TOOLBAR_PAUSE_STROKE: Color32 = Color32::from_rgb(200, 140, 24);
    pub const TOOLBAR_PAUSE_HOVER: Color32 = Color32::from_rgb(64, 48, 20);
    pub const TOOLBAR_RESULTS_FILL: Color32 = Color32::from_rgb(36, 24, 56);
    pub const TOOLBAR_RESULTS_STROKE: Color32 = Color32::from_rgb(100, 72, 180);
    pub const TOOLBAR_RESULTS_HOVER: Color32 = Color32::from_rgb(48, 32, 72);
    pub const TOOLBAR_DELETE_FILL: Color32 = Color32::from_rgb(32, 20, 28);
    pub const TOOLBAR_DELETE_STROKE: Color32 = Color32::from_rgb(120, 48, 64);
    pub const TOOLBAR_DELETE_HOVER: Color32 = Color32::from_rgb(44, 28, 36);
    pub const TOOLBAR_ARCHIVE_FILL: Color32 = Color32::from_rgb(40, 32, 16);
    pub const TOOLBAR_ARCHIVE_STROKE: Color32 = Color32::from_rgb(180, 130, 40);
    pub const TOOLBAR_ARCHIVE_HOVER: Color32 = Color32::from_rgb(56, 44, 20);
    pub const TOOLBAR_NEUTRAL_FILL: Color32 = INPUT;
    pub const TOOLBAR_NEUTRAL_STROKE: Color32 = BORDER;
    pub const TOOLBAR_NEUTRAL_HOVER: Color32 = ELEVATED;

    pub const BTN_ROUND: f32 = 8.0;
    pub const CARD_ROUND: f32 = 12.0;
    pub const CARD_ROUND_LG: f32 = 16.0;
    pub const ROUND: f32 = BTN_ROUND;
    pub const GAP: f32 = 8.0;
    pub const GRID_GAP: f32 = 16.0;
    pub const FONT: f32 = 13.0;
    pub const FONT_SMALL: f32 = 12.0;
    pub const FONT_TINY: f32 = 11.0;
    pub const FONT_LABEL: f32 = 9.0;
    pub const FONT_PAGE: f32 = 22.0;
    pub const BROWSE_W: f32 = 88.0;
    pub const ACTION_W: f32 = 96.0;
    pub const TOOLBAR_W: f32 = 74.0;
    pub const CTRL_H: f32 = 32.0;
    pub const CONTENT_HEADER_H: f32 = 72.0;
    pub const SIDEBAR_W: f32 = 252.0;
    pub const NAV_ITEM_H: f32 = 44.0;
    pub const LOG_CARD_H: f32 = 108.0;
    pub const STAT_TILE_H: f32 = 72.0;
    pub const UI_ZOOM: f32 = crate::config::UI_ZOOM_FIXED;
    pub const WINDOW_W: f32 = 1200.0 * UI_ZOOM;
    pub const WINDOW_H: f32 = 760.0 * UI_ZOOM;
    pub const WINDOW_SHELL_ROUND: f32 = 16.0;
    pub const WIN_CTRL_SIZE: f32 = 28.0;

    fn stroke_border() -> Stroke {
        Stroke::new(1.0_f32, BORDER)
    }

    pub fn apply(ctx: &egui::Context) {
        let mut visuals = Visuals::dark();
        visuals.panel_fill = PANEL;
        visuals.window_fill = BG;
        visuals.extreme_bg_color = BG;
        visuals.faint_bg_color = PANEL;
        visuals.code_bg_color = LOG_BG;
        visuals.window_stroke = stroke_border();
        visuals.widgets.noninteractive.bg_fill = CARD;
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, TEXT);
        visuals.widgets.inactive.bg_fill = INPUT;
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, TEXT);
        visuals.widgets.inactive.bg_stroke = stroke_border();
        visuals.widgets.hovered.bg_fill = ELEVATED;
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, TEXT);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, BORDER_STRONG);
        visuals.widgets.active.bg_fill = ELEVATED;
        visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, TEXT);
        visuals.widgets.open.bg_fill = ELEVATED;
        visuals.selection.bg_fill = ELEVATED.gamma_multiply(0.85);
        visuals.selection.stroke = Stroke::new(1.0_f32, BORDER_STRONG);
        visuals.hyperlink_color = SECONDARY;
        visuals.warn_fg_color = WARNING;
        visuals.error_fg_color = DANGER;
        visuals.window_rounding = Rounding::same(CARD_ROUND);
        ctx.set_visuals(visuals);
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(GAP, GAP);
        style.spacing.button_padding = egui::vec2(10.0, 4.0);
        style.spacing.interact_size = egui::vec2(36.0, CTRL_H);
        ctx.set_style(style);
    }

    fn card_stroke() -> Stroke {
        stroke_border()
    }

    fn nav_active_fill() -> Color32 {
        Color32::from_rgba_unmultiplied(123, 97, 255, 31)
    }

    fn nav_active_stroke() -> Stroke {
        Stroke::new(1.0_f32, PRIMARY.gamma_multiply(0.35))
    }

    pub fn section_frame() -> egui::Frame {
        egui::Frame::none()
            .fill(CARD)
            .stroke(Stroke::new(1.0_f32, BORDER))
            .rounding(Rounding::same(CARD_ROUND))
            .inner_margin(egui::vec2(14.0, 10.0))
    }

    pub fn header_frame() -> egui::Frame {
        egui::Frame::none()
            .fill(PANEL)
            .inner_margin(egui::vec2(16.0, 10.0))
            .outer_margin(egui::Margin::symmetric(0.0, 0.0))
    }

    pub fn sidebar_frame() -> egui::Frame {
        egui::Frame::none()
            .fill(PANEL)
            .inner_margin(egui::vec2(16.0, 20.0))
    }

    pub fn content_header_frame() -> egui::Frame {
        egui::Frame::none().fill(BG)
    }

    pub fn action_bar_frame() -> egui::Frame {
        egui::Frame::none().fill(BG)
    }

    pub fn page_title(ui: &mut Ui, title: &str, subtitle: &str) {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.label(
                egui::RichText::new(title)
                    .size(FONT_PAGE)
                    .strong()
                    .color(TEXT),
            );
            ui.label(
                egui::RichText::new(subtitle)
                    .size(FONT_SMALL)
                    .color(MUTED),
            );
        });
    }

    pub fn log_frame() -> egui::Frame {
        egui::Frame::none()
            .fill(LOG_BG)
            .stroke(Stroke::new(1.0_f32, BORDER))
            .rounding(Rounding::same(CARD_ROUND_LG))
            .inner_margin(egui::vec2(12.0, 10.0))
    }

    pub fn section_title(ui: &mut Ui, text: &str) {
        ui.label(
            egui::RichText::new(text.to_uppercase())
                .size(FONT_LABEL)
                .strong()
                .color(SECONDARY),
        );
    }

    pub fn stat_tile(ui: &mut Ui, label: &str, value: &str, value_color: Color32) {
        let (border, fill) = if value_color == SUCCESS {
            (SUCCESS.gamma_multiply(0.45), SUCCESS.gamma_multiply(0.12))
        } else if value_color == DANGER {
            (DANGER.gamma_multiply(0.45), DANGER.gamma_multiply(0.12))
        } else if value_color == WARNING {
            (WARNING.gamma_multiply(0.45), WARNING.gamma_multiply(0.12))
        } else {
            (BORDER.gamma_multiply(0.85), CARD)
        };
        egui::Frame::none()
            .fill(fill)
            .stroke(Stroke::new(1.0_f32, border))
            .rounding(Rounding::same(CARD_ROUND))
            .inner_margin(egui::vec2(6.0, 4.0))
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new(label.to_uppercase())
                            .size(9.0)
                            .color(MUTED)
                            .strong(),
                    );
                    ui.label(
                        egui::RichText::new(value)
                            .size(FONT_SMALL)
                            .strong()
                            .monospace()
                            .color(value_color),
                    );
                });
            });
    }

    /// Horizontal sidebar nav (legacy `.mode` / CheckerShell `NavItem`).
    pub fn sidebar_nav(
        ui: &mut Ui,
        icon: egui::TextureId,
        label: &str,
        active: bool,
    ) -> egui::Response {
        let w = ui.available_width();
        let h = NAV_ITEM_H;
        let (rect, response) = ui.allocate_exact_size(egui::vec2(w, h), egui::Sense::click());
        if ui.is_rect_visible(rect) {
            let fill = if active {
                nav_active_fill()
            } else if response.hovered() {
                ELEVATED.gamma_multiply(0.65)
            } else {
                Color32::TRANSPARENT
            };
            ui.painter()
                .rect_filled(rect, Rounding::same(CARD_ROUND), fill);
            if active {
                let bar = egui::Rect::from_min_max(
                    egui::pos2(rect.left(), rect.top() + 6.0),
                    egui::pos2(rect.left() + 3.0, rect.bottom() - 6.0),
                );
                ui.painter().rect_filled(bar, Rounding::same(2.0), PRIMARY);
            }
            let text_color = if active { TEXT } else { SECONDARY };
            let icon_tint = if active {
                SECONDARY
            } else {
                MUTED.gamma_multiply(0.85)
            };
            let icon_size = 16.0;
            let icon_rect = egui::Rect::from_center_size(
                egui::pos2(rect.left() + 22.0, rect.center().y),
                egui::vec2(icon_size, icon_size),
            );
            ui.painter().image(
                icon,
                icon_rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                icon_tint,
            );
            ui.painter().text(
                egui::pos2(rect.left() + 38.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                label,
                egui::FontId::proportional(if active { FONT_SMALL } else { FONT_SMALL }),
                if active { TEXT } else { text_color },
            );
        }
        response.on_hover_text(label)
    }

    pub fn tab_chip(label: &str, active: bool) -> egui::Button<'_> {
        let (fill, stroke, text_color) = if active {
            (nav_active_fill(), nav_active_stroke(), TEXT)
        } else {
            (INPUT, stroke_border(), MUTED)
        };
        egui::Button::new(
            egui::RichText::new(label)
                .size(FONT_TINY)
                .strong()
                .color(text_color),
        )
        .fill(fill)
        .stroke(stroke)
        .rounding(Rounding::same(BTN_ROUND))
        .min_size(egui::vec2(0.0, CTRL_H))
    }

    pub fn sidebar_section_title(ui: &mut Ui, text: &str) {
        ui.label(
            egui::RichText::new(text.to_uppercase())
                .size(FONT_LABEL)
                .strong()
                .color(MUTED),
        );
        ui.add_space(2.0);
    }

    pub fn heading(ui: &mut Ui, text: &str) {
        ui.label(egui::RichText::new(text).size(FONT).strong().color(TEXT));
    }

    pub fn body(ui: &mut Ui, text: &str, color: Color32) {
        fill_width(ui);
        ui.add(
            egui::Label::new(egui::RichText::new(text).size(FONT).color(color)).wrap(),
        );
    }

    pub fn muted(ui: &mut Ui, text: &str) {
        fill_width(ui);
        ui.add(
            egui::Label::new(egui::RichText::new(text).size(FONT_SMALL).color(MUTED)).wrap(),
        );
    }

    /// Primary CTA — same muted shell as Browse buttons.
    pub fn primary_button(text: &str) -> egui::Button<'_> {
        egui::Button::new(egui::RichText::new(text).size(FONT_SMALL).strong().color(SECONDARY))
            .fill(CARD)
            .stroke(Stroke::new(1.0_f32, BORDER))
            .rounding(Rounding::same(BTN_ROUND))
            .min_size(egui::vec2(ACTION_W, CTRL_H))
    }

    pub fn secondary_button(text: &str) -> egui::Button<'_> {
        egui::Button::new(egui::RichText::new(text).size(FONT_SMALL).color(SECONDARY))
            .fill(CARD)
            .stroke(Stroke::new(1.0_f32, BORDER))
            .rounding(Rounding::same(BTN_ROUND))
            .min_size(egui::vec2(BROWSE_W, CTRL_H))
    }

    /// Compact bottom-toolbar button (fits fixed 110% window width).
    pub fn toolbar_button(text: &str) -> egui::Button<'_> {
        egui::Button::new(egui::RichText::new(text).size(FONT_TINY).color(SECONDARY))
            .fill(CARD)
            .stroke(Stroke::new(1.0_f32, BORDER))
            .rounding(Rounding::same(BTN_ROUND))
            .min_size(egui::vec2(TOOLBAR_W, CTRL_H - 2.0))
    }

    fn toolbar_tinted(
        ui: &mut Ui,
        text: &str,
        fill: Color32,
        stroke: Color32,
        hover_fill: Color32,
        strong: bool,
    ) -> egui::Response {
        let size = egui::vec2(TOOLBAR_W, CTRL_H - 2.0);
        let (rect, response) = ui.allocate_at_least(size, egui::Sense::click());
        if ui.is_rect_visible(rect) {
            let enabled = ui.is_enabled();
            let hovered = response.hovered() && enabled;
            let active_fill = if !enabled {
                fill.gamma_multiply(0.45)
            } else if hovered {
                hover_fill
            } else {
                fill
            };
            let active_stroke = if !enabled {
                stroke.gamma_multiply(0.45)
            } else if hovered {
                hover_fill.gamma_multiply(1.15)
            } else {
                stroke
            };
            let text_color = if !enabled {
                MUTED
            } else if hovered {
                TEXT
            } else {
                SECONDARY
            };
            ui.painter()
                .rect_filled(rect, Rounding::same(BTN_ROUND), active_fill);
            ui.painter().rect_stroke(
                rect,
                Rounding::same(BTN_ROUND),
                Stroke::new(1.0_f32, active_stroke),
            );
            let font = egui::FontId::proportional(FONT_TINY);
            let galley = if strong {
                ui.painter().layout(
                    text.to_owned(),
                    font,
                    text_color,
                    f32::INFINITY,
                )
            } else {
                ui.painter().layout(
                    text.to_owned(),
                    font,
                    text_color,
                    f32::INFINITY,
                )
            };
            let pos = rect.center() - galley.size() / 2.0;
            ui.painter().galley(pos, galley, text_color);
        }
        response
    }

    pub fn toolbar_start(ui: &mut Ui, text: &str) -> egui::Response {
        toolbar_tinted(
            ui,
            text,
            TOOLBAR_START_FILL,
            TOOLBAR_START_STROKE,
            TOOLBAR_START_HOVER,
            true,
        )
    }

    pub fn toolbar_pause(ui: &mut Ui, text: &str) -> egui::Response {
        toolbar_tinted(
            ui,
            text,
            TOOLBAR_PAUSE_FILL,
            TOOLBAR_PAUSE_STROKE,
            TOOLBAR_PAUSE_HOVER,
            false,
        )
    }

    pub fn toolbar_stop(ui: &mut Ui, text: &str) -> egui::Response {
        let size = egui::vec2(TOOLBAR_W, CTRL_H - 2.0);
        let (rect, response) = ui.allocate_at_least(size, egui::Sense::click());
        if ui.is_rect_visible(rect) {
            let enabled = ui.is_enabled();
            let hovered = response.hovered() && enabled;
            let active_fill = if !enabled {
                DANGER.gamma_multiply(0.45)
            } else if hovered {
                DANGER.gamma_multiply(1.12)
            } else {
                DANGER
            };
            let active_stroke = if !enabled {
                DANGER.gamma_multiply(0.45)
            } else if hovered {
                DANGER_LIGHT
            } else {
                DANGER
            };
            let text_color = if enabled { TEXT } else { MUTED };
            ui.painter()
                .rect_filled(rect, Rounding::same(BTN_ROUND), active_fill);
            ui.painter().rect_stroke(
                rect,
                Rounding::same(BTN_ROUND),
                Stroke::new(1.0_f32, active_stroke),
            );
            let galley = ui.painter().layout(
                text.to_owned(),
                egui::FontId::proportional(FONT_TINY),
                text_color,
                f32::INFINITY,
            );
            let pos = rect.center() - galley.size() / 2.0;
            ui.painter().galley(pos, galley, text_color);
        }
        response
    }

    pub fn toolbar_results(ui: &mut Ui, text: &str) -> egui::Response {
        toolbar_tinted(
            ui,
            text,
            TOOLBAR_RESULTS_FILL,
            TOOLBAR_RESULTS_STROKE,
            TOOLBAR_RESULTS_HOVER,
            false,
        )
    }

    pub fn toolbar_delete(ui: &mut Ui, text: &str) -> egui::Response {
        toolbar_tinted(
            ui,
            text,
            TOOLBAR_DELETE_FILL,
            TOOLBAR_DELETE_STROKE,
            TOOLBAR_DELETE_HOVER,
            false,
        )
    }

    pub fn toolbar_archive(ui: &mut Ui, text: &str) -> egui::Response {
        toolbar_tinted(
            ui,
            text,
            TOOLBAR_ARCHIVE_FILL,
            TOOLBAR_ARCHIVE_STROKE,
            TOOLBAR_ARCHIVE_HOVER,
            false,
        )
    }

    pub fn toolbar_neutral(ui: &mut Ui, text: &str) -> egui::Response {
        toolbar_tinted(
            ui,
            text,
            TOOLBAR_NEUTRAL_FILL,
            TOOLBAR_NEUTRAL_STROKE,
            TOOLBAR_NEUTRAL_HOVER,
            false,
        )
    }

    pub fn toolbar_primary(text: &str) -> egui::Button<'_> {
        egui::Button::new(egui::RichText::new(text).size(FONT_TINY).strong().color(SECONDARY))
            .fill(CARD)
            .stroke(Stroke::new(1.0_f32, BORDER))
            .rounding(Rounding::same(BTN_ROUND))
            .min_size(egui::vec2(TOOLBAR_W, CTRL_H - 2.0))
    }

    pub fn toolbar_danger(text: &str) -> egui::Button<'_> {
        egui::Button::new(egui::RichText::new(text).size(FONT_TINY).strong().color(TEXT))
            .fill(DANGER)
            .stroke(Stroke::new(1.0_f32, DANGER))
            .rounding(Rounding::same(BTN_ROUND))
            .min_size(egui::vec2(TOOLBAR_W, CTRL_H - 2.0))
    }

    /// Red stop CTA (legacy `.runbtn.stop`).
    pub fn danger_button(text: &str) -> egui::Button<'_> {
        egui::Button::new(egui::RichText::new(text).size(FONT_SMALL).strong().color(TEXT))
            .fill(DANGER)
            .stroke(Stroke::new(1.0_f32, DANGER))
            .rounding(Rounding::same(BTN_ROUND))
            .min_size(egui::vec2(ACTION_W, CTRL_H))
    }

    /// Single-line path/input field at fixed control height, fills remaining width.
    pub fn path_edit<'a>(value: &'a mut String, hint: &'a str, width: f32) -> egui::TextEdit<'a> {
        egui::TextEdit::singleline(value)
            .desired_width(width.max(80.0))
            .hint_text(hint)
            .margin(egui::vec2(8.0, 6.0))
            .min_size(egui::vec2(80.0, CTRL_H))
    }

    /// `[stretching field] [Browse]` — returns true if browse clicked.
    pub fn path_browse_row(ui: &mut Ui, value: &mut String, hint: &str, browse: &str) -> bool {
        fill_width(ui);
        let row_w = ui.available_width();
        let mut clicked = false;
        ui.horizontal(|ui| {
            ui.set_max_width(row_w);
            ui.spacing_mut().item_spacing.x = GAP;
            let field_w = (ui.available_width() - BROWSE_W - GAP).max(80.0);
            ui.add(path_edit(value, hint, field_w));
            clicked = ui.add(secondary_button(browse)).clicked();
        });
        clicked
    }

    /// `[stretching field] [Browse] [Action]` — (browse_clicked, action_clicked).
    pub fn path_browse_action_row(
        ui: &mut Ui,
        value: &mut String,
        hint: &str,
        browse: &str,
        action: &str,
        action_enabled: bool,
    ) -> (bool, bool) {
        fill_width(ui);
        let row_w = ui.available_width();
        let mut browse_clicked = false;
        let mut act = false;
        ui.horizontal(|ui| {
            ui.set_max_width(row_w);
            ui.spacing_mut().item_spacing.x = GAP;
            let field_w = (ui.available_width() - BROWSE_W - ACTION_W - GAP * 2.0).max(80.0);
            ui.add(path_edit(value, hint, field_w));
            browse_clicked = ui.add(secondary_button(browse)).clicked();
            act = ui
                .add_enabled(action_enabled, primary_button(action))
                .clicked();
        });
        (browse_clicked, act)
    }

    /// Full-width field, then a row of equal browse-sized buttons.
    pub fn path_file_folder_row(
        ui: &mut Ui,
        value: &mut String,
        hint: &str,
        file_label: &str,
        folder_label: &str,
        folder_hint: &str,
    ) -> (bool, bool) {
        fill_width(ui);
        ui.add(path_edit(value, hint, ui.available_width()));
        ui.add_space(4.0);
        let mut file = false;
        let mut folder = false;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = GAP;
            file = ui.add(secondary_button(file_label)).clicked();
            folder = ui.add(secondary_button(folder_label)).clicked();
            muted(ui, folder_hint);
        });
        (file, folder)
    }

    pub fn sidebar_tab(label: &str, glyph: &str, color: Color32, active: bool) -> egui::Button<'static> {
        let fill = if active { ELEVATED } else { Color32::TRANSPARENT };
        let text_color = if active { TEXT } else { MUTED };
        let glyph_color = if active { color } else { color.gamma_multiply(0.75) };
        let stroke = if active {
            nav_active_stroke()
        } else {
            Stroke::new(1.0_f32, Color32::TRANSPARENT)
        };
        let mut job = egui::text::LayoutJob::default();
        job.append(
            glyph,
            0.0,
            egui::TextFormat {
                font_id: egui::FontId::proportional(FONT),
                color: glyph_color,
                ..Default::default()
            },
        );
        job.append(
            &format!("  {label}"),
            0.0,
            egui::TextFormat {
                font_id: egui::FontId::proportional(FONT),
                color: text_color,
                ..Default::default()
            },
        );
        egui::Button::new(job)
            .fill(fill)
            .stroke(stroke)
            .rounding(Rounding::same(ROUND))
            .min_size(egui::vec2(f32::INFINITY, CTRL_H))
    }

    pub fn sidebar_primary(text: &str) -> egui::Button<'_> {
        egui::Button::new(egui::RichText::new(text).size(FONT).strong().color(SECONDARY))
            .fill(CARD)
            .stroke(Stroke::new(1.0_f32, BORDER))
            .rounding(Rounding::same(ROUND))
            .min_size(egui::vec2(f32::INFINITY, CTRL_H))
    }

    pub fn fill_width(ui: &mut Ui) {
        ui.set_width(ui.available_width());
    }

    pub fn instruction_body(ui: &mut Ui, text: &str) {
        ui.label(
            egui::RichText::new(text)
                .size(FONT_TINY)
                .color(SECONDARY),
        );
    }

    pub fn instruction_heading(ui: &mut Ui, text: &str) {
        ui.label(
            egui::RichText::new(text)
                .size(FONT_TINY)
                .strong()
                .color(TEXT),
        );
    }

    pub fn instruction_mono(ui: &mut Ui, text: &str) {
        ui.label(
            egui::RichText::new(text)
                .size(11.0)
                .color(MUTED)
                .family(egui::FontFamily::Monospace),
        );
    }

    pub fn instruction_button(ui: &mut Ui, open: &mut bool, label: &str) {
        ui.horizontal(|ui| {
            if ui.add(secondary_button(label)).clicked() {
                *open = true;
            }
        });
    }

    pub fn instruction_modal(
        ui: &mut Ui,
        open: &mut bool,
        title: &str,
        close_label: &str,
        content: impl FnOnce(&mut Ui),
    ) {
        if !*open {
            return;
        }
        let ctx = ui.ctx().clone();
        let mut keep_open = true;
        let close_clicked = std::cell::Cell::new(false);
        egui::Window::new(title)
            .open(&mut keep_open)
            .collapsible(false)
            .resizable(true)
            .default_width(520.0)
            .max_width(640.0)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .frame(
                egui::Frame::window(&ctx.style())
                    .fill(CARD)
                    .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
                    .inner_margin(egui::vec2(16.0, 12.0)),
            )
            .show(&ctx, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(420.0)
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.set_max_width(580.0);
                        content(ui);
                    });
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add(primary_button(close_label)).clicked() {
                            close_clicked.set(true);
                        }
                    });
                });
            });
        *open = keep_open && !close_clicked.get();
    }

    fn paint_brand_logo(ui: &mut Ui) -> egui::Rect {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(26.0, 26.0), egui::Sense::hover());
        let r = 7.0;
        ui.painter().rect_filled(rect, Rounding::same(r), CARD);
        ui.painter().rect_stroke(rect, Rounding::same(r), Stroke::new(1.0_f32, BORDER_STRONG));
        ui.painter().rect_filled(
            rect.shrink(2.0),
            Rounding::same(r - 1.0),
            ELEVATED,
        );
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "LH",
            egui::FontId::proportional(9.0),
            TEXT,
        );
        rect
    }

    pub fn sidebar_brand(ui: &mut Ui, subtitle: &str) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 10.0;
            paint_brand_logo(ui);
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 1.0;
                ui.label(
                    egui::RichText::new("Local Hash Finder")
                        .size(13.0)
                        .strong()
                        .color(TEXT),
                );
                ui.label(
                    egui::RichText::new(subtitle)
                        .size(FONT_TINY)
                        .color(MUTED),
                );
            });
        });
    }

    pub fn brand_block(ui: &mut Ui, subtitle: &str) {
        sidebar_brand(ui, subtitle);
    }

    pub fn vsep(ui: &mut Ui) {
        let h = 18.0;
        let (rect, _) = ui.allocate_exact_size(egui::vec2(1.0, h), egui::Sense::hover());
        ui.painter().rect_filled(rect, Rounding::ZERO, BORDER);
    }

    pub fn paint_window_shell(ctx: &egui::Context) {
        let rect = ctx.screen_rect();
        let painter = ctx.layer_painter(egui::LayerId::background());
        painter.rect_filled(rect, Rounding::same(WINDOW_SHELL_ROUND), BG);
        painter.rect_stroke(
            rect,
            Rounding::same(WINDOW_SHELL_ROUND),
            Stroke::new(1.0_f32, BORDER),
        );
    }

    pub fn window_drag_region(ctx: &egui::Context, ui: &mut Ui, rect: egui::Rect, id_suffix: &str) {
        let id = ui.id().with(id_suffix);
        let response = ui.interact(rect, id, egui::Sense::click_and_drag());
        if response.dragged() {
            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }
    }

    fn window_chrome_button(ui: &mut Ui, glyph: &str, danger: bool) -> egui::Response {
        let size = egui::vec2(WIN_CTRL_SIZE, WIN_CTRL_SIZE);
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
        if ui.is_rect_visible(rect) {
            let hovered = response.hovered();
            let fill = if danger && hovered {
                DANGER.gamma_multiply(0.85)
            } else if hovered {
                ELEVATED
            } else {
                INPUT.gamma_multiply(0.35)
            };
            let stroke_col = if danger && hovered {
                DANGER
            } else if hovered {
                BORDER_STRONG
            } else {
                BORDER.gamma_multiply(0.75)
            };
            ui.painter()
                .rect_filled(rect, Rounding::same(BTN_ROUND), fill);
            ui.painter().rect_stroke(
                rect,
                Rounding::same(BTN_ROUND),
                Stroke::new(1.0_f32, stroke_col),
            );
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                glyph,
                egui::FontId::proportional(if danger { 14.0 } else { 12.0 }),
                if danger && hovered {
                    TEXT
                } else {
                    SECONDARY
                },
            );
        }
        response
    }

    pub fn window_controls(ctx: &egui::Context, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            if window_chrome_button(ui, "—", false).clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
            }
            if window_chrome_button(ui, "×", true).clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }

    pub fn db_pill(ui: &mut Ui, status: &str, path: &str, path_tip: &str) {
        let frame = egui::Frame::none()
            .fill(ELEVATED)
            .stroke(Stroke::new(1.0_f32, BORDER))
            .rounding(Rounding::same(CARD_ROUND))
            .inner_margin(egui::vec2(8.0, 4.0));
        let response = frame.show(ui, |ui| {
            ui.set_max_width(240.0);
            ui.horizontal(|ui| {
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(8.0, 8.0), egui::Sense::hover());
                ui.painter().circle_filled(rect.center(), 3.0, SUCCESS_LIGHT);
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(status).size(FONT_TINY).color(SECONDARY),
                    )
                    .truncate(),
                );
            });
        });
        response.response.on_hover_text(format!("{path_tip}\n{path}"));
    }

    pub fn hash_db_path_row(ui: &mut Ui, label: &str, path: &str, tip: &str) {
        fill_width(ui);
        ui.label(
            egui::RichText::new(label)
                .size(FONT_SMALL)
                .color(SECONDARY)
                .strong(),
        );
        let path_label = ui.add(
            egui::Label::new(
                egui::RichText::new(path)
                    .size(FONT_SMALL)
                    .monospace()
                    .color(MUTED),
            )
            .selectable(true)
            .wrap(),
        );
        path_label.on_hover_text(tip);
    }

    pub fn sidebar_action(text: &str) -> egui::Button<'_> {
        egui::Button::new(egui::RichText::new(text).size(FONT).color(SECONDARY))
            .fill(INPUT)
            .stroke(stroke_border())
            .rounding(Rounding::same(ROUND))
            .min_size(egui::vec2(f32::INFINITY, CTRL_H))
    }

    pub fn sidebar_danger(text: &str) -> egui::Button<'_> {
        egui::Button::new(egui::RichText::new(text).size(FONT).color(TEXT))
            .fill(DANGER)
            .rounding(Rounding::same(ROUND))
            .min_size(egui::vec2(f32::INFINITY, CTRL_H))
    }

    pub fn status_pill(ui: &mut Ui, text: &str, color: Color32) {
        egui::Frame::none()
            .fill(color.gamma_multiply(0.22))
            .stroke(Stroke::new(1.0_f32, color.gamma_multiply(0.45)))
            .rounding(Rounding::same(100.0))
            .inner_margin(egui::vec2(8.0, 3.0))
            .show(ui, |ui| {
                ui.label(egui::RichText::new(text).size(FONT_TINY).color(color));
            });
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Lookup,
    Merge,
    ExtractSql,
    CustomRegex,
    SqlColumns,
    Combo,
    Ulp,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TabJob {
    Lookup,
    Merge,
    Sql,
    SqlColumns,
    Regex,
    Combo,
    Ulp,
}


fn window_icon() -> Option<egui::IconData> {
    eframe::icon_data::from_png_bytes(include_bytes!("../assets/app-icon.png")).ok()
}

#[cfg(windows)]
fn apply_native_window_shell(handle: &dyn raw_window_handle::HasWindowHandle) {
    use raw_window_handle::{HasWindowHandle as _, RawWindowHandle};
    use windows_sys::Win32::Foundation::HWND;
    use windows_sys::Win32::Graphics::Dwm::{
        DwmSetWindowAttribute, DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_ROUND,
    };

    let Ok(window_handle) = handle.window_handle() else {
        return;
    };
    let RawWindowHandle::Win32(win32) = window_handle.as_raw() else {
        return;
    };
    let hwnd = win32.hwnd.get() as HWND;
    let round = DWMWCP_ROUND as u32;
    unsafe {
        DwmSetWindowAttribute(
            hwnd,
            DWMWA_WINDOW_CORNER_PREFERENCE as u32,
            &round as *const _ as *const _,
            std::mem::size_of::<u32>() as u32,
        );
    }
}

#[cfg(not(windows))]
fn apply_native_window_shell(_handle: &dyn raw_window_handle::HasWindowHandle) {}

fn sidebar_link_color_image() -> Option<egui::ColorImage> {
    png_color_image(include_bytes!("../assets/sidebar-link.png"))
}

const LEAKBASE_FORUM_URL: &str = "https://leakbase.su";

fn leakbase_logo_color_image() -> Option<egui::ColorImage> {
    png_color_image(include_bytes!("../assets/leakbase-logo.png"))
}

fn png_color_image(bytes: &[u8]) -> Option<egui::ColorImage> {
    let icon = eframe::icon_data::from_png_bytes(bytes).ok()?;
    let w = icon.width as usize;
    let h = icon.height as usize;
    let mut rgba = icon.rgba;
    for pixel in rgba.chunks_exact_mut(4) {
        if pixel[0] < 40 && pixel[1] < 40 && pixel[2] < 40 {
            pixel[3] = 0;
        }
    }
    Some(egui::ColorImage::from_rgba_unmultiplied([w, h], &rgba))
}

#[cfg(windows)]
fn open_external_url(url: &str) {
    let _ = std::process::Command::new("cmd")
        .args(["/C", "start", "", url])
        .spawn();
}

#[cfg(not(windows))]
fn open_external_url(url: &str) {
    let _ = url;
}

#[cfg(windows)]
fn open_in_explorer(path: &Path) {
    if path.is_file() {
        let _ = std::process::Command::new("explorer")
            .arg(format!("/select,{}", path.display()))
            .spawn();
    } else if path.is_dir() {
        let _ = std::process::Command::new("explorer").arg(path).spawn();
    }
}

#[cfg(not(windows))]
fn open_in_explorer(path: &Path) {
    let _ = path;
}

pub struct App {
    db: Arc<HashDb>,
    tab: Tab,
    input_path: String,
    threads: u32,
    running: bool,
    lookup_control: Arc<JobControl>,
    progress: Arc<Mutex<Progress>>,
    status: String,
    error: Option<String>,
    lookup_log: Arc<TabLog>,
    lookup_results: Arc<TabResults>,
    merge_control: Arc<JobControl>,
    merge_log: Arc<TabLog>,
    merge_results: Arc<TabResults>,
    merge_mail_path: String,
    merge_dehash_path: String,
    merge_running: bool,
    merge_status: String,
    merge_error: Option<String>,
    merge_result: Arc<Mutex<Option<MergeStats>>>,
    sql_path: String,
    sql_threads: u32,
    sql_running: bool,
    sql_status: String,
    sql_error: Option<String>,
    sql_result: Arc<Mutex<Option<ExtractStats>>>,
    sql_live: Arc<Mutex<BatchLiveProgress>>,
    sql_batch: bool,
    sql_control: Arc<JobControl>,
    sql_log: Arc<TabLog>,
    sql_results: Arc<TabResults>,
    regex_source_path: String,
    regex_pattern: String,
    regex_template: String,
    regex_case_insensitive: bool,
    regex_multiline: bool,
    regex_dotall: bool,
    regex_dedupe: bool,
    regex_preset_idx: usize,
    regex_running: bool,
    regex_status: String,
    regex_error: Option<String>,
    regex_result: Arc<Mutex<Option<RegexExtractStats>>>,
    regex_control: Arc<JobControl>,
    regex_log: Arc<TabLog>,
    regex_results: Arc<TabResults>,
    sqlcol_path: String,
    sqlcol_running: bool,
    sqlcol_status: String,
    sqlcol_error: Option<String>,
    sqlcol_result: Arc<Mutex<Option<SqlColumnsStats>>>,
    sqlcol_live: Arc<Mutex<BatchLiveProgress>>,
    sqlcol_batch: bool,
    sqlcol_control: Arc<JobControl>,
    sqlcol_log: Arc<TabLog>,
    sqlcol_results: Arc<TabResults>,
    combo_tool: ComboTool,
    combo_input: String,
    combo_input_b: String,
    combo_output: String,
    combo_output_dir: String,
    combo_filter: String,
    combo_use_regex: bool,
    combo_lines_per_file: usize,
    combo_running: bool,
    combo_status: String,
    combo_error: Option<String>,
    combo_control: Arc<JobControl>,
    combo_log: Arc<TabLog>,
    combo_results: Arc<TabResults>,
    combo_result: Arc<Mutex<Option<Result<combo::ComboJobSummary, String>>>>,
    ulp_tool: UlpTool,
    ulp_input: String,
    ulp_output: String,
    ulp_output_dir: String,
    ulp_keywords: String,
    ulp_running: bool,
    ulp_status: String,
    ulp_error: Option<String>,
    ulp_control: Arc<JobControl>,
    ulp_log: Arc<TabLog>,
    ulp_results: Arc<TabResults>,
    ulp_result: Arc<Mutex<Option<Result<ulp::UlpJobSummary, String>>>>,
    append_path: String,
    append_running: bool,
    append_status: String,
    append_error: Option<String>,
    append_result: Arc<Mutex<Option<Result<AppendStats, String>>>>,
    lmdb_path_input: String,
    lmdb_apply_error: Option<String>,
    db_status: String,
    lang: Lang,
    instruction_open: bool,
    sidebar_tab_icon: Option<egui::TextureHandle>,
    leakbase_logo_texture: Option<egui::TextureHandle>,
}

impl App {
    fn new(
        db: Arc<HashDb>,
            ) -> Self {
        let _ = db.open_existing();
        let count = db.count();
        let lmdb = db.lmdb_path().display().to_string();
        let lmdb_ready = db.lmdb_path().is_dir();
        let lang = load_ui_lang();
        let app = Self {
            db,
            tab: Tab::Lookup,
            input_path: String::new(),
            threads: 32,
            running: false,
            lookup_control: JobControl::new_shared(),
            progress: Arc::new(Mutex::new(Progress::default())),
            lookup_log: TabLog::shared(),
            lookup_results: TabResults::shared(),
            status: String::new(),
            db_status: if count > 0 {
                i18n::db_status_entries(lang, count)
            } else if lmdb_ready {
                i18n::tr(lang).db_path_saved_empty.into()
            } else {
                i18n::tr(lang).db_pick_lmdb.into()
            },
            error: None,
            merge_mail_path: String::new(),
            merge_dehash_path: String::new(),
            merge_running: false,
            merge_status: String::new(),
            merge_error: None,
            merge_result: Arc::new(Mutex::new(None)),
            merge_control: JobControl::new_shared(),
            merge_log: TabLog::shared(),
            merge_results: TabResults::shared(),
            sql_path: String::new(),
            sql_threads: 1,
            sql_running: false,
            sql_status: String::new(),
            sql_error: None,
            sql_result: Arc::new(Mutex::new(None)),
            sql_live: Arc::new(Mutex::new(BatchLiveProgress::default())),
            sql_batch: false,
            sql_control: JobControl::new_shared(),
            sql_log: TabLog::shared(),
            sql_results: TabResults::shared(),
            regex_source_path: String::new(),
            regex_pattern: PRESETS[0].pattern.into(),
            regex_template: PRESETS[0].template.into(),
            regex_case_insensitive: PRESETS[0].case_insensitive,
            regex_multiline: false,
            regex_dotall: false,
            regex_dedupe: true,
            regex_preset_idx: 0,
            regex_running: false,
            regex_status: String::new(),
            regex_error: None,
            regex_result: Arc::new(Mutex::new(None)),
            regex_control: JobControl::new_shared(),
            regex_log: TabLog::shared(),
            regex_results: TabResults::shared(),
            sqlcol_path: String::new(),
            sqlcol_running: false,
            sqlcol_status: String::new(),
            sqlcol_error: None,
            sqlcol_result: Arc::new(Mutex::new(None)),
            sqlcol_live: Arc::new(Mutex::new(BatchLiveProgress::default())),
            sqlcol_batch: false,
            sqlcol_control: JobControl::new_shared(),
            sqlcol_log: TabLog::shared(),
            sqlcol_results: TabResults::shared(),
            combo_tool: ComboTool::Compare,
            combo_input: String::new(),
            combo_input_b: String::new(),
            combo_output: String::new(),
            combo_output_dir: String::new(),
            combo_filter: String::new(),
            combo_use_regex: false,
            combo_lines_per_file: 100_000,
            combo_running: false,
            combo_status: String::new(),
            combo_error: None,
            combo_control: JobControl::new_shared(),
            combo_log: TabLog::shared(),
            combo_results: TabResults::shared(),
            combo_result: Arc::new(Mutex::new(None)),
            ulp_tool: UlpTool::Sort,
            ulp_input: String::new(),
            ulp_output: String::new(),
            ulp_output_dir: String::new(),
            ulp_keywords: String::new(),
            ulp_running: false,
            ulp_status: String::new(),
            ulp_error: None,
            ulp_control: JobControl::new_shared(),
            ulp_log: TabLog::shared(),
            ulp_results: TabResults::shared(),
            ulp_result: Arc::new(Mutex::new(None)),
            append_path: String::new(),
            append_running: false,
            append_status: String::new(),
            append_error: None,
            append_result: Arc::new(Mutex::new(None)),
            lmdb_path_input: lmdb,
            lmdb_apply_error: None,
            lang,
            instruction_open: false,
            sidebar_tab_icon: None,
            leakbase_logo_texture: None,
        };
        let _ = crate::config::save_ui_zoom(harmony::UI_ZOOM);
        app
    }

    fn ensure_sidebar_tab_icon(&mut self, ctx: &egui::Context) {
        if self.sidebar_tab_icon.is_some() {
            return;
        }
        if let Some(image) = sidebar_link_color_image() {
            self.sidebar_tab_icon = Some(ctx.load_texture(
                "sidebar_tab_link",
                image,
                egui::TextureOptions::LINEAR,
            ));
        }
    }

    fn ensure_leakbase_logo_texture(&mut self, ctx: &egui::Context) {
        if self.leakbase_logo_texture.is_some() {
            return;
        }
        if let Some(image) = leakbase_logo_color_image() {
            self.leakbase_logo_texture = Some(ctx.load_texture(
                "leakbase_logo",
                image,
                egui::TextureOptions::LINEAR,
            ));
        }
    }

    fn ui_leakbase_sponsor(&mut self, ui: &mut egui::Ui) {
        let Some(texture) = self.leakbase_logo_texture.as_ref() else {
            return;
        };
        let t = self.tr();
        const LOGO_H: f32 = 28.0;
        let size = texture.size_vec2();
        let aspect = if size.y > 0.0 { size.x / size.y } else { 1.0 };
        let logo_size = egui::vec2(LOGO_H * aspect, LOGO_H);

        let sponsor_w = ui
            .fonts(|f| {
                f.layout_no_wrap(
                    t.sponsor_label.to_owned(),
                    egui::FontId::proportional(11.0),
                    harmony::MUTED,
                )
                .size()
                .x
            })
            .max(0.0);
        let total_size = egui::vec2(sponsor_w + 6.0 + logo_size.x, logo_size.y.max(14.0));
        let (rect, response) = ui.allocate_exact_size(total_size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let cy = rect.center().y;
            ui.painter().text(
                egui::pos2(rect.left(), cy),
                egui::Align2::LEFT_CENTER,
                t.sponsor_label,
                egui::FontId::proportional(11.0),
                harmony::MUTED,
            );
            let logo_rect = egui::Rect::from_min_size(
                egui::pos2(rect.left() + sponsor_w + 6.0, rect.top() + (rect.height() - logo_size.y) * 0.5),
                logo_size,
            );
            ui.painter().image(
                texture.id(),
                logo_rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        }
        if response.clicked() {
            open_external_url(LEAKBASE_FORUM_URL);
        }
        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        response.on_hover_text(t.sponsor_forum_tip);
    }

    fn tr(&self) -> &'static i18n::I18n {
        i18n::tr(self.lang)
    }

    fn lmdb_path_display(&self) -> String {
        let path = self.db.lmdb_path();
        path.canonicalize()
            .unwrap_or(path)
            .display()
            .to_string()
    }

    fn set_lang(&mut self, lang: Lang) {
        if self.lang == lang {
            return;
        }
        self.lang = lang;
        let _ = save_ui_lang(lang);
        if let Ok(n) = self.db.open_existing() {
            self.db_status = if n > 0 {
                i18n::db_status_entries(lang, n)
            } else {
                self.tr().db_path_saved_empty.into()
            };
        }
    }

    fn pick_lmdb_folder(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .set_title(self.tr().dialog_lmdb_folder)
            .pick_folder()
        {
            self.lmdb_path_input = normalize_lmdb_path(&path).display().to_string();
            self.lmdb_apply_error = None;
        }
    }

    fn apply_lmdb_path(&mut self) {
        let raw = self.lmdb_path_input.trim();
        if raw.is_empty() {
            self.lmdb_apply_error = Some(self.tr().err_lmdb_path.into());
            return;
        }

        let normalized = normalize_lmdb_path(PathBuf::from(raw).as_path());
        if let Err(e) = save_lmdb_path(&normalized) {
            self.lmdb_apply_error = Some(format!("{}: {e}", self.tr().err_save_config));
            return;
        }

        self.db.set_lmdb_path(normalized.clone());
        self.lmdb_path_input = normalized.display().to_string();
        self.lmdb_apply_error = None;
        self.error = None;

        match self.db.open_existing() {
            Ok(n) if n > 0 => {
                self.db_status = i18n::db_status_entries(self.lang, n);
            }
            Ok(_) => {
                self.db_status = self.tr().db_path_saved_empty.into();
            }
            Err(e) => {
                self.lmdb_apply_error = Some(e.to_string());
            }
        }
    }

    fn pick_input_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text", &["txt", "csv", "log"])
            .pick_file()
        {
            self.input_path = path.display().to_string();
        }
    }

    fn pick_merge_mail(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text", &["txt", "csv", "log"])
            .pick_file()
        {
            self.merge_mail_path = path.display().to_string();
        }
    }

    fn pick_merge_dehash(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text", &["txt", "csv", "log"])
            .pick_file()
        {
            self.merge_dehash_path = path.display().to_string();
        }
    }

    fn pick_sql_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("SQL", &["sql", "txt", "dump"])
            .pick_file()
        {
            self.sql_path = path.display().to_string();
        }
    }

    fn pick_sql_folder(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.sql_path = path.display().to_string();
        }
    }

    fn pick_regex_source(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text", &["txt", "sql", "csv", "log", "dump"])
            .pick_file()
        {
            self.regex_source_path = path.display().to_string();
        }
    }

    fn pick_sqlcol_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("SQL", &["sql", "txt", "dump"])
            .pick_file()
        {
            self.sqlcol_path = path.display().to_string();
        }
    }

    fn pick_sqlcol_folder(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.sqlcol_path = path.display().to_string();
        }
    }

    fn pick_combo_input(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text", &["txt", "csv", "log", "sql", "json"])
            .pick_file()
        {
            self.combo_input = path.display().to_string();
        }
    }

    fn pick_combo_input_b(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text", &["txt", "csv", "log", "sql", "json"])
            .pick_file()
        {
            self.combo_input_b = path.display().to_string();
        }
    }

    fn pick_combo_output(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text", &["txt", "csv", "log"])
            .save_file()
        {
            self.combo_output = path.display().to_string();
        }
    }

    fn pick_combo_output_dir(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.combo_output_dir = path.display().to_string();
        }
    }

    fn pick_ulp_input(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text", &["txt", "csv", "log", "ulp", "lst"])
            .pick_file()
        {
            self.ulp_input = path.display().to_string();
        }
    }

    fn pick_ulp_input_folder(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.ulp_input = path.display().to_string();
        }
    }

    fn pick_ulp_output(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text", &["txt", "csv", "log"])
            .save_file()
        {
            self.ulp_output = path.display().to_string();
        }
    }

    fn pick_ulp_output_dir(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.ulp_output_dir = path.display().to_string();
        }
    }

    fn start_ulp(&mut self) {
        let t = self.tr();
        if self.ulp_input.trim().is_empty() {
            self.ulp_error = Some(t.err_input_file.into());
            return;
        }
        if self.ulp_tool.needs_output_dir() && self.ulp_output_dir.trim().is_empty() {
            self.ulp_error = Some(t.err_ulp_output_dir.into());
            return;
        }
        if !self.ulp_tool.needs_output_dir() && self.ulp_output.trim().is_empty() {
            self.ulp_error = Some(t.err_ulp_output.into());
            return;
        }
        self.ulp_error = None;
        self.ulp_running = true;
        self.ulp_control.reset();
        self.ulp_log
            .push(i18n::log_start_path(self.lang, self.ulp_input.trim()));
        self.ulp_status = self.ulp_tool.label(self.lang).into();
        *self.ulp_result.lock().unwrap() = None;

        let tool = self.ulp_tool;
        let input = self.ulp_input.clone();
        let output = self.ulp_output.clone();
        let output_dir = self.ulp_output_dir.clone();
        let keywords: Vec<String> = self
            .ulp_keywords
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let control = Arc::clone(&self.ulp_control);
        let slot = Arc::clone(&self.ulp_result);

        thread::spawn(move || {
            let result = ulp::run_tool(
                tool,
                &input,
                &output,
                &output_dir,
                &keywords,
                Some(&control),
            )
            .map_err(|e| e.to_string());
            *slot.lock().unwrap() = Some(result);
        });
    }

    fn start_sql_columns_extract(&mut self) {
        let t = self.tr();
        if self.sqlcol_path.is_empty() {
            self.sqlcol_error = Some(t.err_file_or_folder.into());
            return;
        }
        let path = PathBuf::from(&self.sqlcol_path);
        if !path.exists() {
            self.sqlcol_error = Some(t.err_path_not_found.into());
            return;
        }

        self.sqlcol_error = None;
        self.sqlcol_running = true;
        self.sqlcol_control.reset();
        self.sqlcol_log
            .push(i18n::log_start_path(self.lang, &path.display().to_string()));
        self.sqlcol_batch = path.is_dir();
        *self.sqlcol_result.lock().unwrap() = None;
        self.sqlcol_live.lock().unwrap().reset();
        self.sqlcol_live.lock().unwrap().lang = self.lang;

        let slot = Arc::clone(&self.sqlcol_result);
        let live = Arc::clone(&self.sqlcol_live);
        let control = Arc::clone(&self.sqlcol_control);
        let lang = self.lang;

        if path.is_dir() {
            self.sqlcol_status = t.status_sqlcol_batch.into();
            thread::spawn(move || {
                *slot.lock().unwrap() = Some(
                    match sql_columns::extract_folder(&path, Some(&live), Some(&control), lang) {
                        Ok(batch) => SqlColumnsStats {
                            written: batch.written,
                            skipped: batch.skipped,
                            tables_found: batch.tables_found,
                            inserts_parsed: batch.inserts_parsed,
                            lines_scanned: batch.lines_scanned,
                            output_path: batch.summary(lang),
                            ..Default::default()
                        },
                        Err(e) => SqlColumnsStats {
                            output_path: i18n::wrap_err(&e.to_string()),
                            ..Default::default()
                        },
                    },
                );
            });
            return;
        }

        if !path.is_file() {
            self.sqlcol_running = false;
            self.sqlcol_batch = false;
            self.sqlcol_error = Some(t.err_file_or_folder.into());
            return;
        }

        self.sqlcol_batch = false;

        let source = path;
        let output = sql_columns::default_output(&source);
        self.sqlcol_status = t.status_sqlcol_parsing.into();

        thread::spawn(move || {
            *slot.lock().unwrap() = Some(
                match sql_columns::extract_from_sql_columns(&source, &output, Some(&control)) {
                    Ok(s) => s,
                    Err(e) => SqlColumnsStats {
                        output_path: i18n::wrap_err(&e.to_string()),
                        ..Default::default()
                    },
                },
            );
        });
    }

    fn apply_regex_preset(&mut self, idx: usize) {
        if let Some(p) = PRESETS.get(idx) {
            self.regex_preset_idx = idx;
            self.regex_pattern = p.pattern.into();
            self.regex_template = p.template.into();
            self.regex_case_insensitive = p.case_insensitive;
        }
    }

    fn start_regex_extract(&mut self) {
        let t = self.tr();
        if self.regex_source_path.is_empty() {
            self.regex_error = Some(t.err_regex_source.into());
            return;
        }
        if self.regex_pattern.trim().is_empty() {
            self.regex_error = Some(t.err_regex_pattern.into());
            return;
        }
        if self.regex_template.trim().is_empty() {
            self.regex_error = Some(t.err_regex_template.into());
            return;
        }

        let source = PathBuf::from(&self.regex_source_path);
        if !source.is_file() {
            self.regex_error = Some(t.err_file_not_found.into());
            return;
        }

        if let Err(e) = regex_extract::compile_regex(&RegexExtractConfig {
            pattern: self.regex_pattern.clone(),
            output_template: self.regex_template.clone(),
            case_insensitive: self.regex_case_insensitive,
            multiline: self.regex_multiline,
            dot_matches_newline: self.regex_dotall,
            dedupe: self.regex_dedupe,
            skip_empty: true,
        }) {
            self.regex_error = Some(format!("Regex: {e}"));
            return;
        }

        let output = regex_extract::default_output(&source);
        let config = RegexExtractConfig {
            pattern: self.regex_pattern.clone(),
            output_template: self.regex_template.clone(),
            case_insensitive: self.regex_case_insensitive,
            multiline: self.regex_multiline,
            dot_matches_newline: self.regex_dotall,
            dedupe: self.regex_dedupe,
            skip_empty: true,
        };

        self.regex_error = None;
        self.regex_running = true;
        self.regex_control.reset();
        self.regex_log
            .push(i18n::log_start_path(self.lang, &source.display().to_string()));
        self.regex_status = t.status_regex_extract.into();
        *self.regex_result.lock().unwrap() = None;

        let slot = Arc::clone(&self.regex_result);
        let control = Arc::clone(&self.regex_control);
        thread::spawn(move || {
            *slot.lock().unwrap() = Some(
                match regex_extract::extract_with_regex(&source, &output, &config, Some(&control)) {
                Ok(s) => s,
                Err(e) => RegexExtractStats {
                    output_path: i18n::wrap_err(&e.to_string()),
                    ..Default::default()
                },
            });
        });
    }

    fn start_combo(&mut self) {
        let t = self.tr();
        if self.combo_input.trim().is_empty() {
            self.combo_error = Some(t.err_input_file.into());
            return;
        }
        if matches!(self.combo_tool, ComboTool::Compare) && self.combo_input_b.trim().is_empty() {
            self.combo_error = Some(t.err_combo_input_b.into());
            return;
        }
        let needs_dir = matches!(
            self.combo_tool,
            ComboTool::Compare
                | ComboTool::MxCheck
                | ComboTool::Analyze
                | ComboTool::SplitNamePass
        );
        if needs_dir && self.combo_output_dir.trim().is_empty() {
            self.combo_error = Some(t.err_combo_output_dir.into());
            return;
        }
        if !needs_dir && self.combo_output.trim().is_empty() {
            self.combo_error = Some(t.err_combo_output.into());
            return;
        }
        self.combo_error = None;
        self.combo_running = true;
        self.combo_control.reset();
        self.combo_log
            .push(i18n::log_start_path(self.lang, self.combo_input.trim()));
        self.combo_status = self.combo_tool.label(self.lang).into();
        *self.combo_result.lock().unwrap() = None;

        let tool = self.combo_tool;
        let input = self.combo_input.clone();
        let input_b = self.combo_input_b.clone();
        let output = self.combo_output.clone();
        let output_dir = self.combo_output_dir.clone();
        let filter = self.combo_filter.clone();
        let use_regex = self.combo_use_regex;
        let lines_per_file = self.combo_lines_per_file;
        let control = Arc::clone(&self.combo_control);
        let slot = Arc::clone(&self.combo_result);

        thread::spawn(move || {
            let result = combo::run_tool(
                tool,
                &input,
                &input_b,
                &output,
                &output_dir,
                &filter,
                use_regex,
                lines_per_file,
                Some(&control),
            )
            .map_err(|e| e.to_string());
            *slot.lock().unwrap() = Some(result);
        });
    }

    fn pick_append_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text", &["txt", "csv", "log"])
            .pick_file()
        {
            self.append_path = path.display().to_string();
        }
    }

    fn start(&mut self) {
        let t = self.tr();
        if self.input_path.is_empty() {
            self.error = Some(t.err_input_file.into());
            return;
        }
        if !self.db.is_open() {
            match self.db.open_existing() {
                Ok(0) | Err(_) => {
                    self.error = Some(self.tr().err_no_lmdb_hint.into());
                    return;
                }
                Ok(n) => {
                    self.db_status = i18n::db_status_entries(self.lang, n);
                }
            }
        }

        let input = PathBuf::from(&self.input_path);
        if !input.is_file() {
            self.error = Some(t.err_file_not_found.into());
            return;
        }

        self.error = None;
        self.status.clear();
        self.running = true;
        self.lookup_control.reset();
        self.lookup_log
            .push(i18n::log_start_path(self.lang, &input.display().to_string()));
        *self.progress.lock().unwrap() = Progress::default();

        let db = Arc::clone(&self.db);
        let threads = self.threads as usize;
        let control = Arc::clone(&self.lookup_control);
        let progress = Arc::clone(&self.progress);
        let input_clone = input.clone();

        thread::spawn(move || {
            let prog = Arc::clone(&progress);
            let result = processor::process_file(db, input_clone, threads, control, move |p| {
                *prog.lock().unwrap() = p;
            });
            if let Err(e) = result {
                tracing::error!(error = %e, "process failed");
                let mut g = progress.lock().unwrap();
                g.done = true;
                g.file = i18n::wrap_err(&e.to_string());
            }
        });
    }

    fn stop_lookup(&mut self) {
        self.stop_job(TabJob::Lookup);
    }

    fn toggle_pause_lookup(&mut self) {
        self.toggle_pause_job(TabJob::Lookup);
    }

    fn start_merge(&mut self) {
        let t = self.tr();
        if self.merge_mail_path.is_empty() || self.merge_dehash_path.is_empty() {
            self.merge_error = Some(t.err_both_merge_files.into());
            return;
        }
        let mail = PathBuf::from(&self.merge_mail_path);
        let dehash = PathBuf::from(&self.merge_dehash_path);
        if !mail.is_file() || !dehash.is_file() {
            self.merge_error = Some(t.err_merge_file_missing.into());
            return;
        }

        self.merge_error = None;
        self.merge_running = true;
        self.merge_control.reset();
        self.merge_log.push(i18n::log_start_path(
            self.lang,
            &format!("{} + {}", mail.display(), dehash.display()),
        ));
        self.merge_status = t.status_merge_loading.into();
        *self.merge_result.lock().unwrap() = None;

        let result_slot = Arc::clone(&self.merge_result);
        let control = Arc::clone(&self.merge_control);
        thread::spawn(move || {
            let outcome = merger::merge_files(&mail, &dehash, Some(&control));
            *result_slot.lock().unwrap() = Some(match outcome {
                Ok(stats) => stats,
                Err(e) => MergeStats {
                    plain_path: i18n::wrap_err(&e.to_string()),
                    ..Default::default()
                },
            });
        });
    }

    fn start_sql_extract(&mut self) {
        let t = self.tr();
        if self.sql_path.is_empty() {
            self.sql_error = Some(t.err_file_or_folder.into());
            return;
        }
        let path = PathBuf::from(&self.sql_path);
        if !path.exists() {
            self.sql_error = Some(t.err_path_not_found.into());
            return;
        }

        self.sql_error = None;
        self.sql_running = true;
        self.sql_control.reset();
        self.sql_log.push(i18n::log_start_threads(
            self.lang,
            &path.display().to_string(),
            self.sql_threads,
        ));
        self.sql_batch = path.is_dir();
        *self.sql_result.lock().unwrap() = None;
        self.sql_live.lock().unwrap().reset();
        self.sql_live.lock().unwrap().lang = self.lang;

        let slot = Arc::clone(&self.sql_result);
        let live = Arc::clone(&self.sql_live);
        let control = Arc::clone(&self.sql_control);
        let threads = self.sql_threads as usize;
        let lang = self.lang;

        if path.is_dir() {
            self.sql_status = t
                .status_sql_batch
                .replace("{threads}", &threads.to_string());
            thread::spawn(move || {
                *slot.lock().unwrap() = Some(
                    match sql_extract::extract_folder(
                        &path,
                        Some(&live),
                        Some(&control),
                        threads,
                        lang,
                    ) {
                        Ok(batch) => ExtractStats {
                            total: batch.total,
                            md5: batch.md5,
                            sha1: batch.sha1,
                            trash: batch.trash,
                            lines_scanned: batch.lines_scanned,
                            output_path: batch.summary(lang),
                            trash_path: String::new(),
                        },
                        Err(e) => ExtractStats {
                            output_path: i18n::wrap_err(&e.to_string()),
                            ..Default::default()
                        },
                    },
                );
            });
            return;
        }

        if !path.is_file() {
            self.sql_running = false;
            self.sql_batch = false;
            self.sql_error = Some(t.err_file_or_folder.into());
            return;
        }

        self.sql_batch = false;

        let source = path;
        let output = sql_extract::default_output(&source);
        let trash = sql_extract::default_trash_output(&source);
        self.sql_status = t
            .status_sql_parsing
            .replace("{threads}", &threads.to_string());

        thread::spawn(move || {
            *slot.lock().unwrap() = Some(
                match sql_extract::extract_from_sql_with_trash(
                    &source,
                    &output,
                    &trash,
                    Some(&control),
                    threads,
                )
                {
                    Ok(s) => s,
                    Err(e) => ExtractStats {
                        output_path: i18n::wrap_err(&e.to_string()),
                        ..Default::default()
                    },
                },
            );
        });
    }

    fn start_append(&mut self) {
        let t = self.tr();
        if self.running {
            self.append_error = Some(t.err_stop_lookup_before_append.into());
            return;
        }
        if self.append_path.is_empty() {
            self.append_error = Some(t.err_hashpass_file.into());
            return;
        }
        let path = PathBuf::from(&self.append_path);
        if !path.is_file() {
            self.append_error = Some(t.err_file_not_found.into());
            return;
        }

        self.append_error = None;
        self.append_running = true;
        self.append_status = t.status_append.into();
        *self.append_result.lock().unwrap() = None;

        let db = Arc::clone(&self.db);
        let slot = Arc::clone(&self.append_result);
        thread::spawn(move || {
            let result = db.append(&[path], 280).map_err(|e| e.to_string());
            *slot.lock().unwrap() = Some(result);
        });
    }

    fn tab_job(&self) -> TabJob {
        match self.tab {
            Tab::Lookup => TabJob::Lookup,
            Tab::Merge => TabJob::Merge,
            Tab::ExtractSql => TabJob::Sql,
            Tab::SqlColumns => TabJob::SqlColumns,
            Tab::CustomRegex => TabJob::Regex,
            Tab::Combo => TabJob::Combo,
            Tab::Ulp => TabJob::Ulp,
        }
    }

    fn control_for(&self, job: TabJob) -> &Arc<JobControl> {
        match job {
            TabJob::Lookup => &self.lookup_control,
            TabJob::Merge => &self.merge_control,
            TabJob::Sql => &self.sql_control,
            TabJob::SqlColumns => &self.sqlcol_control,
            TabJob::Regex => &self.regex_control,
            TabJob::Combo => &self.combo_control,
            TabJob::Ulp => &self.ulp_control,
        }
    }

    fn log_for(&self, job: TabJob) -> &Arc<TabLog> {
        match job {
            TabJob::Lookup => &self.lookup_log,
            TabJob::Merge => &self.merge_log,
            TabJob::Sql => &self.sql_log,
            TabJob::SqlColumns => &self.sqlcol_log,
            TabJob::Regex => &self.regex_log,
            TabJob::Combo => &self.combo_log,
            TabJob::Ulp => &self.ulp_log,
        }
    }

    fn results_for(&self, job: TabJob) -> &Arc<TabResults> {
        match job {
            TabJob::Lookup => &self.lookup_results,
            TabJob::Merge => &self.merge_results,
            TabJob::Sql => &self.sql_results,
            TabJob::SqlColumns => &self.sqlcol_results,
            TabJob::Regex => &self.regex_results,
            TabJob::Combo => &self.combo_results,
            TabJob::Ulp => &self.ulp_results,
        }
    }

    fn is_job_running(&self, job: TabJob) -> bool {
        match job {
            TabJob::Lookup => self.running,
            TabJob::Merge => self.merge_running,
            TabJob::Sql => self.sql_running,
            TabJob::SqlColumns => self.sqlcol_running,
            TabJob::Regex => self.regex_running,
            TabJob::Combo => self.combo_running,
            TabJob::Ulp => self.ulp_running,
        }
    }

    fn can_start_job(&self, job: TabJob) -> bool {
        match job {
            TabJob::Lookup => !self.input_path.is_empty(),
            TabJob::Merge => {
                !self.merge_mail_path.is_empty() && !self.merge_dehash_path.is_empty()
            }
            TabJob::Sql => !self.sql_path.is_empty(),
            TabJob::SqlColumns => !self.sqlcol_path.is_empty(),
            TabJob::Regex => {
                !self.regex_source_path.is_empty()
                    && !self.regex_pattern.trim().is_empty()
                    && !self.regex_template.trim().is_empty()
            }
            TabJob::Combo => !self.combo_input.is_empty(),
            TabJob::Ulp => !self.ulp_input.is_empty(),
        }
    }

    fn start_label(&self, job: TabJob) -> &'static str {
        match job {
            TabJob::Lookup => "▶ Старт",
            TabJob::Merge => "▶ Склеить",
            TabJob::Sql | TabJob::SqlColumns | TabJob::Regex | TabJob::Combo | TabJob::Ulp => {
                "▶ Извлечь"
            }
        }
    }

    fn status_badge(&self) -> (&'static str, egui::Color32) {
        let t = self.tr();
        if self.running {
            return (t.badge_lookup, harmony::PRIMARY);
        }
        if self.merge_running {
            return (t.badge_merge, harmony::AMBER);
        }
        if self.sql_running {
            return (t.badge_sql, harmony::ROSE);
        }
        if self.sqlcol_running {
            return (t.badge_columns, harmony::ORANGE);
        }
        if self.regex_running {
            return (t.badge_regex, harmony::CYAN);
        }
        if self.combo_running {
            return (t.badge_combo, harmony::SUCCESS);
        }
        if self.ulp_running {
            return (t.badge_ulp, harmony::SKY);
        }
        if self.append_running {
            return (t.badge_append, harmony::VIOLET);
        }
        (t.badge_idle, harmony::MUTED)
    }

    fn start_current_job(&mut self) {
        match self.tab_job() {
            TabJob::Lookup => self.start(),
            TabJob::Merge => self.start_merge(),
            TabJob::Sql => self.start_sql_extract(),
            TabJob::SqlColumns => self.start_sql_columns_extract(),
            TabJob::Regex => self.start_regex_extract(),
            TabJob::Combo => self.start_combo(),
            TabJob::Ulp => self.start_ulp(),
        }
    }

    fn stop_job(&mut self, job: TabJob) {
        let t = self.tr();
        self.control_for(job).request_stop();
        self.log_for(job).push(t.log_stop_requested);
        let stopping = t.status_stopping.to_string();
        match job {
            TabJob::Lookup => self.status = stopping.clone(),
            TabJob::Merge => self.merge_status = stopping.clone(),
            TabJob::Sql => self.sql_status = stopping.clone(),
            TabJob::SqlColumns => self.sqlcol_status = stopping.clone(),
            TabJob::Regex => self.regex_status = stopping.clone(),
            TabJob::Combo => self.combo_status = stopping.clone(),
            TabJob::Ulp => self.ulp_status = stopping,
        }
    }

    fn toggle_pause_job(&mut self, job: TabJob) {
        self.control_for(job).toggle_pause();
        let t = self.tr();
        if self.control_for(job).is_paused() {
            self.log_for(job).push(t.log_pause);
        } else {
            self.log_for(job).push(t.log_resume);
        }
    }

    fn open_current_results(&self) {
        let job = self.tab_job();
        if let Some(folder) = self.results_for(job).folder() {
            open_in_explorer(&folder);
            self.log_for(job).push(
                self.tr()
                    .log_folder_opened
                    .replace("{path}", &folder.display().to_string()),
            );
        }
    }

    fn delete_current_results(&mut self) {
        let job = self.tab_job();
        let t = self.tr();
        match self.results_for(job).delete_all() {
            Ok(n) => self
                .log_for(job)
                .push(t.log_deleted.replace("{n}", &n.to_string())),
            Err(e) => self.log_for(job).push(
                t.log_delete_err.replace("{e}", &e.to_string()),
            ),
        }
    }

    fn zip_current_results(&mut self) {
        let job = self.tab_job();
        let t = self.tr();
        match self.results_for(job).zip_pack(t.no_result_files) {
            Ok(path) => self.log_for(job).push(
                t.log_zip.replace("{path}", &path.display().to_string()),
            ),
            Err(e) => self
                .log_for(job)
                .push(t.log_zip_err.replace("{e}", &e.to_string())),
        }
    }

    fn merge_current_results(&mut self) {
        let job = self.tab_job();
        let t = self.tr();
        match self.results_for(job).merge_text() {
            Ok(path) => self.log_for(job).push(
                t.log_merged_one
                    .replace("{path}", &path.display().to_string()),
            ),
            Err(e) => self
                .log_for(job)
                .push(t.log_merge_err.replace("{e}", &e.to_string())),
        }
    }

    fn tab_page_title(&self) -> (&str, &str) {
        let t = self.tr();
        match self.tab {
            Tab::Lookup => (t.tab_lookup, t.app_subtitle),
            Tab::Merge => (t.tab_merge, t.merge_subtitle),
            Tab::ExtractSql => (t.tab_sql, t.sql_subtitle),
            Tab::SqlColumns => (t.tab_columns, t.sqlcol_intro),
            Tab::CustomRegex => (t.tab_regex, t.regex_intro),
            Tab::Combo => (t.tab_combo, t.combo_subtitle),
            Tab::Ulp => (t.tab_ulp, t.ulp_subtitle),
        }
    }

    fn ui_content_header(&mut self, ui: &mut egui::Ui) {
        self.ensure_leakbase_logo_texture(ui.ctx());
        let (page_title, page_sub) = self.tab_page_title();
        let page_title = page_title.to_string();
        let page_sub = page_sub.to_string();
        ui.set_min_height(harmony::CONTENT_HEADER_H - 8.0);
        let ctx = ui.ctx().clone();
        ui.horizontal(|ui| {
            let title_rect = ui
                .horizontal(|ui| {
                    harmony::page_title(ui, &page_title, &page_sub);
                })
                .response
                .rect;
            harmony::window_drag_region(&ctx, ui, title_rect, "header_drag");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                harmony::window_controls(&ctx, ui);
                self.ui_leakbase_sponsor(ui);
                self.ui_lang_controls(ui);
                let t = self.tr();
                let path = self.lmdb_path_display();
                harmony::db_pill(ui, &self.db_status, &path, t.tip_hash_db_path);
                harmony::vsep(ui);
                let (label, color) = self.status_badge();
                harmony::status_pill(ui, label, color);
            });
        });
    }

    fn ui_action_bar(&mut self, ui: &mut egui::Ui) {
        let job = self.tab_job();
        let t = self.tr();
        let running = self.is_job_running(job);
        let can_start = self.can_start_job(job) && !running;
        let has_results = self.results_for(job).has_files();
        let paused = self.control_for(job).is_paused();
        let start_label = match job {
            TabJob::Lookup => t.btn_start,
            TabJob::Merge => t.btn_merge,
            TabJob::Sql | TabJob::SqlColumns | TabJob::Regex | TabJob::Combo | TabJob::Ulp => {
                t.btn_extract
            }
        };
        let pause_label = if paused { t.btn_resume } else { t.btn_pause };

        ui.horizontal_wrapped(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);
                if ui
                    .add_enabled_ui(has_results && !running, |ui| {
                        harmony::toolbar_neutral(ui, t.btn_merge_one)
                    })
                    .inner
                    .clicked()
                {
                    self.merge_current_results();
                }
                if ui
                    .add_enabled_ui(has_results && !running, |ui| {
                        harmony::toolbar_archive(ui, t.btn_zip)
                    })
                    .inner
                    .clicked()
                {
                    self.zip_current_results();
                }
                if ui
                    .add_enabled_ui(has_results && !running, |ui| {
                        harmony::toolbar_delete(ui, t.btn_delete)
                    })
                    .inner
                    .clicked()
                {
                    self.delete_current_results();
                }
                if ui
                    .add_enabled_ui(has_results && !running, |ui| {
                        harmony::toolbar_results(ui, t.btn_results)
                    })
                    .inner
                    .clicked()
                {
                    self.open_current_results();
                }
                if ui
                    .add_enabled_ui(running, |ui| harmony::toolbar_stop(ui, t.btn_stop))
                    .inner
                    .clicked()
                {
                    self.stop_job(job);
                }
                if ui
                    .add_enabled_ui(running, |ui| harmony::toolbar_pause(ui, pause_label))
                    .inner
                    .clicked()
                {
                    self.toggle_pause_job(job);
                }
                if ui
                    .add_enabled_ui(can_start, |ui| harmony::toolbar_start(ui, start_label))
                    .inner
                    .clicked()
                {
                    self.start_current_job();
                }
            });
        });
    }

    fn ui_log_card(&mut self, ui: &mut egui::Ui) {
        let job = self.tab_job();
        let t = self.tr();
        let log_text = self.log_for(job).text();
        harmony::section_frame().show(ui, |ui| {
            harmony::fill_width(ui);
            ui.set_min_height(harmony::LOG_CARD_H - 28.0);
            harmony::section_title(ui, t.section_log);
            harmony::log_frame().show(ui, |ui| {
                harmony::fill_width(ui);
                ui.set_min_height(56.0);
                ui.label(
                    egui::RichText::new(if log_text.is_empty() {
                        t.log_waiting
                    } else {
                        &log_text
                    })
                    .size(harmony::FONT_TINY)
                    .monospace()
                    .color(harmony::LOG_TEXT),
                );
            });
        });
    }

    fn ui_lang_controls(&mut self, ui: &mut egui::Ui) {
        let t = self.tr();
        egui::Frame::none()
            .fill(harmony::INPUT)
            .stroke(egui::Stroke::new(1.0_f32, harmony::BORDER))
            .rounding(egui::Rounding::same(harmony::ROUND))
            .inner_margin(egui::vec2(8.0, 4.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    ui.label(
                        egui::RichText::new(t.lang_label)
                            .size(harmony::FONT_TINY)
                            .color(harmony::SECONDARY),
                    );
                    if ui
                        .selectable_label(self.lang == Lang::Ru, "RU")
                        .clicked()
                    {
                        self.set_lang(Lang::Ru);
                    }
                    if ui
                        .selectable_label(self.lang == Lang::En, "EN")
                        .clicked()
                    {
                        self.set_lang(Lang::En);
                    }
                });
            });
    }

    fn ui_sidebar(&mut self, ui: &mut egui::Ui) {
        let t = self.tr();
        self.ensure_sidebar_tab_icon(ui.ctx());
        let Some(icon) = self.sidebar_tab_icon.as_ref() else {
            return;
        };
        let icon_id = icon.id();
        let ctx = ui.ctx().clone();
        let brand_rect = ui
            .horizontal(|ui| {
                harmony::sidebar_brand(ui, t.app_subtitle);
            })
            .response
            .rect;
        harmony::window_drag_region(&ctx, ui, brand_rect, "sidebar_brand_drag");
        ui.add_space(16.0);
        let body_size = egui::vec2(ui.available_width(), ui.available_height());
        ui.allocate_ui_with_layout(body_size, egui::Layout::top_down(egui::Align::Min), |ui| {
            let footer_h = 36.0;
            let scroll_h = (ui.available_height() - footer_h - 8.0).max(0.0);
            egui::ScrollArea::vertical()
                .id_salt("sidebar_tabs")
                .auto_shrink([false; 2])
                .max_height(scroll_h)
                .show(ui, |ui| {
                    harmony::fill_width(ui);
                    ui.spacing_mut().item_spacing.y = 4.0;
                    harmony::sidebar_section_title(
                        ui,
                        if self.lang == Lang::Ru { "Меню" } else { "Menu" },
                    );
                    ui.add_space(6.0);
                    let tabs = [
                        (Tab::Lookup, t.tab_lookup),
                        (Tab::Merge, t.tab_merge),
                        (Tab::ExtractSql, t.tab_sql),
                        (Tab::SqlColumns, t.tab_columns),
                        (Tab::CustomRegex, t.tab_regex),
                        (Tab::Combo, t.tab_combo),
                        (Tab::Ulp, t.tab_ulp),
                    ];
                    for (tab, label) in tabs {
                        if harmony::sidebar_nav(ui, icon_id, label, self.tab == tab).clicked() {
                            self.tab = tab;
                            self.instruction_open = false;
                        }
                    }
                });
            ui.add_space(8.0);
            egui::Frame::none()
                .fill(harmony::CARD)
                .stroke(egui::Stroke::new(1.0_f32, harmony::BORDER))
                .rounding(egui::Rounding::same(harmony::BTN_ROUND))
                .inner_margin(egui::vec2(10.0, 8.0))
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("v1 · Local only")
                            .size(harmony::FONT_TINY)
                            .color(harmony::MUTED),
                    );
                });
        });
    }


}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(harmony::UI_ZOOM);
        harmony::paint_window_shell(ctx);

        if self.running
            || self.merge_running
            || self.sql_running
            || self.regex_running
            || self.sqlcol_running
            || self.append_running
            || self.combo_running
            || self.ulp_running
        {
            ctx.request_repaint_after(std::time::Duration::from_millis(200));
        }

        if self.append_running {
            if let Some(result) = self.append_result.lock().unwrap().take() {
                self.append_running = false;
                match result {
                    Ok(stats) => {
                        self.db_status = i18n::db_status_entries(self.lang, stats.final_count);
                        self.append_status = i18n::append_status(
                            self.lang,
                            stats.added,
                            stats.skipped,
                            stats.bad_lines,
                            stats.final_count,
                        );
                    }
                    Err(e) => {
                        self.append_error = Some(e);
                        self.append_status.clear();
                    }
                }
            }
        }

        if self.merge_running {
            if let Some(stats) = self.merge_result.lock().unwrap().clone() {
                self.merge_running = false;
                if i18n::is_err(&stats.plain_path) {
                    let msg = i18n::format_error_display(self.lang, &stats.plain_path);
                    self.merge_error = Some(msg.clone());
                    self.merge_status.clear();
                    self.merge_log.push(msg);
                } else {
                    self.merge_results.set_from_strings([
                        stats.plain_path.clone(),
                        stats.nohash_path.clone(),
                        stats.trash_path.clone(),
                    ]);
                    self.merge_log.push(i18n::merge_done_short(
                        self.lang,
                        stats.merged,
                        stats.nohash,
                        stats.trash,
                    ));
                    self.merge_status = i18n::merge_done_full(
                        self.lang,
                        stats.merged,
                        stats.nohash,
                        stats.bad,
                        stats.trash,
                        stats.total,
                        &stats.plain_path,
                        &stats.nohash_path,
                        &stats.trash_path,
                    );
                }
            }
        }

        if self.sql_running {
            if let Some(stats) = self.sql_result.lock().unwrap().clone() {
                self.sql_running = false;
                self.sql_batch = false;
                if i18n::is_err(&stats.output_path) {
                    let msg = i18n::format_error_display(self.lang, &stats.output_path);
                    self.sql_error = Some(msg.clone());
                    self.sql_status.clear();
                    self.sql_log.push(msg);
                } else if stats.trash_path.is_empty() && i18n::is_folder_batch(&stats.output_path) {
                    self.sql_log.push(self.tr().log_batch_done);
                    let folder = PathBuf::from(self.sql_path.trim());
                    if folder.is_dir() {
                        let paths = tab_results::collect_suffixes_in_folder(
                            &folder,
                            &["_emails.txt", "_trash.txt"],
                        );
                        self.sql_results.set_paths(paths);
                    }
                    self.sql_status = stats
                        .output_path
                        .strip_prefix(i18n::FOLDER_BATCH_PREFIX)
                        .unwrap_or(&stats.output_path)
                        .to_string();
                } else {
                    self.sql_results
                        .set_from_strings([stats.output_path.clone(), stats.trash_path.clone()]);
                    self.sql_log.push(i18n::sql_done_short(
                        self.lang,
                        stats.total,
                        stats.trash,
                    ));
                    self.sql_status = i18n::sql_done_full(
                        self.lang,
                        stats.total,
                        stats.md5,
                        stats.sha1,
                        stats.trash,
                        stats.lines_scanned,
                        &stats.output_path,
                        &stats.trash_path,
                    );
                }
            }
        }

        if self.regex_running {
            if let Some(stats) = self.regex_result.lock().unwrap().clone() {
                self.regex_running = false;
                if i18n::is_err(&stats.output_path) {
                    let msg = i18n::format_error_display(self.lang, &stats.output_path);
                    self.regex_error = Some(msg.clone());
                    self.regex_status.clear();
                    self.regex_log.push(msg);
                } else {
                    self.regex_results
                        .set_from_strings([stats.output_path.clone()]);
                    self.regex_log
                        .push(i18n::regex_done_log(self.lang, stats.written));
                    self.regex_status = i18n::regex_done_full(
                        self.lang,
                        stats.written,
                        stats.match_hits,
                        stats.duplicates,
                        stats.skipped_empty,
                        stats.lines_scanned,
                        &stats.output_path,
                    );
                }
            }
        }

        if self.combo_running {
            if let Some(result) = self.combo_result.lock().unwrap().take() {
                self.combo_running = false;
                match result {
                    Ok(summary) => {
                        self.combo_status = summary.message.clone();
                        self.combo_log.push(summary.message.clone());
                        if !self.combo_output.is_empty() {
                            self.combo_results
                                .set_from_strings([self.combo_output.clone()]);
                        } else if !self.combo_output_dir.is_empty() {
                            self.combo_results
                                .set_from_strings([self.combo_output_dir.clone()]);
                        }
                    }
                    Err(e) => {
                        self.combo_error = Some(e.clone());
                        self.combo_log.push(e);
                    }
                }
            }
        }

        if self.ulp_running {
            if let Some(result) = self.ulp_result.lock().unwrap().take() {
                self.ulp_running = false;
                match result {
                    Ok(summary) => {
                        self.ulp_status = summary.message.clone();
                        self.ulp_log.push(summary.message.clone());
                        if !self.ulp_output.is_empty() {
                            self.ulp_results
                                .set_from_strings([self.ulp_output.clone()]);
                        } else if !self.ulp_output_dir.is_empty() {
                            self.ulp_results
                                .set_from_strings([self.ulp_output_dir.clone()]);
                        }
                    }
                    Err(e) => {
                        self.ulp_error = Some(e.clone());
                        self.ulp_log.push(e);
                    }
                }
            }
        }

        if self.sqlcol_running {
            if let Some(stats) = self.sqlcol_result.lock().unwrap().clone() {
                self.sqlcol_running = false;
                self.sqlcol_batch = false;
                if i18n::is_err(&stats.output_path) {
                    let msg = i18n::format_error_display(self.lang, &stats.output_path);
                    self.sqlcol_error = Some(msg.clone());
                    self.sqlcol_status.clear();
                    self.sqlcol_log.push(msg);
                } else if i18n::is_folder_batch(&stats.output_path) {
                    self.sqlcol_log.push(self.tr().log_batch_done);
                    let folder = PathBuf::from(self.sqlcol_path.trim());
                    if folder.is_dir() {
                        let paths = tab_results::collect_suffixes_in_folder(
                            &folder,
                            &["_loginpass.txt"],
                        );
                        self.sqlcol_results.set_paths(paths);
                    }
                    self.sqlcol_status = stats
                        .output_path
                        .strip_prefix(i18n::FOLDER_BATCH_PREFIX)
                        .unwrap_or(&stats.output_path)
                        .to_string();
                } else {
                    self.sqlcol_results
                        .set_from_strings([stats.output_path.clone()]);
                    self.sqlcol_log
                        .push(i18n::sqlcol_done_log(self.lang, stats.written));
                    self.sqlcol_status = i18n::sqlcol_done_full(
                        self.lang,
                        stats.written,
                        stats.skipped,
                        stats.tables_found,
                        stats.inserts_parsed,
                        stats.lines_scanned,
                        &stats.output_path,
                    );
                }
            }
        }

        let snap = self.progress.lock().unwrap().clone();
        if snap.done && self.running {
            self.running = false;
            if i18n::is_err(&snap.file) {
                self.lookup_log
                    .push(i18n::format_error_display(self.lang, &snap.file));
            } else {
                let input = PathBuf::from(self.input_path.trim());
                if input.is_file() {
                    let (good, nohash, bad, trash) = processor::output_paths(&input);
                    self.lookup_results.set_paths([good, nohash, bad, trash]);
                } else if !snap.good_path.is_empty() {
                    self.lookup_results.set_from_strings([
                        snap.good_path.clone(),
                        snap.nohash_path.clone(),
                        snap.trash_path.clone(),
                    ]);
                }
                self.lookup_log.push(i18n::lookup_done_short(
                    self.lang,
                    snap.found,
                    snap.nohash,
                    snap.bad,
                    snap.trash,
                ));
            }
            self.status = i18n::lookup_done_full(
                self.lang,
                snap.found,
                snap.nohash,
                snap.bad,
                snap.trash,
                &snap.good_path,
                &snap.nohash_path,
                &snap.trash_path,
            );
        }

        egui::SidePanel::left("sidebar")
            .exact_width(harmony::SIDEBAR_W)
            .resizable(false)
            .frame(harmony::sidebar_frame())
            .show(ctx, |ui| {
                self.ui_sidebar(ui);
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(harmony::BG)
                    .inner_margin(egui::vec2(24.0, 20.0)),
            )
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = harmony::GRID_GAP;
                harmony::content_header_frame().show(ui, |ui| {
                    self.ui_content_header(ui);
                });
                harmony::action_bar_frame().show(ui, |ui| {
                    self.ui_action_bar(ui);
                });

                let log_reserve = harmony::LOG_CARD_H + harmony::GRID_GAP;
                let scroll_h = (ui.available_height() - log_reserve).max(160.0);
                egui::ScrollArea::vertical()
                    .id_salt("main_content")
                    .auto_shrink([false; 2])
                    .max_height(scroll_h)
                    .show(ui, |ui| {
                        harmony::fill_width(ui);
                        ui.spacing_mut().item_spacing =
                            egui::vec2(harmony::GRID_GAP, harmony::GRID_GAP);
                        match self.tab {
                            Tab::Lookup => self.ui_lookup(ui, &snap),
                            Tab::Merge => self.ui_merge(ui),
                            Tab::ExtractSql => self.ui_extract_sql(ui),
                            Tab::SqlColumns => self.ui_sql_columns(ui),
                            Tab::CustomRegex => self.ui_custom_regex(ui),
                            Tab::Combo => self.ui_combo(ui),
                            Tab::Ulp => self.ui_ulp(ui),
                        }
                    });

                self.ui_log_card(ui);
            });
    }
}

impl App {
    fn ui_lookup(&mut self, ui: &mut egui::Ui, snap: &Progress) {
        let t = self.tr();
        harmony::instruction_button(ui, &mut self.instruction_open, t.btn_instruction);

        let status_label = if self.running {
            if self.lang == Lang::Ru {
                "В работе"
            } else {
                "Running"
            }
        } else if snap.done {
            if self.lang == Lang::Ru {
                "Готово"
            } else {
                "Done"
            }
        } else {
            if self.lang == Lang::Ru {
                "Ожидание"
            } else {
                "Idle"
            }
        };
        let progress_pct = if snap.total > 0 {
            format!("{:.0}%", snap.processed as f32 / snap.total as f32 * 100.0)
        } else {
            "—".to_string()
        };

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = harmony::GRID_GAP;
            let tile_w = ((ui.available_width() - harmony::GRID_GAP * 3.0) / 4.0).max(120.0);
            ui.allocate_ui_with_layout(
                egui::vec2(tile_w, harmony::STAT_TILE_H),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    harmony::stat_tile(ui, t.section_database, &self.db_status, harmony::SUCCESS);
                },
            );
            ui.allocate_ui_with_layout(
                egui::vec2(tile_w, harmony::STAT_TILE_H),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    harmony::stat_tile(
                        ui,
                        t.section_threads,
                        &format!("{}", self.threads),
                        harmony::ACCENT_CYAN,
                    );
                },
            );
            ui.allocate_ui_with_layout(
                egui::vec2(tile_w, harmony::STAT_TILE_H),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    harmony::stat_tile(ui, "Status", status_label, harmony::ACCENT);
                },
            );
            ui.allocate_ui_with_layout(
                egui::vec2(tile_w, harmony::STAT_TILE_H),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    harmony::stat_tile(ui, t.heading_progress, &progress_pct, harmony::SECONDARY);
                },
            );
        });

        let w = ui.available_width();
        let gap = harmony::GRID_GAP;
        let min_col = 340.0;
        let two_col = w >= min_col * 2.0 + gap;
        if two_col {
            let col_w = ((w - gap) / 2.0).floor();
            ui.horizontal_top(|ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(col_w, 0.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.set_max_width(col_w);
                        self.ui_lookup_left(ui);
                    },
                );
                ui.add_space(gap);
                ui.allocate_ui_with_layout(
                    egui::vec2(col_w, 0.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.set_max_width(col_w);
                        self.ui_lookup_right(ui, snap);
                    },
                );
            });
        } else {
            self.ui_lookup_left(ui);
            ui.add_space(gap);
            self.ui_lookup_right(ui, snap);
        }
        harmony::instruction_modal(ui, &mut self.instruction_open, t.instruction, t.btn_close, |ui| {
            harmony::instruction_body(ui, t.lookup_instr_body);
            ui.add_space(6.0);
            harmony::instruction_heading(ui, t.lookup_instr_formats);
            harmony::instruction_mono(ui, t.lookup_instr_formats_mono);
            ui.add_space(4.0);
            harmony::instruction_heading(ui, t.lookup_instr_outputs);
            harmony::instruction_mono(ui, t.lookup_instr_outputs_mono);
            ui.add_space(4.0);
            harmony::instruction_heading(ui, t.lookup_instr_controls);
            harmony::instruction_body(ui, t.lookup_instr_controls_body);
        });
    }

    fn ui_lookup_left(&mut self, ui: &mut egui::Ui) {
        let t = self.tr();
        harmony::section_frame().show(ui, |ui| {
            harmony::fill_width(ui);
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_database);
            harmony::heading(ui, t.heading_lmdb);
            let can_open = !self.lmdb_path_input.trim().is_empty();
            let (browse, open) = harmony::path_browse_action_row(
                ui,
                &mut self.lmdb_path_input,
                t.hint_lmdb_path,
                t.browse,
                t.btn_open,
                can_open,
            );
            if browse {
                self.pick_lmdb_folder();
            }
            if open {
                self.apply_lmdb_path();
            }
            if let Some(err) = &self.lmdb_apply_error {
                harmony::body(ui, err, harmony::DANGER);
            } else {
                harmony::body(ui, &self.db_status.clone(), harmony::SUCCESS);
            }
            harmony::hash_db_path_row(
                ui,
                t.label_hash_db,
                &self.lmdb_path_display(),
                t.tip_hash_db_path,
            );

            ui.add_space(4.0);
            harmony::section_title(ui, t.section_append);
            let can_append =
                !self.append_path.is_empty() && !self.append_running && !self.running;
            let (browse, add) = harmony::path_browse_action_row(
                ui,
                &mut self.append_path,
                t.hint_append_file,
                t.browse,
                t.btn_add,
                can_append,
            );
            if browse {
                self.pick_append_file();
            }
            if add {
                self.start_append();
            }
            if let Some(err) = &self.append_error {
                harmony::body(ui, err, harmony::DANGER);
            } else if self.append_running {
                ui.horizontal(|ui| {
                    ui.spinner();
                    harmony::body(ui, &self.append_status.clone(), harmony::ACCENT);
                });
            } else if !self.append_status.is_empty() {
                harmony::muted(ui, &self.append_status.clone());
            }
        });

        harmony::section_frame().show(ui, |ui| {
            harmony::fill_width(ui);
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_files);
            harmony::heading(ui, t.heading_input_file);
            harmony::muted(ui, t.hint_input_formats);
            if harmony::path_browse_row(
                ui,
                &mut self.input_path,
                t.hint_input_placeholder,
                t.browse,
            ) {
                self.pick_input_file();
            }
        });

        harmony::section_frame().show(ui, |ui| {
            harmony::fill_width(ui);
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_batch);
            harmony::heading(ui, t.section_threads);
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut self.threads, 1..=512).logarithmic(true));
                egui::Frame::none()
                    .fill(harmony::INPUT)
                    .stroke(egui::Stroke::new(1.0_f32, harmony::BORDER))
                    .rounding(egui::Rounding::same(harmony::ROUND))
                    .inner_margin(egui::vec2(10.0, 6.0))
                    .show(ui, |ui| {
                        ui.set_min_size(egui::vec2(harmony::ACTION_W * 0.5, harmony::CTRL_H - 12.0));
                        ui.label(
                            egui::RichText::new(format!("{}", self.threads))
                                .size(harmony::FONT)
                                .color(harmony::ACCENT_CYAN),
                        );
                    });
            });
        });
    }

    fn ui_lookup_right(&mut self, ui: &mut egui::Ui, snap: &Progress) {
        let t = self.tr();
        harmony::section_frame().show(ui, |ui| {
            harmony::fill_width(ui);
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_progress);
            harmony::heading(ui, t.heading_progress);

            if let Some(err) = &self.error {
                harmony::body(ui, err, harmony::DANGER);
                harmony::hash_db_path_row(
                    ui,
                    t.label_hash_db,
                    &self.lmdb_path_display(),
                    t.tip_hash_db_path,
                );
            }

            if self.running {
                let pct = if snap.total > 0 {
                    snap.processed as f32 / snap.total as f32
                } else {
                    0.0
                };
                ui.add(
                    egui::ProgressBar::new(pct)
                        .fill(harmony::BORDER_STRONG)
                        .desired_height(6.0)
                        .animate(true),
                );
                harmony::body(
                    ui,
                    &format!(
                        "{}/{} · good {} · nohash {} · bad {} · trash {} · {:.0}s",
                        snap.processed,
                        snap.total,
                        snap.found,
                        snap.nohash,
                        snap.bad,
                        snap.trash,
                        snap.elapsed_ms as f64 / 1000.0
                    ),
                    harmony::SECONDARY,
                );
            } else if snap.done && !snap.good_path.is_empty() {
                harmony::body(
                    ui,
                    &i18n::lookup_done_short(
                        self.lang,
                        snap.found,
                        snap.nohash,
                        snap.bad,
                        snap.trash,
                    ),
                    harmony::SUCCESS,
                );
                if !snap.good_path.is_empty() {
                    harmony::muted(ui, &snap.good_path);
                }
                if !snap.nohash_path.is_empty() {
                    harmony::muted(ui, &snap.nohash_path);
                }
                if !snap.trash_path.is_empty() {
                    harmony::muted(ui, &snap.trash_path);
                }
            } else if !self.status.is_empty() && self.error.is_none() {
                harmony::muted(ui, &self.status.clone());
            } else {
                harmony::muted(ui, t.status_waiting_start);
            }

            if self.running || snap.done {
                ui.add_space(8.0);
                ui.columns(4, |cols| {
                    harmony::stat_tile(
                        &mut cols[0],
                        t.stat_total,
                        &format!("{}", snap.total),
                        harmony::TEXT,
                    );
                    harmony::stat_tile(
                        &mut cols[1],
                        t.stat_found,
                        &format!("{}", snap.found),
                        harmony::SUCCESS,
                    );
                    harmony::stat_tile(
                        &mut cols[2],
                        t.stat_not_found,
                        &format!("{}", snap.nohash),
                        harmony::WARNING,
                    );
                    harmony::stat_tile(
                        &mut cols[3],
                        t.stat_trash,
                        &format!("{}", snap.bad + snap.trash),
                        harmony::DANGER,
                    );
                });
            }

            harmony::muted(ui, t.status_progress_after_start);
        });
    }

    fn ui_merge(&mut self, ui: &mut egui::Ui) {
        let t = self.tr();
        harmony::instruction_button(ui, &mut self.instruction_open, t.btn_instruction);

        harmony::section_frame().show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_merge);
            harmony::heading(ui, t.merge_heading);
            harmony::muted(ui, t.merge_subtitle);
        });

        harmony::section_frame().show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_mail);
            harmony::heading(ui, t.heading_email_hash);
            if harmony::path_browse_row(
                ui,
                &mut self.merge_mail_path,
                t.hint_merge_mail,
                t.browse,
            ) {
                self.pick_merge_mail();
            }
        });

        harmony::section_frame().show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_dehash);
            harmony::heading(ui, t.heading_mail_plain);
            if harmony::path_browse_row(
                ui,
                &mut self.merge_dehash_path,
                t.hint_merge_dehash,
                t.browse,
            ) {
                self.pick_merge_dehash();
            }
        });

        harmony::section_frame().show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_output);
            harmony::heading(ui, t.heading_result);

            if let Some(err) = &self.merge_error {
                harmony::body(ui, err, harmony::DANGER);
            }

            if self.merge_running {
                ui.horizontal(|ui| {
                    ui.spinner();
                    harmony::body(ui, &self.merge_status.clone(), harmony::ACCENT);
                });
            } else if !self.merge_status.is_empty() {
                harmony::muted(ui, &self.merge_status.clone());
            } else {
                harmony::muted(ui, t.hint_merge_output);
            }
        });
        harmony::instruction_modal(ui, &mut self.instruction_open, t.instruction, t.btn_close, |ui| {
            harmony::instruction_body(ui, t.merge_instr_body);
            ui.add_space(6.0);
            harmony::instruction_heading(ui, t.merge_instr_formats);
            harmony::instruction_mono(ui, t.merge_instr_formats_mono);
            ui.add_space(4.0);
            harmony::instruction_heading(ui, t.merge_instr_outputs);
            harmony::instruction_mono(ui, t.merge_instr_outputs_mono);
        });
    }

    fn ui_batch_live_sql(&self, ui: &mut egui::Ui, live: &BatchLiveProgress) {
        ui.horizontal(|ui| {
            ui.spinner();
            ui.vertical(|ui| {
                for line in live.sql_status_lines() {
                    harmony::body(ui, &line, harmony::ACCENT);
                }
            });
        });
    }

    fn ui_batch_live_columns(&self, ui: &mut egui::Ui, live: &BatchLiveProgress) {
        ui.horizontal(|ui| {
            ui.spinner();
            ui.vertical(|ui| {
                for line in live.columns_status_lines() {
                    harmony::body(ui, &line, harmony::ACCENT);
                }
            });
        });
    }

    fn ui_extract_sql(&mut self, ui: &mut egui::Ui) {
        let t = self.tr();
        harmony::instruction_button(ui, &mut self.instruction_open, t.btn_instruction);

        harmony::section_frame().show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_sql);
            harmony::heading(ui, t.sql_heading);
            harmony::muted(ui, t.sql_subtitle);
        });

        harmony::section_frame().show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_source);
            harmony::heading(ui, t.heading_file_or_folder);
            let (file, folder) = harmony::path_file_folder_row(
                ui,
                &mut self.sql_path,
                t.hint_sql_source,
                t.file_btn,
                t.folder_btn,
                t.folder_hint_dumps,
            );
            if file {
                self.pick_sql_file();
            }
            if folder {
                self.pick_sql_folder();
            }
        });

        harmony::section_frame().show(ui, |ui| {
            harmony::fill_width(ui);
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_threads);
            harmony::heading(ui, t.heading_parallelism);
            harmony::muted(ui, t.hint_sql_threads);
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut self.sql_threads, 1..=5));
                egui::Frame::none()
                    .fill(harmony::INPUT)
                    .stroke(egui::Stroke::new(1.0_f32, harmony::BORDER))
                    .rounding(egui::Rounding::same(harmony::ROUND))
                    .inner_margin(egui::vec2(10.0, 6.0))
                    .show(ui, |ui| {
                        ui.set_min_size(egui::vec2(harmony::ACTION_W * 0.5, harmony::CTRL_H - 12.0));
                        ui.label(
                            egui::RichText::new(format!("{}", self.sql_threads))
                                .size(harmony::FONT)
                                .color(harmony::ACCENT),
                        );
                    });
            });
        });

        harmony::section_frame().show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_output);
            harmony::heading(ui, t.heading_result);

            if let Some(err) = &self.sql_error {
                harmony::body(ui, err, harmony::DANGER);
            }

            if self.sql_running {
                if self.sql_batch {
                    let live = self.sql_live.lock().unwrap().clone();
                    self.ui_batch_live_sql(ui, &live);
                } else {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        harmony::body(ui, &self.sql_status.clone(), harmony::ACCENT);
                    });
                }
            } else if !self.sql_status.is_empty() {
                harmony::muted(ui, &self.sql_status.clone());
            } else {
                harmony::muted(ui, t.hint_sql_output);
            }
        });
        harmony::instruction_modal(ui, &mut self.instruction_open, t.instruction, t.btn_close, |ui| {
            harmony::instruction_body(ui, t.sql_instr_body);
            ui.add_space(6.0);
            harmony::instruction_heading(ui, t.sql_instr_extracted);
            harmony::instruction_mono(ui, t.sql_instr_extracted_mono);
            ui.add_space(4.0);
            harmony::instruction_heading(ui, t.sql_instr_outputs);
            harmony::instruction_mono(ui, t.sql_instr_outputs_mono);
        });
    }

    fn ui_sql_columns(&mut self, ui: &mut egui::Ui) {
        let t = self.tr();
        harmony::instruction_button(ui, &mut self.instruction_open, t.btn_instruction);
        harmony::section_frame().show(ui, |ui| {
            harmony::section_title(ui, t.section_sql_columns);
            harmony::heading(ui, t.heading_loginpass);
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new(t.sqlcol_intro)
                    .size(12.0)
                    .color(harmony::MUTED),
            );
        });

        harmony::section_frame().show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_source);
            let (file, folder) = harmony::path_file_folder_row(
                ui,
                &mut self.sqlcol_path,
                t.hint_sql_source,
                t.file_btn,
                t.folder_btn,
                t.folder_hint_dumps,
            );
            if file {
                self.pick_sqlcol_file();
            }
            if folder {
                self.pick_sqlcol_folder();
            }
        });

        harmony::section_frame().show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_output);
            if let Some(err) = &self.sqlcol_error {
                harmony::body(ui, err, harmony::DANGER);
            }
            if self.sqlcol_running {
                if self.sqlcol_batch {
                    let live = self.sqlcol_live.lock().unwrap().clone();
                    self.ui_batch_live_columns(ui, &live);
                } else {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        harmony::body(ui, &self.sqlcol_status.clone(), harmony::ACCENT);
                    });
                }
            } else if !self.sqlcol_status.is_empty() {
                harmony::muted(ui, &self.sqlcol_status.clone());
            } else {
                harmony::muted(ui, t.hint_sqlcol_output);
            }
        });
        harmony::instruction_modal(ui, &mut self.instruction_open, t.instruction, t.btn_close, |ui| {
            harmony::instruction_body(ui, t.sqlcol_instr_body);
            ui.add_space(6.0);
            harmony::instruction_heading(ui, t.sqlcol_login_cols);
            harmony::instruction_mono(ui, t.sqlcol_login_cols_mono);
            ui.add_space(4.0);
            harmony::instruction_heading(ui, t.sqlcol_pass_cols);
            harmony::instruction_mono(ui, t.sqlcol_pass_cols_mono);
            ui.add_space(4.0);
            harmony::instruction_heading(ui, t.sqlcol_example);
            harmony::instruction_mono(ui, t.sqlcol_example_mono);
        });
    }

    fn ui_custom_regex(&mut self, ui: &mut egui::Ui) {
        let t = self.tr();
        harmony::instruction_button(ui, &mut self.instruction_open, t.btn_instruction);
        harmony::section_frame().show(ui, |ui| {
            harmony::section_title(ui, t.section_regex_engine);
            harmony::heading(ui, t.heading_custom_regex);
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new(t.regex_intro)
                    .size(12.0)
                    .color(harmony::MUTED),
            );
        });

        harmony::section_frame().show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_preset);
            ui.horizontal(|ui| {
                ui.set_min_height(harmony::CTRL_H);
                ui.label(
                    egui::RichText::new(t.hint_regex_template_label)
                        .size(harmony::FONT)
                        .color(harmony::SECONDARY),
                );
                let prev = self.regex_preset_idx;
                egui::ComboBox::from_id_salt("regex_preset")
                    .height(harmony::CTRL_H)
                    .selected_text(
                        PRESETS
                            .get(self.regex_preset_idx)
                            .map(|p| p.name)
                            .unwrap_or("custom"),
                    )
                    .show_ui(ui, |ui| {
                        for (i, p) in PRESETS.iter().enumerate() {
                            if ui.selectable_label(self.regex_preset_idx == i, p.name).clicked() {
                                self.apply_regex_preset(i);
                            }
                        }
                    });
                if self.regex_preset_idx != prev {
                    self.apply_regex_preset(self.regex_preset_idx);
                }
            });
        });

        harmony::section_frame().show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_source);
            if harmony::path_browse_row(
                ui,
                &mut self.regex_source_path,
                "any.txt / .sql / .log",
                t.browse,
            ) {
                self.pick_regex_source();
            }
        });

        harmony::section_frame().show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, "Regex");
            harmony::muted(ui, t.regex_pattern_label);
            ui.add(
                egui::TextEdit::multiline(&mut self.regex_pattern)
                    .desired_width(f32::INFINITY)
                    .desired_rows(3)
                    .font(egui::TextStyle::Monospace)
                    .margin(egui::vec2(8.0, 6.0)),
            );
            harmony::muted(ui, t.regex_output_template_label);
            ui.add(
                harmony::path_edit(&mut self.regex_template, "$1:$2", ui.available_width())
                    .font(egui::TextStyle::Monospace),
            );
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.regex_case_insensitive, t.regex_flag_i);
                ui.checkbox(&mut self.regex_multiline, t.regex_flag_m);
                ui.checkbox(&mut self.regex_dotall, t.regex_flag_s);
                ui.checkbox(&mut self.regex_dedupe, t.regex_flag_dedupe);
            });
        });

        harmony::section_frame().show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_output);
            if let Some(err) = &self.regex_error {
                harmony::body(ui, err, harmony::DANGER);
            }
            if self.regex_running {
                ui.horizontal(|ui| {
                    ui.spinner();
                    harmony::body(ui, &self.regex_status.clone(), harmony::ACCENT);
                });
            } else if !self.regex_status.is_empty() {
                harmony::muted(ui, &self.regex_status.clone());
            } else {
                harmony::muted(ui, t.hint_regex_output);
            }
        });
        harmony::instruction_modal(ui, &mut self.instruction_open, t.instruction, t.btn_close, |ui| {
            harmony::instruction_body(ui, t.regex_instr_body);
            ui.add_space(8.0);
            harmony::instruction_heading(ui, t.regex_template_heading);
            harmony::instruction_mono(ui, t.regex_template_mono);
            ui.add_space(6.0);
            harmony::instruction_heading(ui, t.regex_flags_heading);
            harmony::instruction_mono(ui, t.regex_flags_mono);
            ui.add_space(6.0);
            harmony::instruction_heading(ui, t.regex_examples_heading);
            harmony::instruction_mono(ui, t.regex_examples_mono);
            ui.add_space(6.0);
            harmony::instruction_heading(ui, t.regex_cli_heading);
            harmony::instruction_mono(ui, t.regex_cli_mono);
        });
    }

    fn ui_combo(&mut self, ui: &mut egui::Ui) {
        let t = self.tr();
        harmony::fill_width(ui);
        let content_w = ui.available_width();
        harmony::instruction_button(ui, &mut self.instruction_open, t.btn_instruction);

        harmony::section_frame().show(ui, |ui| {
            harmony::fill_width(ui);
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_combo);
            harmony::heading(ui, t.heading_combo);
            harmony::muted(ui, t.combo_subtitle);
            ui.add_space(4.0);
            harmony::muted(ui, t.label_combo_tool);
            harmony::fill_width(ui);
            egui::ComboBox::from_id_salt("combo_tool")
                .selected_text(self.combo_tool.label(self.lang))
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    for tool in ComboTool::ALL {
                        ui.selectable_value(
                            &mut self.combo_tool,
                            tool,
                            tool.label(self.lang),
                        );
                    }
                });
        });

        harmony::section_frame().show(ui, |ui| {
            harmony::fill_width(ui);
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_source);
            harmony::heading(ui, t.heading_file_or_folder);
            if harmony::path_browse_row(ui, &mut self.combo_input, t.hint_combo_input, t.browse) {
                self.pick_combo_input();
            }
            if matches!(self.combo_tool, ComboTool::Compare) {
                ui.add_space(4.0);
                harmony::heading(ui, t.hint_combo_input_b);
                if harmony::path_browse_row(
                    ui,
                    &mut self.combo_input_b,
                    t.hint_combo_input_b,
                    t.browse,
                ) {
                    self.pick_combo_input_b();
                }
            }
            if matches!(self.combo_tool, ComboTool::LineFilter) {
                ui.add_space(4.0);
                harmony::fill_width(ui);
                ui.label(
                    egui::RichText::new(t.hint_combo_filter)
                        .size(harmony::FONT_SMALL)
                        .color(harmony::SECONDARY),
                );
                ui.horizontal(|ui| {
                    ui.set_max_width(content_w);
                    ui.spacing_mut().item_spacing.x = harmony::GAP;
                    let regex_w = 56.0;
                    let field_w = (ui.available_width() - regex_w - harmony::GAP).max(80.0);
                    ui.add(
                        egui::TextEdit::singleline(&mut self.combo_filter)
                            .hint_text("contains…")
                            .desired_width(field_w)
                            .margin(egui::vec2(8.0, 6.0))
                            .min_size(egui::vec2(80.0, harmony::CTRL_H)),
                    );
                    ui.checkbox(&mut self.combo_use_regex, "regex");
                });
            }
            if matches!(self.combo_tool, ComboTool::LineSplit) {
                ui.add_space(4.0);
                harmony::fill_width(ui);
                ui.horizontal(|ui| {
                    ui.set_max_width(content_w);
                    ui.spacing_mut().item_spacing.x = harmony::GAP;
                    ui.label(
                        egui::RichText::new(t.hint_combo_lines)
                            .size(harmony::FONT_SMALL)
                            .color(harmony::SECONDARY),
                    );
                    ui.add(
                        egui::DragValue::new(&mut self.combo_lines_per_file)
                            .range(1_000..=10_000_000)
                            .speed(1_000),
                    );
                });
            }
        });

        harmony::section_frame().show(ui, |ui| {
            harmony::fill_width(ui);
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_output);
            harmony::heading(ui, t.heading_result);
            if matches!(
                self.combo_tool,
                ComboTool::Compare
                    | ComboTool::MxCheck
                    | ComboTool::Analyze
                    | ComboTool::SplitNamePass
            ) {
                if harmony::path_browse_row(
                    ui,
                    &mut self.combo_output_dir,
                    t.hint_combo_output_dir,
                    t.folder_btn,
                ) {
                    self.pick_combo_output_dir();
                }
            } else if harmony::path_browse_row(
                ui,
                &mut self.combo_output,
                t.hint_combo_output,
                t.browse,
            ) {
                self.pick_combo_output();
            }
            if let Some(err) = &self.combo_error {
                harmony::body(ui, err, harmony::DANGER);
            }
            if self.combo_running {
                ui.horizontal(|ui| {
                    ui.spinner();
                    harmony::body(ui, &self.combo_status, harmony::ACCENT);
                });
            } else if !self.combo_status.is_empty() {
                harmony::muted(ui, &self.combo_status);
            } else {
                harmony::muted(ui, t.hint_combo_output_idle);
            }
        });
        harmony::instruction_modal(ui, &mut self.instruction_open, t.instruction, t.btn_close, |ui| {
            harmony::instruction_body(ui, t.combo_instr_body);
            ui.add_space(6.0);
            harmony::instruction_heading(ui, t.combo_instr_tools);
            harmony::instruction_mono(ui, t.combo_instr_tools_mono);
            ui.add_space(4.0);
            harmony::instruction_heading(ui, t.combo_instr_formats);
            harmony::instruction_mono(ui, t.combo_instr_formats_mono);
            ui.add_space(4.0);
            harmony::instruction_heading(ui, t.combo_instr_outputs);
            harmony::instruction_mono(ui, t.combo_instr_outputs_mono);
            ui.add_space(4.0);
            harmony::instruction_heading(ui, t.combo_instr_controls);
            harmony::instruction_body(ui, t.combo_instr_controls_body);
        });
    }

    fn ui_ulp(&mut self, ui: &mut egui::Ui) {
        let t = self.tr();
        harmony::instruction_button(ui, &mut self.instruction_open, t.btn_instruction);

        harmony::section_frame().show(ui, |ui| {
            harmony::fill_width(ui);
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_ulp);
            harmony::heading(ui, t.heading_ulp);
            harmony::muted(ui, t.ulp_subtitle);
            ui.add_space(4.0);
            harmony::muted(ui, t.label_ulp_tool);
            harmony::fill_width(ui);
            egui::ComboBox::from_id_salt("ulp_tool")
                .selected_text(self.ulp_tool.label(self.lang))
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    for tool in UlpTool::ALL {
                        ui.selectable_value(&mut self.ulp_tool, tool, tool.label(self.lang));
                    }
                });
        });

        harmony::section_frame().show(ui, |ui| {
            harmony::fill_width(ui);
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_source);
            harmony::heading(ui, t.heading_file_or_folder);
            if harmony::path_browse_row(ui, &mut self.ulp_input, t.hint_ulp_input, t.browse) {
                self.pick_ulp_input();
            }
            ui.add_space(4.0);
            harmony::fill_width(ui);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = harmony::GAP;
                if ui.add(harmony::secondary_button(t.folder_btn)).clicked() {
                    self.pick_ulp_input_folder();
                }
                harmony::muted(ui, t.folder_hint_dumps);
            });
            if self.ulp_tool.needs_keywords() {
                ui.add_space(4.0);
                harmony::fill_width(ui);
                ui.label(
                    egui::RichText::new(t.hint_ulp_keywords)
                        .size(harmony::FONT_SMALL)
                        .color(harmony::SECONDARY),
                );
                ui.add(
                    egui::TextEdit::singleline(&mut self.ulp_keywords)
                        .hint_text("gmail.com, paypal")
                        .desired_width(ui.available_width())
                        .margin(egui::vec2(8.0, 6.0))
                        .min_size(egui::vec2(80.0, harmony::CTRL_H)),
                );
            }
        });

        harmony::section_frame().show(ui, |ui| {
            harmony::fill_width(ui);
            ui.spacing_mut().item_spacing = egui::vec2(harmony::GAP, harmony::GAP);
            harmony::section_title(ui, t.section_output);
            harmony::heading(ui, t.heading_result);
            if self.ulp_tool.needs_output_dir() {
                if harmony::path_browse_row(
                    ui,
                    &mut self.ulp_output_dir,
                    t.hint_ulp_output_dir,
                    t.folder_btn,
                ) {
                    self.pick_ulp_output_dir();
                }
            } else if harmony::path_browse_row(
                ui,
                &mut self.ulp_output,
                t.hint_ulp_output,
                t.browse,
            ) {
                self.pick_ulp_output();
            }
            if let Some(err) = &self.ulp_error {
                harmony::body(ui, err, harmony::DANGER);
            }
            if self.ulp_running {
                ui.horizontal(|ui| {
                    ui.spinner();
                    harmony::body(ui, &self.ulp_status, harmony::ACCENT);
                });
            } else if !self.ulp_status.is_empty() {
                harmony::muted(ui, &self.ulp_status);
            } else {
                harmony::muted(ui, t.hint_ulp_output_idle);
            }
        });
        harmony::instruction_modal(ui, &mut self.instruction_open, t.instruction, t.btn_close, |ui| {
            harmony::instruction_body(ui, t.ulp_instr_body);
            ui.add_space(6.0);
            harmony::instruction_heading(ui, t.ulp_instr_tools);
            harmony::instruction_mono(ui, t.ulp_instr_tools_mono);
            ui.add_space(4.0);
            harmony::instruction_heading(ui, t.ulp_instr_outputs);
            harmony::instruction_mono(ui, t.ulp_instr_outputs_mono);
            ui.add_space(4.0);
            harmony::instruction_heading(ui, t.ulp_instr_controls);
            harmony::instruction_body(ui, t.ulp_instr_controls_body);
        });
    }
}

pub fn run(db: Arc<HashDb>) -> Result<()> {
    let size = [harmony::WINDOW_W, harmony::WINDOW_H];
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size(size)
        .with_min_inner_size(size)
        .with_max_inner_size(size)
        .with_resizable(false)
        .with_decorations(false)
        .with_transparent(true)
        .with_title("LocalHashFinder");
    if let Some(icon) = window_icon() {
        viewport = viewport.with_icon(std::sync::Arc::new(icon));
    }
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        "LocalHashFinder",
        options,
        Box::new(move |cc| {
            harmony::apply(&cc.egui_ctx);
            apply_native_window_shell(&*cc);
            Ok(Box::new(App::new(db)))
        }),
    )
    .map_err(|e| anyhow::anyhow!(e.to_string()))
}
