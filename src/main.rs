use druid::{AppLauncher, Color, WindowDesc, PlatformError, AppDelegate, WidgetExt};
use druid::widget::{Widget, Button, Flex, Label};
use druid::widget;
use druid::widget::prelude::*;
use druid::im;
use druid;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use log::warn;

use tokio;

// Polychat
use polychat_ipc;

use helper::layout_settings::LayoutSettings;

use data::app_state_data::*;

mod widgets;
mod helper;
mod data;
mod settings_ui;
mod chat_ui;
mod plugin_ui;
mod core_interface;

// Env keys to define layout in the environment
pub const ITEM_LAYOUT_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.item_layout");
pub const METADATA_LAYOUT_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.metadata_layout");
pub const PICTURE_SHAPE_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.picture_shape");
pub const PICTURE_SIZE_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.picture_size");
pub const CHAT_BUBBLE_TAIL_SHAPE_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.tail_shape");
pub const CHAT_BUBBLE_TAIL_SIZE_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.tail_size");
pub const CHAT_BUBBLE_RADIUS_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.bubble_radius");
pub const CHAT_BUBBLE_IMG_SPACING_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.bubble_img_spacing");
pub const SHOW_SELF_PROFILE_PIC_KEY: druid::env::Key<bool> = druid::env::Key::new("polysoft.druid-demo.show_self_pic");
pub const BUBBLE_PADDING_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.bubble_padding");
pub const METADATA_CONTENT_SPACING_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.metadata_content_padding");
pub const ALIGN_TO_PICTURE: druid::env::Key<bool> = druid::env::Key::new("polysoft.druid-demo.align-to-picture");
pub const GROUP_SPACING_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.group_spacing");
pub const SINGLE_MESSAGE_SPACING_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.single_message_spacing");
pub const SHOW_LEFT_LINE_KEY: druid::env::Key<bool> = druid::env::Key::new("polysoft.druid-demo.show_left_line");
pub const LEFT_SPACING_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.left_spacing");
pub const LEFT_BUBBLE_FLIPPED_KEY: druid::env::Key<bool> = druid::env::Key::new("polysoft.druid-demo.left_bubble_flipped");
pub const RIGHT_BUBBLE_FLIPPED_KEY: druid::env::Key<bool> = druid::env::Key::new("polysoft.druid-demo.right_bubble_flipped");
pub const SENDER_FONT_SIZE_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.sender_font_size");
pub const CONTENT_FONT_SIZE_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.content_font_size");
pub const DATETIME_FONT_SIZE_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.datetime_font_size");
pub const HEADER_FONT_BOLDED_KEY: druid::env::Key<bool> = druid::env::Key::new("polysoft.druid-demo.metadata_font_bolded");
pub const DATETIME_FORMAT_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.datetime_format");
pub const SIDE_TIME_FORMAT_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.side_time_format");
pub const SENDER_COLOR_KEY: druid::env::Key<druid::Color> = druid::env::Key::new("polysoft.druid-demo.sender_color");
pub const DATETIME_COLOR_KEY: druid::env::Key<druid::Color> = druid::env::Key::new("polysoft.druid-demo.datetime_color");
pub const SELF_DATETIME_COLOR_KEY: druid::env::Key<druid::Color> = druid::env::Key::new("polysoft.druid-demo.self_datetime_color");
pub const SELF_SENDER_COLOR_KEY: druid::env::Key<druid::Color> = druid::env::Key::new("polysoft.druid-demo.self_sender_color");
pub const LEFT_META_OFFSET_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.left_meta_offset");
pub const IRC_STACK_WIDTH_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.irc_stack_width");
pub const IRC_HEADER_WIDTH_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.irc_header_width");
// Commands to communicate things that need to happen
const REFRESH_UI_SELECTOR: druid::Selector = druid::Selector::new("polysoft.druid-demo.refresh_ui");

struct Delegate {
    window_count: i32,
}

impl AppDelegate<AppState> for Delegate {
    fn event(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _window_id: druid::WindowId,
        event: druid::Event,
        _data: &mut AppState,
        _env: &druid::Env,
    ) -> Option<druid::Event> {
        Some(event)
    }

    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        _cmd: &druid::Command,
        _data: &mut AppState,
        _env: &druid::Env,
    ) -> druid::Handled {
        druid::Handled::No
    }

    fn window_added(
        &mut self,
        _id: druid::WindowId,
        _handle: druid::WindowHandle,
        _data: &mut AppState,
        _env: &druid::Env,
        _ctx: &mut druid::DelegateCtx,
    ) {
        self.window_count += 1;
    }

    fn window_removed(&mut self, _id: druid::WindowId, data: &mut AppState, _env: &druid::Env, _ctx: &mut druid::DelegateCtx) {
        self.window_count -= 1;
        data.settings_open = false;
        if self.window_count <= 0 {
            println!("All windows closed. Quitting...");
            druid::Application::global().quit();
        }
    }
}

fn get_main_window_row() -> impl Widget<AppState> {
    Flex::row()
        .with_flex_child(
            Flex::column()
                .with_child(Label::new("Plugins").padding(5.0))
                .with_child(plugin_ui::build_plugin_ui())
                .expand_width()
        , 1.0)
        .with_flex_child(
            Flex::column()
                .with_child(Label::new("Accounts").padding(5.0))
                .expand_width()
        , 1.0)
        .with_flex_child(
            Flex::column()
                .with_child(Label::new("Conversations").padding(5.0))
                .with_child(Button::new("Open Chat Window").on_click( |ctx: &mut EventCtx, _data: &mut AppState, _ | {
                    ctx.new_window(WindowDesc::new(chat_ui::build_chat_ui())
                        .window_size((300.0, 450.0)));
                }))
                .expand_width()
        , 1.0)
    .cross_axis_alignment(widget::CrossAxisAlignment::Start)
    .expand_height()
    .background(Color::BLACK)
    
}

fn get_main_window_widget(tx: mpsc::Sender<bool>) -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new("PolyChat").padding(5.0))
        .with_flex_child(get_main_window_row(), 1.0)
    .cross_axis_alignment(widget::CrossAxisAlignment::Fill)
    .on_added( move |&mut _, _ctx: &mut LifeCycleCtx, _data: &AppState, _env: &Env| {
        notify_gui_ready(tx.clone());
    })
}


fn get_main_window_desc(tx: mpsc::Sender<bool>) -> WindowDesc<AppState> {
    let main_window = WindowDesc::new(
        get_main_window_widget(tx)
    ).window_size((900.0, 450.0));
    return main_window;
}

fn notify_gui_ready(tx: mpsc::Sender<bool>) {
    println!("Lifecycle event hit. Sending message.");
    let result = tx.send(true);
    if result.is_err() {
        warn!("Failed to send ready message from GUI. Err: {:?}", result.err())
    }
}

fn main() -> Result<(), PlatformError> {
    // create the initial app state
    let initial_state = AppState {
        text_edit: "".to_string().into(),
        timeline_data: im::vector![],
        profile_pics: im::vector![],
        settings_open: false,
        layout_settings: LayoutSettings::default(),
        plugin_load_status: "Not loaded.".to_string(),
        plugin_load_dir: None,
        plugin_list: im::vector![],
    };

    println!("Starting the GUI.");
    let (tx, rx) = mpsc::channel(); // A channel to send info from the UI to the core.
    let launcher = AppLauncher::with_window(
        get_main_window_desc(tx)
    ).delegate(
        Delegate {
            window_count: 0,
        }
    );
    let event_sink = launcher.get_external_handle();
    
    let thread_handle = thread::spawn( move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        println!("Launching core in runtime.");
        rt.block_on(async {
            println!("Waiting for GUI to be ready for messages being passed over.");
            let received = rx.recv_timeout(Duration::from_secs(1));
            if received.is_err() || received.unwrap() == false {
                warn!("Failed to receive GUI ready message, or it was false.");
                println!("Failed to receive GUI ready message, or it was false.");
            } else {
                println!("Got GUI ready message. Starting core.");
            }
            thread::sleep(Duration::from_secs(1));
            println!("Initializing core from new thread in GUI");
            let application_core = polychat_ipc::core::Core::new_in_home();
            let core_interface = core_interface::CoreInterface::new(event_sink);

            println!("Starting core in another thread from the GUI.");
            application_core.unwrap().run(&core_interface).unwrap();
            thread::sleep(Duration::from_millis(20));
        });
    });


    launcher.launch(
        initial_state
    )?;
    println!("Waiting for core thread to stop");
    thread_handle.join().unwrap();
    Ok(())
}