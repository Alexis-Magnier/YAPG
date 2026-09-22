use anyhow::bail;
use glam::Vec3;
use godot::classes::mesh::{ArrayCustomFormat, ArrayFormat};
use godot::obj::EngineBitfield;
use godot::{classes::MeshInstance3D, prelude::*};
use godot::classes::{ArrayMesh, BaseMaterial3D, Material, StandardMaterial3D, mesh};
use terrain_gen_core::data::PlanetData;
use super::terrain_gen::pipeline::TerrainGenerationPipeline;

#[derive(GodotClass)]
#[class(base=Node3D, tool, init)]
pub struct Planet{
    #[export]
    pipeline: Option<Gd<TerrainGenerationPipeline>>,
    
    #[export_tool_button(fn=Self::generate)]
    generate_planet: PhantomVar<Callable>,

    #[export]
    material: Option<Gd<Material>>,

    base: Base<Node3D>
}

struct ColorStop {
    elevation: f32,
    color: [f32; 3], // RGB normalized 0.0 to 1.0
}

const PALETTE: &[ColorStop] = &[
    ColorStop { elevation: -1.0,  color: [0.008, 0.043, 0.149] }, // Deep Trench
    ColorStop { elevation: -0.6,  color: [0.043, 0.129, 0.290] }, // Deep Ocean
    ColorStop { elevation: -0.2,  color: [0.122, 0.369, 0.549] }, // Shallow Water
    ColorStop { elevation: 0.0,   color: [0.851, 0.765, 0.510] }, // Sand/Shore
    ColorStop { elevation: 0.1,   color: [0.322, 0.549, 0.220] }, // Lowland
    ColorStop { elevation: 0.3,   color: [0.176, 0.349, 0.133] }, // Forest
    ColorStop { elevation: 0.5,   color: [0.549, 0.451, 0.294] }, // Plateau
    ColorStop { elevation: 0.7,   color: [0.361, 0.322, 0.282] }, // Rock
    ColorStop { elevation: 0.85,  color: [0.722, 0.698, 0.655] }, // High Ridge
    ColorStop { elevation: 1.0,   color: [1.000, 1.000, 1.000] }, // Snow Peak
];

pub fn get_elevation_color(h: f32) -> [f32; 3] {
    let h = h.clamp(-1.0, 1.0);
    
    for i in 0..PALETTE.len() - 1 {
        if h <= PALETTE[i + 1].elevation {
            let t = (h - PALETTE[i].elevation) / (PALETTE[i + 1].elevation - PALETTE[i].elevation);
            let c1 = PALETTE[i].color;
            let c2 = PALETTE[i + 1].color;
            
            return [
                c1[0] + t * (c2[0] - c1[0]),
                c1[1] + t * (c2[1] - c1[1]),
                c1[2] + t * (c2[2] - c1[2]),
            ];
        }
    }
    
    PALETTE[PALETTE.len() - 1].color
}

#[godot_api]
impl Planet {
    fn generate_planet(&mut self) -> anyhow::Result<()>{
        let Some(pipeline) = self.pipeline.clone() else {
            anyhow::bail!("No pipeline is assigned");
        };

        let pipeline_res = pipeline.bind();
        let pipeline = pipeline_res.make_pipeline()?;

        let planet_data = pipeline.run()?;

        self.make_mesh(planet_data)?;

        Ok(())
    }

    #[func]
    fn generate(&mut self){
        match self.generate_planet(){
            Ok(_) => {
                godot_print!("Planet generated !");
            }

            Err(err) => {
                godot_error!("Error during planet generation \"{err}\"");
            }
        }
    }

    fn get_mesh_node(&mut self) -> anyhow::Result<Gd<MeshInstance3D>> {
        const MESH_NAME: &str = "MESH";

        if let Some(node) = self.base().try_get_node_as::<MeshInstance3D>(MESH_NAME) {
            return Ok(node);
        }

        if let Some(node) = self.base().get_node_or_null(MESH_NAME) {
            bail!(
                "Node '{}' exists but is not a MeshInstance3D (found {})",
                MESH_NAME,
                node.get_class()
            );
        }

        let mut mesh = MeshInstance3D::new_alloc();
        mesh.set_name(MESH_NAME);
        self.base_mut().add_child(&mesh);

        Ok(mesh)
    }

    fn generate_triangle_centers(data: &PlanetData) -> Vec<Vec3> {
        let mesh = &data.mesh;
        let positions = &data.cells.position;
        let height = &data.cells.height;
        let triangle_count = mesh.num_triangles;

        (0..triangle_count).map(|t| {

            
            let a = mesh.s_begin_r(3*t) as usize;
            let b = mesh.s_begin_r(3*t+1) as usize;
            let c = mesh.s_begin_r(3*t+2) as usize;
            
            let a_pos = positions[a as usize];
            let b_pos = positions[b as usize];
            let c_pos = positions[c as usize];

            (a_pos + b_pos + c_pos) / 3.
        }).collect()
    }
    

    fn make_mesh(&mut self, data: PlanetData) -> anyhow::Result<()>{

        let mut mesh_node = self.get_mesh_node()?;
        
        let mesh = &data.mesh;

        let mut vertices = PackedVector3Array::new();
        vertices.resize((mesh.num_sides * 3) as usize);

        let mut height = PackedFloat32Array::new();
        height.resize((mesh.num_sides * 3) as usize);
        height.fill(0.0);

        // let mut normals = PackedVector3Array::new();
        // vertices.resize((mesh.num_sides * 3) as usize);


        let t_center = Self::generate_triangle_centers(&data);

        for s in 0..mesh.num_sides{
            let s_idx = s as usize;

            let inner_t = mesh.s_inner_t(s) as usize;
            let outer_t = mesh.s_outer_t(s) as usize;
            let r = mesh.s_begin_r(s) as usize;

            let h = data.cells.height[r] / 200.0;
            
            // let plate_t = data.cells.plate_id[r];
            // let h = data.plates.height_mean[plate_t as usize] / 50.0 + b * ;
        
            vertices[s_idx * 3 + 0] = Vector3::from_array(t_center[inner_t].to_array());
            vertices[s_idx * 3 + 1] = Vector3::from_array(t_center[outer_t].to_array());
            vertices[s_idx * 3 + 2] = Vector3::from_array(data.cells.position[r].to_array());

            height[s_idx * 3 + 0] = h;
            height[s_idx * 3 + 1] = h;
            height[s_idx * 3 + 2] = h;
        }

        let mut surface_array = VarArray::new();
        surface_array.resize(mesh::ArrayType::MAX.ord() as usize, &Variant::nil());
        
        surface_array.set(mesh::ArrayType::VERTEX.ord() as usize, &vertices.to_variant());
        surface_array.set(mesh::ArrayType::NORMAL.ord() as usize, &vertices.to_variant());
        surface_array.set(mesh::ArrayType::CUSTOM0.ord() as usize, &height.to_variant());

        let custom_bits = (ArrayCustomFormat::R_FLOAT.ord() as u64) << ArrayFormat::CUSTOM0_SHIFT.ord();

        // Combine raw bitwise mask with enum flags
        let flags = EngineBitfield::from_ord(custom_bits);

        let mut array_mesh = ArrayMesh::new_gd();
        // array_mesh.add_surface_from_arrays(mesh::PrimitiveType::TRIANGLES, &surface_array);
        array_mesh.add_surface_from_arrays_ex(mesh::PrimitiveType::TRIANGLES, &surface_array)
            .flags(flags)
            .done();
        
        mesh_node.set_mesh(&array_mesh);

        if let Some(material) = &self.material{
            mesh_node.set_material_override(material);
        }

        Ok(())
    }
}