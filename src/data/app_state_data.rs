use std::sync;
use druid::im;
use druid::ImageBuf;

use crate::helper::layout_settings::LayoutSettings;

use super::plugin_item_data::PluginItemData;

#[derive(Clone, druid::Data, druid::Lens)]
pub struct AppState {
    pub text_edit: sync::Arc<String>,
    pub timeline_data: im::Vector<MessageGroup>,
    pub profile_pics: im::Vector<ImageBuf>,
    pub layout_settings: LayoutSettings,
    pub settings_open: bool,
    pub plugin_load_status: String,
    pub plugin_load_dir: Option<String>,
    pub plugin_list: im::Vector<PluginItemData>,
}

#[derive(Clone, druid::Data, druid::Lens)]
pub struct MessageGroup {
    pub user_id: u32,
    pub profile_pic: ImageBuf,
    pub messages: im::Vector<Message>,
}

#[derive(Clone, druid::Data)]
pub struct Message {
    pub message: String,
    pub position_in_group: u32,
    pub timestamp_epoch_seconds: i64,
}