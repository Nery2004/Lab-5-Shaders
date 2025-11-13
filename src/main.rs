use nalgebra_glm::{Vec3, Mat4};
use minifb::{Key, Window, WindowOptions, MouseButton, MouseMode};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod line;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use triangle::triangle;
use shaders::vertex_shader;
use color::Color;

pub struct Uniforms {
    model_matrix: Mat4,
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

fn render_nave(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertices: &[Vertex], indices: &[u32]) {
    
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
                
                // Procesar fragmentos
                for fragment in fragments {
                    let x = fragment.position.x as usize;
                    let y = fragment.position.y as usize;
                    if x < framebuffer.width && y < framebuffer.height {
                        let color = fragment.color.to_hex();
                        framebuffer.set_current_color(color);
                        framebuffer.point(x, y, fragment.depth);
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
        "Proyecto 1 - Nave Espacial",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    // Configurar colores
    framebuffer.set_background_color(0x001122); // Fondo negro para el espacio
    
    // Parámetros de transformación
    let mut translation = Vec3::new(400.0, 300.0, 0.0); // Centrar en pantalla
    let mut rotation = Vec3::new(0.0, 0.0, 0.0);
    let mut scale = 100.0f32;

    // Cargar el modelo de la nave
    let obj = match Obj::load("assets/CazaTie.obj") {
        Ok(obj) => {
            println!("¡Modelo CazaTie.obj cargado exitosamente!");
            obj
        },
        Err(e) => {
            eprintln!("Error cargando CazaTie.obj: {:?}", e);
            eprintln!("Asegúrate de que el archivo assets/CazaTie.obj existe");
            return;
        }
    };

    let (vertices, indices) = obj.get_vertex_and_index_arrays();
    println!("Nave cargada: {} vértices, {} triángulos", vertices.len(), indices.len() / 3);

    // Variables para el control del mouse
    let mut last_mouse_pos: Option<(f32, f32)> = None;
    let mouse_sensitivity = 0.005;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Manejar rotación con el mouse
        if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Clamp) {
            if window.get_mouse_down(MouseButton::Left) {
                if let Some((last_x, last_y)) = last_mouse_pos {
                    let delta_x = mx - last_x;
                    let delta_y = my - last_y;
                    
                    // Rotar el modelo basado en el movimiento del mouse
                    rotation.y += delta_x * mouse_sensitivity; // rotación horizontal (yaw)
                    rotation.x += delta_y * mouse_sensitivity; // rotación vertical (pitch)
                }
                last_mouse_pos = Some((mx, my));
            } else {
                last_mouse_pos = None;
            }
        }

        handle_input(&window, &mut translation, &mut rotation, &mut scale);

        // Limpiar framebuffer
        framebuffer.clear();

        // Crear matriz de transformación
        let model_matrix = create_model_matrix(translation, scale, rotation);
        let uniforms = Uniforms { model_matrix };

        // Renderizar la nave
        framebuffer.set_current_color(0xFFDD44); // Color blanco para la nave
        render_nave(&mut framebuffer, &uniforms, &vertices, &indices);

        // Actualizar ventana
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }

    println!("¡Renderizado completado!");
}

fn handle_input(window: &Window, translation: &mut Vec3, rotation: &mut Vec3, scale: &mut f32) {
    let move_speed = 5.0;
    let rotation_speed = PI / 30.0;
    let scale_speed = 2.0;

    // Movimiento
    if window.is_key_down(Key::Right) {
        translation.x += move_speed;
    }
    if window.is_key_down(Key::Left) {
        translation.x -= move_speed;
    }
    if window.is_key_down(Key::Up) {
        translation.y -= move_speed;
    }
    if window.is_key_down(Key::Down) {
        translation.y += move_speed;
    }

    // Escala
    if window.is_key_down(Key::S) {
        *scale += scale_speed;
    }
    if window.is_key_down(Key::A) {
        *scale -= scale_speed;
        if *scale < 1.0 {
            *scale = 1.0;
        }
    }

    // Rotación
    if window.is_key_down(Key::Q) {
        rotation.x -= rotation_speed;
    }
    if window.is_key_down(Key::W) {
        rotation.x += rotation_speed;
    }
    if window.is_key_down(Key::E) {
        rotation.y -= rotation_speed;
    }
    if window.is_key_down(Key::R) {
        rotation.y += rotation_speed;
    }
    if window.is_key_down(Key::T) {
        rotation.z -= rotation_speed;
    }
    if window.is_key_down(Key::Y) {
        rotation.z += rotation_speed;
    }
}