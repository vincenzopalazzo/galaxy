//! Galaxy plugin implementation.
//!
//! The end goal of this plugin is provide a collection solution
//! to be able to collect costly information about a lightning network
//! node and expose this information throgh a minimal interface.
use clightningrpc_plugin::{
    commands::RPCCommand, errors::PluginError, plugin::Plugin, types::LogLevel,
};
use serde_json::{json, Value};
use std::cell::RefCell;
use std::rc::Rc;

use crate::db::GalaxyDB;

#[derive(Clone)]
pub struct PluginState {
    pub(crate) db: Option<Rc<RefCell<GalaxyDB>>>,
}

impl PluginState {
    fn new() -> Self {
        PluginState { db: None }
    }
}

pub fn build_plugin() -> Plugin<PluginState> {
    Plugin::new(PluginState::new(), true)
        .on_init(&on_init)
        .add_rpc_method("galaxy", "", "Galaxy rpc method give you the possibility to query the lightning network graph in a fast way", Galaxy{})
        .register_notification("shutdown", OnShutdown{})
        .clone()
}

/// HelloRPC is used to register the RPC method
// FIXME: move this inside the macros!
#[derive(Clone)]
struct Galaxy;

/// Implementation of the RPC method
impl RPCCommand<PluginState> for Galaxy {
    fn call<'c>(
        &self,
        plugin: &mut Plugin<PluginState>,
        _request: &'c Value,
    ) -> Result<Value, PluginError> {
        plugin.log(LogLevel::Debug, "call the custom rpc method from rust");
        let response = json!({
            "language": "Hello from rust"
        });
        Ok(response)
    }
}

/// Init method called from the plugin when cln init it
/// the core of this method is prepare the work directory of galaxy.
pub fn on_init(plugin: &mut Plugin<PluginState>) -> Value {
    let dir = plugin
        .conf
        .as_ref()
        .unwrap()
        .configuration
        .lightning_dir
        .as_str();
    plugin.state.db = Some(Rc::new(RefCell::new(GalaxyDB::new(dir, "galaxy_db"))));
    json!({})
}

#[derive(Clone)]
struct OnShutdown;

impl RPCCommand<PluginState> for OnShutdown {
    fn call_void<'c>(&self, _plugin: &mut Plugin<PluginState>, _request: &'c Value) {
        let db = _plugin.state.db.as_ref();
        if let Some(db) = db {
            if let Err(err) = db.borrow_mut().close() {
                _plugin.clone().log(
                    LogLevel::Debug,
                    format!("An error happen during the close db operation {}", err).as_str(),
                );
            }
        }
    }
}
