use godot::prelude::*;
use terrain_gen_core::pipeline::Pipeline;
use terrain_gen_core::stage::{make_stage, Stage};

#[derive(GodotClass)]
#[class(base = Resource, tool, init)]
pub struct TerrainGenerationPipeline{
    #[export]
    stages: Array<Gd<Resource>>,

    base: Base<Resource>
}

impl TerrainGenerationPipeline{
    pub fn make_pipeline(&self) -> anyhow::Result<Pipeline> {
        let stages  = self.stages
            .iter_shared()
            .map(|res| make_stage(res))
            .collect::<anyhow::Result::<Vec<Box<dyn Stage>>>>()?;

        Ok(Pipeline {
            stages
        })
    }
}

