use druid::{Widget, WidgetExt, Color};
use druid::widget;
use crate::AppState;
use crate::data::plugin_item_data::PluginItemData;

pub(crate) fn build_plugin_ui() -> impl Widget<AppState> {
    widget::List::new(|| {
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
    }).lens(AppState::plugin_list)
}
