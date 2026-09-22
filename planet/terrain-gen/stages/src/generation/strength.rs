use godot::prelude::*;

use terrain_gen_core::data::PlanetData;
use terrain_gen_core::stage::Stage;
use terrain_gen_macros::godot_stage_export;

#[godot_stage_export]
#[derive(GodotClass)]
#[class(base = Resource, init, tool)]
struct Strength{

    #[export_group(name = "Noise")]
    #[export]
    noise_seed: u32,

    #[export]
    #[init(val = 6)]
    noise_octaves: u32,

    #[export]
    #[init(val = 1.0)]
    noise_frequency: f32,

    #[export]
    #[init(val = 1.0)]
    noise_lacunarity: f32,

    #[export]
    #[init(val = 0.5)]
    noise_persistence: f32,

    #[export]
    #[init(val = 1.0)]
    noise_amplitude: f32,

    #[export]
    #[init(val = 0.0)]
    noise_mean: f32,

    #[export]
    #[init(val = 1.0)]
    noise_weight: f32,

    #[export_group(name = "Temperature")]

    #[export]
    #[init(val = 1.0)]
    thermal_weight: f32,

    base: Base<Resource>,
}

impl Strength{

    fn add_noise(&self, data: &mut PlanetData) -> anyhow::Result<()>{

        use noise::{Fbm, MultiFractal, NoiseFn, Simplex};

        let noise = Fbm::<Simplex>::new(self.noise_seed)
            .set_octaves(self.noise_octaves as usize)
            .set_frequency(self.noise_frequency as f64)
            .set_lacunarity(self.noise_lacunarity as f64)
            .set_persistence(self.noise_persistence as f64);

        data.cells.yeild_strength = (0..data.cells.count).map(|c|{
            let c_idx = c as usize;

            let p_idx = data.cells.plate_id[c_idx] as usize;

            let position = data.cells.position[c_idx];

            let base = noise.get(position.as_dvec3().to_array()) as f32;

            let amplitude = data.plates.strenght_ampl[p_idx] * self.noise_amplitude;
            let mean = data.plates.strenght_mean[p_idx] + self.noise_mean;

            (base * amplitude + mean).clamp(0., 1.0) * self.noise_weight
        }).collect();

        Ok(())
    }

    fn compute_strength(&self, data: &mut PlanetData) -> anyhow::Result<()>{
        
        data.cells.yeild_strength
            .iter_mut()
            .enumerate()
            .for_each(|(idx, strength)|{
                let t = data.cells.temperature[idx];

                *strength /= 1. + t * self.thermal_weight;
            });
            

        Ok(())
    }
}

impl Stage for Strength{
    fn name(&self) -> &'static str {
        "Strength"
    }

    fn run(&self, planet: &mut terrain_gen_core::data::PlanetData) -> anyhow::Result<()> {
        self.add_noise(planet)?;
        self.compute_strength(planet)?;
        Ok(())
    }
}