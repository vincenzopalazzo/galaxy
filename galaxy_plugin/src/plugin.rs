//! Galaxy plugin implementation.
//!
//! The end goal of this plugin is provide a collection solution
//! to be able to collect costly information about a lightning network
//! node and expose this information throgh a minimal interface.
use clightningrpc_plugin::{commands::RPCCommand, errors::PluginError, plugin::Plugin};
use clightningrpc_plugin_macros::{add_plugin_rpc, rpc_method};
use serde_json::{json, Value};

#[derive(Clone)]
pub struct PluginState;

pub fn build_plugin() -> Plugin<PluginState> {
    let mut plugin = Plugin::new(PluginState {}, true);
    add_plugin_rpc!(plugin, "galaxy");
    plugin.clone()
}

#[rpc_method(
    rpc_name = "galaxy",
    description = "This is a simple and short description"
)]
pub fn mediator_rpc(_plugin: Plugin<()>, _request: Value) -> Result<Value, PluginState> {
    /// The name of the parameters can be used only if used, otherwise can be omitted
    /// the only rules that the macros require is to have a propriety with the following rules:
    /// - Plugin as _plugin
    /// - CLN JSON request as _request
    /// The function parameter can be specified in any order.
    Ok(json!({"is_dynamic": _plugin.dynamic, "rpc_request": _request}))
}
