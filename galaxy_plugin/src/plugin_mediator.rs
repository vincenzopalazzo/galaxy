//! Personal implementation of the Mediator pattern to implement the message
//! dispacing between the logic and the plugin.
use clightningrpc_plugin::errors::PluginError;
use std::rc::Rc;

pub struct GalaxyMediator;

impl GalaxyMediator {
    pub fn dispach<T>(command: &GalaxyCommand) -> Result<Rc<T>, PluginError> {
        match command {
            GalaxyCommand::GetNode(node_id) => todo!("not implemented yet"),
            GalaxyCommand::GetNodes(limit, batch_list) => todo!("not implemented yet"),
        }
    }
}

pub enum GalaxyCommand {
    /// Get the information of a particular node by
    /// node id!
    GetNode(String),
    /// Get The nodes information in small chunks specified
    /// by the user.
    /// Required a node by a limit (u64) that said the first x element
    /// or also a batch request that work like "please give me the info about these nodes"
    GetNodes(Option<u64>, Option<Vec<String>>),
}
