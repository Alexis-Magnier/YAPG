use super::data::PlanetData;
use godot::prelude::*;

pub trait Stage {
    fn name(&self) -> &'static str;
    fn run(&self, planet: &mut PlanetData) -> anyhow::Result<()>;
}

pub struct StageConverter(pub fn(Gd<Resource>) -> Option<Box<dyn Stage>>);
inventory::collect!(StageConverter);

pub fn make_stage(resource: Gd<Resource>) -> anyhow::Result<Box<dyn Stage>>{
    for entry in inventory::iter::<StageConverter> {
        if let Some(stage) = (entry.0)(resource.clone()) {
            return Ok(stage);
        }
    }
    anyhow::bail!("Unknown resource type \"{}\"", resource.get_name())
}