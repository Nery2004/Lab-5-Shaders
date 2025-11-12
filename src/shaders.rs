use nalgebra_glm::{Vec3, Vec4, Mat3};
use crate::vertex::Vertex;
use crate::Uniforms;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  // Transform position
  let position = Vec4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );
  let transformed = uniforms.model_matrix * position;

  // Perform perspective division
  let w = transformed.w;
  let transformed_position = Vec3::new(
    transformed.x / w,
    transformed.y / w,
    transformed.z / w
  );

  // Transform normal

  let model_mat3 = Mat3::new(
    uniforms.model_matrix[0], uniforms.model_matrix[1], uniforms.model_matrix[2],
    uniforms.model_matrix[4], uniforms.model_matrix[5], uniforms.model_matrix[6],
    uniforms.model_matrix[8], uniforms.model_matrix[9], uniforms.model_matrix[10]
  );
  let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

  let transformed_normal = normal_matrix * vertex.normal;

  // Create a new Vertex with transformed attributes
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position,
    transformed_normal,
  }
}

// Funciones de ruido procedural
fn noise(p: Vec3) -> f32 {
    let i = p.map(|x| x.floor());
    let f = p.map(|x| x.fract());
    let u = f.component_mul(&f).map(|x| x * (3.0 - 2.0 * x));

    let a = i + Vec3::new(0.0, 0.0, 0.0);
    let b = i + Vec3::new(1.0, 0.0, 0.0);
    let c = i + Vec3::new(0.0, 1.0, 0.0);
    let d = i + Vec3::new(1.0, 1.0, 0.0);
    let e = i + Vec3::new(0.0, 0.0, 1.0);
    let f_vec = i + Vec3::new(1.0, 0.0, 1.0);
    let g = i + Vec3::new(0.0, 1.0, 1.0);
    let h = i + Vec3::new(1.0, 1.0, 1.0);

    let mix = |a, b, t| a + t * (b - a);

    mix(
        mix(
            mix(rand(a), rand(b), u.x),
            mix(rand(c), rand(d), u.x),
            u.y,
        ),
        mix(
            mix(rand(e), rand(f_vec), u.x),
            mix(rand(g), rand(h), u.x),
            u.y,
        ),
        u.z,
    )
}

fn rand(p: Vec3) -> f32 {
    (p.dot(&Vec3::new(12.9898, 78.233, 45.5432)).sin() * 43758.5453).fract()
}

fn fbm(p: Vec3, octaves: i32, persistence: f32, lacunarity: f32) -> f32 {
    let mut total = 0.0;
    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    let mut max_value = 0.0;

    for _ in 0..octaves {
        total += noise(p * frequency) * amplitude;
        max_value += amplitude;
        amplitude *= persistence;
        frequency *= lacunarity;
    }

    total / max_value
}

// Shaders para los cuerpos celestes
pub fn shade_star(point: Vec3, time: f32) -> Vec3 {
    let uv = point.normalize();
    
    // Capa 1: Núcleo brillante
    let mut color = Vec3::new(1.0, 1.0, 0.8); // Amarillo muy claro
    let dist_to_center = uv.magnitude();
    color *= 1.0 - (dist_to_center * 0.8).powf(2.0);

    // Capa 2: Gradiente radial de color
    let radial_grad = (1.0 - uv.magnitude()).powf(3.0);
    let grad_color = Vec3::new(1.0, 0.5, 0.0); // Naranja
    color = color.lerp(&grad_color, radial_grad);

    // Capa 3: Turbulencia de flamas (FBM)
    let turbulence_freq = 5.0;
    let turbulence_speed = 0.5;
    let turbulence = fbm(uv * turbulence_freq + Vec3::new(0.0, 0.0, time * turbulence_speed), 4, 0.5, 2.0);
    let flame_color = Vec3::new(1.0, 0.2, 0.0); // Rojo anaranjado
    color = color.lerp(&flame_color, turbulence * 0.5);

    // Capa 4: Pulsación animada
    let pulse = ((time * 2.0).sin() * 0.5 + 0.5) * 0.2 + 0.9; // Varía entre 0.9 y 1.1
    color *= pulse;

    color.map(|x| x.max(0.0).min(1.0))
}

pub fn shade_rocky(point: Vec3, time: f32) -> Vec3 {
    let uv = point.normalize();

    // Capa 1: Ruido base para continentes y océanos (FBM)
    let base_freq = 2.0;
    let n = fbm(uv * base_freq, 6, 0.5, 2.0);

    // Capa 2: Definición de continentes y océanos con threshold
    let threshold = 0.5;
    let is_land = n > threshold;
    
    let ocean_color_deep = Vec3::new(0.0, 0.1, 0.3); // Azul oscuro
    let ocean_color_shallow = Vec3::new(0.1, 0.3, 0.7); // Azul claro
    let land_color_low = Vec3::new(0.1, 0.4, 0.1); // Verde oscuro
    let land_color_high = Vec3::new(0.6, 0.5, 0.3); // Marrón claro

    let mut color;
    if is_land {
        let land_factor = (n - threshold) / (1.0 - threshold);
        color = land_color_low.lerp(&land_color_high, land_factor.powf(0.7));
    } else {
        let ocean_factor = n / threshold;
        color = ocean_color_deep.lerp(&ocean_color_shallow, ocean_factor);
    }

    // Capa 3: Detalles de terreno (ruido secundario)
    let detail_freq = 10.0;
    let detail_noise = fbm(uv * detail_freq + Vec3::new(0.0, 0.0, time * 0.1), 4, 0.4, 2.5);
    if is_land {
        color = color.lerp(&Vec3::new(0.9, 0.9, 0.9), detail_noise * 0.2); // Picos nevados
    }

    // Capa 4: Atmósfera (efecto Fresnel)
    let normal = uv;
    let view_dir = Vec3::new(0.0, 0.0, 1.0); // Asumiendo cámara en Z
    let fresnel = (1.0 - normal.dot(&view_dir)).powf(4.0);
    let atmosphere_color = Vec3::new(0.5, 0.7, 1.0); // Azul claro
    color = color.lerp(&atmosphere_color, fresnel * 0.8);

    color.map(|x| x.max(0.0).min(1.0))
}

pub fn shade_gas_giant(point: Vec3, time: f32) -> Vec3 {
    let uv = point.normalize();
    let mut color;

    // Capa 1: Franjas de gases con seno y ruido
    let band_freq_y = 8.0;
    let band_speed = 0.2;
    let band_noise_freq = 15.0;
    
    let y_component = uv.y + time * band_speed * 0.1;
    let band_noise = noise(uv * band_noise_freq + Vec3::new(time * band_speed, 0.0, 0.0));
    let bands = (y_component * band_freq_y + band_noise * 2.0).sin();

    let band_color1 = Vec3::new(0.8, 0.7, 0.5); // Crema
    let band_color2 = Vec3::new(0.6, 0.4, 0.2); // Marrón
    color = band_color1.lerp(&band_color2, (bands * 0.5 + 0.5));

    // Capa 2: Ruido de alta frecuencia para textura gaseosa
    let gas_texture_noise = fbm(uv * 40.0 + Vec3::new(time * 0.3, 0.0, 0.0), 3, 0.5, 2.0);
    color = color.lerp(&Vec3::new(1.0, 1.0, 1.0), gas_texture_noise * 0.1);

    // Capa 3: Gran Tormenta (estilo Júpiter)
    let storm_pos = Vec3::new(0.0, -0.4, 0.0).normalize();
    let storm_radius = 0.3;
    let dist_to_storm = (uv - storm_pos).magnitude();

    if dist_to_storm < storm_radius {
        let storm_factor = 1.0 - (dist_to_storm / storm_radius);
        let storm_uv = (uv - storm_pos) / storm_radius;
        let storm_time = time * 1.5;
        let storm_angle = storm_uv.y.atan2(storm_uv.x) + storm_time;
        let storm_dist_center = storm_uv.magnitude();
        
        let storm_noise_freq = 5.0;
        let storm_noise = fbm(
            Vec3::new(storm_angle.cos(), storm_angle.sin(), 0.0) * storm_noise_freq * storm_dist_center,
            3, 0.5, 2.0
        );

        let storm_color1 = Vec3::new(0.9, 0.2, 0.1); // Rojo oscuro
        let storm_color2 = Vec3::new(1.0, 0.8, 0.6); // Naranja claro
        let storm_color = storm_color1.lerp(&storm_color2, storm_noise);
        color = color.lerp(&storm_color, storm_factor.powf(2.0));
    }

    // Capa 4: Oscurecimiento en los polos
    let polar_darkening = 1.0 - uv.y.abs().powf(3.0) * 0.5;
    color *= polar_darkening;

    color.map(|x| x.max(0.0).min(1.0))
}