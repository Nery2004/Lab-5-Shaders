use nalgebra_glm::Vec3;
use crate::color::Color;

pub struct Fragment {
    pub position: Vec3,
    pub color: Color,
    pub depth: f32,
    pub vertex_position: Vec3, // Posición original del vértice para los shaders
}

impl Fragment {
    pub fn new(x: f32, y: f32, color: Color, depth: f32) -> Self {
        Fragment {
            position: Vec3::new(x, y, depth),
            color,
            depth,
            vertex_position: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn new_with_vertex_position(x: f32, y: f32, color: Color, depth: f32, vertex_position: Vec3) -> Self {
        Fragment {
            position: Vec3::new(x, y, depth),
            color,
            depth,
            vertex_position,
        }
    }
}
