use godot::prelude::*;

use rand_distr::num_traits::Inv;
use terrain_gen_core::data::PlanetData;
use terrain_gen_core::stage::Stage;
use terrain_gen_macros::godot_stage_export;

use glam::Vec3;

#[godot_stage_export]
#[derive(GodotClass)]
#[class(base = Resource, init, tool)]
struct CollisionDetection{

    #[export]
    #[init(val = 0.1)]
    dt: f32,

    #[export]
    #[init(val = 0.3)]
    boundary_threshold: f32,

    #[export_group(name = "Pinning")]

    #[export]
    #[init(val = 1.0)]
    weight_stress_factor: f32,

    #[export]
    #[init(val = -0.1)]
    self_subduction_height: f32,

    #[export]
    #[init(val = 0.0)]
    other_subduction_height: f32,

    base: Base<Resource>
}

impl CollisionDetection{

    fn detect_collisions(&self, data: &mut PlanetData) -> anyhow::Result<()>{
        
        let cell_count = data.cells.position.len();
        let dt = self.dt;

        let mut cell_stress = vec![Vec3::ZERO; cell_count];
        let mut cell_height = vec![f32::NAN; cell_count];

        let mut convergent_cells: Vec<u32> = Vec::new();
        let mut divergent_cells: Vec<u32> = Vec::new();
        let mut transform_cells: Vec<u32> = Vec::new();

        for current_cell in 0..cell_count{
            let mut best_compression = f32::NEG_INFINITY;
            let mut best_cell = u32::MAX;

            let current_plate = data.cells.plate_id[current_cell];
            let current_plate_vec = &data.plates.velocity[current_plate as usize];
            let current_pos = &data.cells.position[current_cell];
            let current_pos_after = current_pos + current_plate_vec * dt;

            for neighbor in data.mesh.r_circulate_r(current_cell as u32){
                let n_idx = neighbor as usize;
                let n_plate = data.cells.plate_id[n_idx];

                if current_plate == n_plate{
                    continue;
                }

                let n_plate_vec = &data.plates.velocity[n_plate as usize];

                let n_pos = &data.cells.position[n_idx];
                let n_pos_after = n_pos + n_plate_vec * dt;

                let distance_before = (current_pos - n_pos).length();
                let distance_after  = (current_pos_after - n_pos_after).length();

                let compression =  (distance_before - distance_after).inv();

                if compression > best_compression{
                    best_cell = neighbor;
                    best_compression = compression;
                }
            }
            
            // If there is no best cell, skip
            // This cases can append if all the cell's neighbor are on the same plate
            if best_cell == u32::MAX {
                continue;
            }

            
            let best_plate = data.cells.plate_id[best_cell as usize];
            let best_plate_vec = &data.plates.velocity[best_plate as usize];


            let dir = (data.cells.position[best_cell as usize] - current_pos).normalize();

            // Relative velocity of current plate relative to best plate
            let rel_vel = current_plate_vec - best_plate_vec;
            
            // How much the cell is converging toward the other
            // A negative value thus represents a divergent collision
            let convergent_stress = rel_vel.dot(dir);


            let current_weight = data.plates.density_mean[current_plate as usize] * data.plates.thickness_mean[current_plate as usize];
            let best_weight = data.plates.density_mean[best_plate as usize] * data.plates.thickness_mean[best_plate as usize];

            let subduction_factor = (current_weight - best_weight) * self.weight_stress_factor * convergent_stress;

            if convergent_stress >= self.boundary_threshold {
                convergent_cells.push(current_cell as u32);

                let height = if subduction_factor > 0.0 {
                        self.self_subduction_height
                    } else {
                        self.other_subduction_height
                    } * subduction_factor.abs();

                cell_height[current_cell] = height;

            } else if convergent_stress <= -self.boundary_threshold{
                divergent_cells.push(current_cell as u32);
            } else {
                transform_cells.push(current_cell as u32);
            }

            cell_stress[current_cell] = dir * convergent_stress;
        }

        data.convergent_cells = convergent_cells;
        data.divergent_cells = divergent_cells;
        data.transform_cells = transform_cells;

        data.cells.height = cell_height;
        data.cells.stress = cell_stress;

        Ok(())
    }
}

impl Stage for CollisionDetection{
    fn name(&self) -> &'static str {
        "Collision Detection"
    }

    fn run(&self, planet: &mut terrain_gen_core::data::PlanetData) -> anyhow::Result<()> {
        self.detect_collisions(planet)
    }
}