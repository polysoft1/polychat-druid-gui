use druid::{Widget, WidgetExt, Color};
use druid::widget;
use druid::widget::prelude::*;
use log::{error, warn};
use crate::AppState;
use crate::data::plugin_item_data::PluginItemData;

fn open_plugin_dir(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
    match data.plugin_load_dir.clone() {
        Some(dir) => {
            let result = opener::open(dir);
            if result.is_err() {
                error!("Failed to open directory in UI: {}", result.unwrap_err())
            }
        },
        None => {
            warn!("No plugin dir set from core.");
        }
    }
}

pub(crate) fn build_plugin_ui() -> impl Widget<AppState> {
    widget::Flex::column()
        .with_child(widget::List::new(|| {
            widget::Flex::row()
                .with_flex_child(
                    widget::Label::new(|data: &PluginItemData, _env: &_| data.plugin_name.clone())
                        .padding(4.0)
                        .center()
                , 1.0)
                .with_child(widget::Button::new("Add Account"))
            .background(Color::rgb8(40, 40, 40))
            .border(Color::GRAY, 1.0)
            .padding(1.0)
        }).lens(AppState::plugin_list))
        .with_child(
            widget::Flex::row()
                .with_default_spacer()
                .with_flex_child(
                    widget::Label::new(|data: &String, _env: &_| format!("Status: {}", data))
                    .lens(AppState::plugin_load_status),
                    1.0,
                )
                .with_flex_spacer(1.0)
                .with_child(
                    widget::Button::new("Open Dir")
                        .on_click(open_plugin_dir)
                )
                .main_axis_alignment(widget::MainAxisAlignment::SpaceBetween)
        ).cross_axis_alignment(widget::CrossAxisAlignment::Start)
}
