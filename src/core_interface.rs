use std::path::PathBuf;

use polychat_ipc::{core::{ui_interface::ui_trait, ui_interface::load_status::LoadStatus}, api::schema::protocol::InitDataInstruction};

use crate::data::{plugin_item_data::PluginItemData, app_state_data::AppState};

/**
 * This struct stores information to allow the core GUI interface
 * to send data to the AppState, as well as other things.
 */
pub(crate) struct CoreInterface {
    /// An event sink that allows passing data changes to the UI.
    event_sink: druid::ExtEventSink,
}

impl CoreInterface {
    pub fn new(event_sink: druid::ExtEventSink) -> CoreInterface {
        CoreInterface { event_sink: event_sink, }
    }
}

/**
 * CoreInterface implements the trait GUI to get information
 * from the core.
 */
 impl ui_trait::GUI for CoreInterface {
    fn on_core_pre_init(&self) {
        println!("on_core_pre_init called.");
    }

    fn on_core_post_init(&self, plugin_loaded_dir: Option<PathBuf>) {
        println!("on_core_post_init called with loaded directory {:?}.", plugin_loaded_dir);
        if let Some(plugin_loaded_dir) = plugin_loaded_dir {
            self.event_sink.add_idle_callback(move |data: &mut AppState| {
                (*data).plugin_load_dir = Some(plugin_loaded_dir.to_string_lossy().to_string());
                println!("on_core_post_init's callback called.");
            });
        }
    }

    fn on_plugin_loaded(&self, plugin_name: String) {
        println!("on_plugin_loaded called with plugin name {plugin_name}.");
        self.event_sink.add_idle_callback(move |data: &mut AppState| {
            (*data).plugin_list.push_back(PluginItemData { plugin_name });
            println!("on_plugin_loaded's callback called.");
        });
    }

    fn on_plugin_load_failure(&self, error_msg: String) {
        println!("on_plugin_load_failure called with error message {error_msg}.")
    }

    fn on_plugins_loaded_status_change(&self, status: LoadStatus) {
        println!("on_plugin_loaded_status_change called with status {:?}.", status);
        self.event_sink.add_idle_callback(move |data: &mut AppState| {
            (*data).plugin_load_status = format!("{:?}", status).to_string();
            println!("on_plugin_loaded_status_change's callback called.");
        });
    }

    fn on_plugin_init(&self, plugin_init_data: InitDataInstruction) {
        todo!()
    }

}