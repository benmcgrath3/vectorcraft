//! Selection colours in Properties: each colour used by the selected subtrees, with an exact
//! replacement through Recolor Artwork. Shared patterns are copied by the engine, so artwork
//! outside the selection keeps its colours. Image pixels are not part of this palette.

use egui::{Ui, vec2};
use serde_json::json;
use std::sync::Arc;
use vectorcraft_color::{Color, Paint};
use vectorcraft_engine::cmd::color_value;

use super::{pstate, set_pstate};
use crate::theme::Tokens;
use crate::{VectorcraftApp, widgets};

#[derive(Clone, Debug)]
pub(super) struct UsedColor {
    key: String,
    color: Color,
    count: u64,
}

/// Query once per document revision (including selection changes and undo), shared with the
/// Recolor quick action. The document uid keeps palettes separate when switching tabs.
pub(super) fn colors(app: &mut VectorcraftApp, ctx: &egui::Context) -> Arc<Vec<UsedColor>> {
    let Some(st) = app.session.active().filter(|st| !st.selection.is_empty()) else { return Arc::new(vec![]) };
    let key = (st.uid, st.revision);
    if let Some((cached, colors)) = pstate::<Option<((u64, u64), Arc<Vec<UsedColor>>)>>(ctx, "selection-colors")
        && cached == key
    {
        return colors;
    }
    let colors = match app.session.execute("recolor.colors", &json!({})) {
        Ok(r) => r
            .get("colors")
            .and_then(|v| v.as_array())
            .into_iter()
            .flatten()
            .filter_map(|v| {
                let key = v.get("key")?.as_str()?.to_string();
                Some(UsedColor { color: color_value(&json!(key))?, key, count: v.get("count")?.as_u64()? })
            })
            .collect::<Vec<_>>(),
        Err(e) => {
            app.status(e.to_string());
            vec![]
        }
    };
    let colors = Arc::new(colors);
    set_pstate(ctx, "selection-colors", Some((key, colors.clone())));
    colors
}

pub(super) fn show(app: &mut VectorcraftApp, ui: &mut Ui) {
    let colors = colors(app, ui.ctx());
    if colors.is_empty() {
        return;
    }
    widgets::divider(ui);
    widgets::section_header(ui, tl!("Selection Colors"));
    let t = Tokens::get(ui.ctx());
    let uid = app.session.active().map_or(0, |st| st.uid);
    egui::ScrollArea::vertical().id_salt("selection-colors-list").max_height(174.0).show_rows(ui, 26.0, colors.len(), |ui, rows| {
        // Only lay out visible rows, including when imported artwork has thousands of colours.
        for used in colors.get(rows).unwrap_or_default() {
            ui.push_id((uid, &used.key), |ui| {
                ui.horizontal(|ui| {
                    let response = ui.add_sized(vec2(26.0, 26.0), egui::Button::new(""));
                    widgets::paint_chip(ui, response.rect.shrink(3.0), &Paint::solid(used.color));
                    response.widget_info(|| {
                        egui::WidgetInfo::labeled(egui::WidgetType::Button, true, format!("{}: {}", tl!("Edit selection color"), used.key))
                    });
                    if response.on_hover_text(tl!("Edit selection color")).clicked() {
                        // app.run reports validation failures in the app's status message.
                        app.run("ui.colorPicker", json!({"replace": used.key})).ok();
                    }
                    if let Some(hex) = widgets::text_field(ui, "hex", Some(&used.color.to_hex().to_uppercase()), 86.0, 1) {
                        replace_hex(app, &used.key, &hex);
                    }
                    widgets::dim_label(ui, tl!(used.color.model_name()));
                    ui.label(egui::RichText::new(format!("×{}", used.count)).size(11.5).color(t.text_dim))
                        .on_hover_text(tl!("Color uses in the selection"));
                });
            });
        }
    });
}

fn replace_hex(app: &mut VectorcraftApp, key: &str, hex: &str) {
    match super::color::parse_hex(hex) {
        Some(color) => {
            // Exact keys keep different colour models separate, even when their hex looks alike.
            app.run("recolor.apply", json!({"map": [{"from": [key], "to": super::color_json(&color)}], "includeImages": false})).ok();
        }
        None => app.status(tl!("Invalid hex color").to_string()),
    }
}

#[cfg(test)]
mod tests {
    use egui::{Event, Key, Modifiers, PointerButton, Pos2, Rect};
    use vectorcraft_doc::NodeId;
    use vectorcraft_engine::Session;

    use super::*;

    fn app() -> VectorcraftApp {
        let mut app = VectorcraftApp::new(Session::new(), Default::default());
        app.run("file.new", json!({"width": 400, "height": 200})).unwrap();
        app
    }

    fn rect(app: &mut VectorcraftApp, color: &str) -> u64 {
        let id = app.run("shape.rectangle", json!({"x": 10, "y": 10, "width": 40, "height": 40})).unwrap()["id"].as_u64().unwrap();
        app.run("paint.setFill", json!({"color": color})).unwrap();
        app.run("paint.setStroke", json!({"none": true})).unwrap();
        id
    }

    fn frame(app: &mut VectorcraftApp, ctx: &egui::Context, events: Vec<Event>) -> Vec<(String, Rect)> {
        let raw = egui::RawInput { screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(270.0, 300.0))), events, ..Default::default() };
        let mut out = ctx.run_ui(raw, |ui| show(app, ui));
        out.textures_delta.clear();
        out.shapes
            .iter()
            .filter_map(|s| match &s.shape {
                egui::Shape::Text(t) => Some((t.galley.text().to_string(), Rect::from_min_size(t.pos, t.galley.size()))),
                _ => None,
            })
            .collect()
    }

    fn pointer(at: Pos2, pressed: bool) -> Event {
        Event::PointerButton { pos: at, button: PointerButton::Primary, pressed, modifiers: Default::default() }
    }

    #[test]
    fn palette_tracks_selection_edits_undo_and_document_switching() {
        let mut app = app();
        let black = rect(&mut app, "#000000");
        let red = rect(&mut app, "#ff0000");
        let ctx = egui::Context::default();
        assert_eq!(colors(&mut app, &ctx)[0].color.to_hex(), "#ff0000");
        app.run("select.set", json!({"ids": [black, red]})).unwrap();
        assert_eq!(colors(&mut app, &ctx).len(), 2);
        replace_hex(&mut app, "rgb 0 0 0", "3366cc");
        assert!(colors(&mut app, &ctx).iter().any(|c| c.key == "rgb 51 102 204"));
        app.run("edit.undo", json!({})).unwrap();
        assert!(colors(&mut app, &ctx).iter().any(|c| c.color == Color::BLACK));
        app.run("file.new", json!({"width": 400, "height": 200})).unwrap();
        assert!(colors(&mut app, &ctx).is_empty());
        rect(&mut app, "#00ff00");
        assert_eq!(colors(&mut app, &ctx)[0].color.to_hex(), "#00ff00");
        app.run("select.none", json!({})).unwrap();
        assert!(frame(&mut app, &ctx, vec![]).is_empty());
    }

    #[test]
    fn selection_palette_has_editable_hex_rows_and_picker_chips_in_every_theme() {
        for theme in crate::theme::Brightness::ALL {
            let mut app = app();
            let black = rect(&mut app, "#000000");
            let another = rect(&mut app, "#000000");
            let outside = rect(&mut app, "#000000");
            app.run("select.set", json!({"ids": [black, another]})).unwrap();
            let before = app.session.doc().unwrap().history.undo.len();
            let ctx = egui::Context::default();
            crate::theme::install_fonts(&ctx);
            crate::theme::apply(&ctx, theme);
            frame(&mut app, &ctx, vec![]);
            let texts = frame(&mut app, &ctx, vec![]);
            assert!(texts.iter().any(|(t, _)| t == "Selection Colors"), "{theme:?}: {texts:?}");
            assert!(texts.iter().any(|(t, _)| t == "×2"), "{texts:?}");
            let hex = texts.iter().find(|(t, _)| t == "#000000").unwrap().1;
            let at = hex.center();
            frame(&mut app, &ctx, vec![Event::PointerMoved(at), pointer(at, true)]);
            frame(&mut app, &ctx, vec![pointer(at, false)]);
            let key = |key, modifiers| Event::Key { key, physical_key: None, pressed: true, repeat: false, modifiers };
            frame(
                &mut app,
                &ctx,
                vec![key(Key::A, Modifiers { ctrl: true, command: true, mac_cmd: true, ..Default::default() }), Event::Text("3366CC".into())],
            );
            frame(&mut app, &ctx, vec![key(Key::Enter, Default::default())]);
            frame(&mut app, &ctx, vec![]);
            let fill = |id| app.session.doc().unwrap().doc.node(NodeId(id)).unwrap().appearance.fill_paint().color().unwrap().to_hex();
            assert_eq!(fill(black), "#3366cc", "{theme:?}");
            assert_eq!(fill(another), "#3366cc");
            assert_eq!(fill(outside), "#000000");
            assert_eq!(app.session.doc().unwrap().history.undo.len(), before + 1);
            let texts = frame(&mut app, &ctx, vec![]);
            let hex = texts.iter().find(|(t, _)| t == "#3366CC").unwrap().1;
            // The chip sits immediately before the hex field.
            let at = Pos2::new(hex.left() - 24.0, hex.center().y);
            frame(&mut app, &ctx, vec![Event::PointerMoved(at), pointer(at, true)]);
            frame(&mut app, &ctx, vec![pointer(at, false)]);
            assert_eq!(app.ui.dialog.as_ref().unwrap().fields.get("__replace"), Some(&json!("rgb 51 102 204")));
        }
    }

    #[test]
    fn invalid_hex_does_not_change_art_and_lookalike_models_stay_separate() {
        let mut app = app();
        let rgb = rect(&mut app, "#ff0000");
        let cmyk = rect(&mut app, "#ff0000");
        app.run("paint.setFill", json!({"color": {"c": 0, "m": 100, "y": 100, "k": 0}})).unwrap();
        let lookalike = app.session.doc().unwrap().doc.node(NodeId(cmyk)).unwrap().appearance.fill_paint().color().unwrap().to_hex();
        app.run("paint.setFill", json!({"ids": [rgb], "color": lookalike})).unwrap();
        app.run("select.set", json!({"ids": [rgb, cmyk]})).unwrap();
        let ctx = egui::Context::default();
        let palette = colors(&mut app, &ctx);
        assert_eq!(palette.len(), 2);
        assert_eq!(palette[0].color.to_hex(), palette[1].color.to_hex());
        let before = app.session.doc().unwrap().doc.clone();
        replace_hex(&mut app, "cmyk 0 100 100 0", "not a color");
        assert!(std::sync::Arc::ptr_eq(&before, &app.session.doc().unwrap().doc));
        replace_hex(&mut app, "cmyk 0 100 100 0", "00ff00");
        assert_eq!(app.session.doc().unwrap().doc.node(NodeId(rgb)).unwrap().appearance.fill_paint().color().unwrap().to_hex(), lookalike);
        assert!(matches!(app.session.doc().unwrap().doc.node(NodeId(cmyk)).unwrap().appearance.fill_paint().color(), Some(Color::Cmyk { .. })));
    }
}
