//! GraphQl Gralaxy server build to observer and enjoy querying your
//! Lightning node.
mod plugin;

fn main() {
    let mut plugin = plugin::build_plugin();
    plugin.start();
}