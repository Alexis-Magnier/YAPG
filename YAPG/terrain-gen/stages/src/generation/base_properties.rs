use godot::prelude::*;

use terrain_gen_core::data::PlanetData;
use terrain_gen_core::stage::Stage;
use terrain_gen_macros::godot_stage_export;

use noise::{Fbm, MultiFractal, NoiseFn, Simplex};

#[godot_stage_export]
#[derive(GodotClass)]
#[class(base = Resource, init, tool)]
struct AssignBaseProperties{

    #[export(range = (0.0, f64::INFINITY, or_greater))]
    density_noise_seed: u32,

    #[export(range = (0.0, f64::INFINITY, or_greater))]
    height_noise_seed: u32,

    #[export]
    #[init(val = 6)]
    octaves: u32,
    
    #[export]
    #[init(val = 1.0)]
    frequency: f32,

    #[export]
    #[init(val = 2.0)]
    lacunarity: f32,

    #[export]
    #[init(val = 0.5)]
    persistence: f32,

    base: Base<Resource>
}

impl AssignBaseProperties{

    fn assign_density(&self, data: &mut PlanetData) -> anyhow::Result<()>{
        let noise = Fbm::<Simplex>::new(self.density_noise_seed)
            .set_octaves(self.octaves as usize)
            .set_frequency(self.frequency as f64)
            .set_lacunarity(self.lacunarity as f64)
            .set_persistence(self.persistence as f64);
    
        data.cells.density = (0..data.cells.count).map(|cell|{
            let cell_idx = cell as usize;

            let plate_idx = data.cells.plate_id[cell_idx] as usize;

            let base_noise = {
                let base = noise.get(data.cells.position[cell_idx].as_dvec3().to_array()) as f32;
                
                let ampl = data.plates.density_ampl[plate_idx];
                let mean = data.plates.density_mean[plate_idx];

                base * ampl + mean
            };

            base_noise
        }).collect();

        Ok(())
    }

    fn assign_width(&self, data: &mut PlanetData) -> anyhow::Result<()>{
        let noise = Fbm::<Simplex>::new(self.height_noise_seed)
            .set_octaves(self.octaves as usize)
            .set_frequency(self.frequency as f64)
            .set_lacunarity(self.lacunarity as f64)
            .set_persistence(self.persistence as f64);

        data.cells.thickness = (0..data.cells.count).map(|cell|{
            let cell_idx = cell as usize;

            let plate_idx = data.cells.plate_id[cell_idx] as usize;

            let base_noise = {
                let base = noise.get(data.cells.position[cell_idx].as_dvec3().to_array()) as f32;
                
                let ampl = data.plates.thickness_ampl[plate_idx];
                let mean = data.plates.thickness_mean[plate_idx];

                base * ampl + mean
            };

            base_noise
        }).collect();

        Ok(())
    }
}

impl Stage for AssignBaseProperties{
    fn name(&self) -> &'static str {
        "Assign properties"
    }

    fn run(&self, planet: &mut terrain_gen_core::data::PlanetData) -> anyhow::Result<()> {
        self.assign_density(planet)?;
        self.assign_width(planet)?;

        Ok(())
    }
}