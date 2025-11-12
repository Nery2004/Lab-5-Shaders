use nalgebra_glm::{Vec3, Mat4};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod shaders;
mod fragment;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use triangle::triangle;
use shaders::{vertex_shader, shade_star, shade_rocky, shade_gas_giant};
use color::Color;

pub struct Uniforms {
    model_matrix: Mat4,
    time: f32,
    shader_type: u32, // 0 = star, 1 = rocky, 2 = gas_giant
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}

fn render_sphere(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertices: &[Vertex], indices: &[u32]) {
    
    // Vertex Shader Stage - transformar todos los vértices
    let mut transformed_vertices = Vec::with_capacity(vertices.len());
    for vertex in vertices {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Recorrer manualmente las caras usando los índices
    // Cada 3 índices forman un triángulo
    for triangle_idx in (0..indices.len()).step_by(3) {
        if triangle_idx + 2 < indices.len() {
            let i1 = indices[triangle_idx] as usize;
            let i2 = indices[triangle_idx + 1] as usize;
            let i3 = indices[triangle_idx + 2] as usize;

            // Verificar que los índices estén dentro del rango
            if i1 < transformed_vertices.len() && i2 < transformed_vertices.len() && i3 < transformed_vertices.len() {
                let v1 = &transformed_vertices[i1];
                let v2 = &transformed_vertices[i2];
                let v3 = &transformed_vertices[i3];

                // Dibujar el triángulo
                let fragments = triangle(v1, v2, v3);
                
                // Procesar fragmentos - Aquí aplicamos los shaders procedurales
                for fragment in fragments {
                    let x = fragment.position.x as usize;
                    let y = fragment.position.y as usize;
                    if x < framebuffer.width && y < framebuffer.height {
                        // Aplicar el shader correspondiente según el tipo
                        let shader_color = match uniforms.shader_type {
                            0 => shade_star(fragment.vertex_position, uniforms.time),
                            1 => shade_rocky(fragment.vertex_position, uniforms.time),
                            2 => shade_gas_giant(fragment.vertex_position, uniforms.time),
                            _ => Vec3::new(1.0, 1.0, 1.0), // Blanco por defecto
                        };
                        
                        // Convertir Vec3 (0.0-1.0) a color hex
                        let r = (shader_color.x * 255.0).clamp(0.0, 255.0) as u32;
                        let g = (shader_color.y * 255.0).clamp(0.0, 255.0) as u32;
                        let b = (shader_color.z * 255.0).clamp(0.0, 255.0) as u32;
                        let color = (r << 16) | (g << 8) | b;
                        
                        framebuffer.set_current_color(color);
                        framebuffer.point(x, y, fragment.depth);
                    }
                }
            }
            }
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Lab 5 - Shaders de Cuerpos Celestes",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    // Configurar colores
    framebuffer.set_background_color(0x000000); // Fondo negro para el espacio
    
    // Parámetros de transformación
    let translation = Vec3::new(400.0, 300.0, 0.0); // Centrar en pantalla
    let mut rotation = Vec3::new(0.0, 0.0, 0.0);
    let scale = 200.0f32;

    // Cargar el modelo de la esfera
    let obj = match Obj::load("assets/sphere.obj") {
        Ok(obj) => {
            println!("¡Modelo sphere.obj cargado exitosamente!");
            obj
        },
        Err(e) => {
            eprintln!("Error cargando sphere.obj: {:?}", e);
            eprintln!("Asegúrate de que el archivo assets/sphere.obj existe");
            return;
        }
    };

    let (vertices, indices) = obj.get_vertex_and_index_arrays();
    println!("Esfera cargada: {} vértices, {} triángulos", vertices.len(), indices.len() / 3);

    // Variable para controlar el shader activo
    let mut current_shader: u32 = 0; // 0 = star, 1 = rocky, 2 = gas_giant
    let mut time = 0.0f32;

    println!("\n=== CONTROLES ===");
    println!("1: Estrella (Sol)");
    println!("2: Planeta Rocoso");
    println!("3: Gigante Gaseoso");
    println!("A/D: Rotar en eje Y");
    println!("W/S: Rotar en eje X");
    println!("ESC: Salir");
    println!("=================\n");

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Cambiar shader con teclas numéricas
        if window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) {
            current_shader = 0;
            println!("Shader activo: Estrella (Sol)");
        }
        if window.is_key_pressed(Key::Key2, minifb::KeyRepeat::No) {
            current_shader = 1;
            println!("Shader activo: Planeta Rocoso");
        }
        if window.is_key_pressed(Key::Key3, minifb::KeyRepeat::No) {
            current_shader = 2;
            println!("Shader activo: Gigante Gaseoso");
        }

        // Control de rotación
        handle_input(&window, &mut rotation);

        // Actualizar tiempo para animación
        time += 0.016; // Aproximadamente 60 FPS

        // Limpiar framebuffer
        framebuffer.clear();

        // Crear matriz de transformación
        let model_matrix = create_model_matrix(translation, scale, rotation);
        let uniforms = Uniforms { 
            model_matrix,
            time,
            shader_type: current_shader,
        };

        // Renderizar la esfera con el shader activo
        render_sphere(&mut framebuffer, &uniforms, &vertices, &indices);

        // Actualizar ventana
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }

    println!("¡Renderizado completado!");
}

fn handle_input(window: &Window, rotation: &mut Vec3) {
    let rotation_speed = PI / 60.0;

    // Rotación
    if window.is_key_down(Key::A) {
        rotation.y -= rotation_speed;
    }
    if window.is_key_down(Key::D) {
        rotation.y += rotation_speed;
    }
    if window.is_key_down(Key::W) {
        rotation.x -= rotation_speed;
    }
    if window.is_key_down(Key::S) {
        rotation.x += rotation_speed;
    }
}