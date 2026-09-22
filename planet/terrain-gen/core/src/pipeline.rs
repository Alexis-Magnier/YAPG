use std::time::Instant;

use godot::global::godot_print;

use super::data::PlanetData;
use super::stage::Stage;


#[derive(Default)]
pub struct Pipeline{
    pub stages: Vec<Box<dyn Stage>>
}

impl Pipeline{
    pub fn run(&self) -> anyhow::Result<PlanetData>{
        let mut planet = PlanetData::default();

        
        for stage in &self.stages{
            let now = Instant::now();
            stage.run(&mut planet)?;
            godot_print!("stage \"{}\" took {:#?}", stage.name(), now.elapsed());
            
        }
        Ok(planet)
    }
}
