//! GraphQl Gralaxy server build to observer and enjoy querying your
//! Lightning node.
mod db;
mod plugin;
mod plugin_mediator;

fn main() {
    let mut plugin = plugin::build_plugin();
    plugin.start();
}
