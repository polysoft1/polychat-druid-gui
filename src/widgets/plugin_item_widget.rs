use druid::kurbo::Rect;
use druid::{Widget, widget, WidgetPod};
use druid::widget::prelude::*;
use druid::Point;
use crate::{Message};
use druid::piet::Color;
use crate::LayoutSettings;
use crate::helper::helper_functions::{self, TimestampFormat};

/// A widget that shows a single message
/// 
/// It also handles timestamps, the settings menu, reactions, and more.
pub struct PluginItemWidget {
    msg_content_label: WidgetPod<Message, widget::Label<Message>>,
    timestamp_label: WidgetPod<Message, widget::Label<Message>>,
}