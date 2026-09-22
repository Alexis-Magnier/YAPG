use godot::classes::notify::NodeNotification;
use godot::prelude::*;
use godot::classes::{Node, INode};

use crate::render::context::VulkanContext;

#[derive(GodotClass)]
#[class(base=Node, tool)]
pub struct PlanetBackend{
    context: VulkanContext,

    base: Base<Node>
}

#[godot_api]
impl PlanetBackend{
    #[func]
    pub fn print_hello(&mut self){
        godot_print!("Hello world !");
    }
}

#[godot_api]
impl INode for PlanetBackend{
    fn init(base: Base<Node>) -> Self {
        godot_print!("Loading PlanetBackend");

        let context = VulkanContext::new()
            .expect("Failed to create vulkan context");

        Self {
            context: context,
            base: base
        }
    }

    fn on_notification(&mut self, what: NodeNotification) {
        if what == NodeNotification::PREDELETE {
            godot_print!("Unloading PlanetBackend");

            self.context.shutdown()
        }
    }
}