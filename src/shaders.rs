// Importa el tipo Color del módulo de framebuffer y Vec3 de nalgebra_glm.
use crate::framebuffer::Color;
use nalgebra_glm::Vec3;
use std::f32::consts::PI;

// Define un trait (una interfaz) para los sombreadores de planetas.
// Cualquier sombreador que implemente este trait debe tener una función `fragment`.
pub trait PlanetShader {
    // Calcula el color de un fragmento (píxel) en una posición y normal dadas.
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color;
}

// --- FUNCIONES DE UTILIDAD ---

// Genera un valor de ruido pseudoaleatorio basado en coordenadas 3D.
#[inline]
fn noise(x: f32, y: f32, z: f32) -> f32 {
    ((x * 12.9898 + y * 78.233 + z * 45.164).sin() * 43758.5453).fract()
}

// Interpola suavemente entre dos valores.
#[inline]
fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

// Calcula el efecto Fresnel, que hace que los bordes de un objeto sean más reflectantes.
#[inline]
fn fresnel(view: &Vec3, normal: &Vec3, power: f32) -> f32 {
    (1.0 - view.dot(normal).abs()).powf(power)
}

// Interpola linealmente entre dos vectores 3D.
#[inline]
fn mix_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a * (1.0 - t) + b * t
}

// Genera turbulencia sumando múltiples capas de ruido (octavas).
#[inline]
fn turbulence(p: Vec3, octaves: i32) -> f32 {
    let mut sum = 0.0;
    let mut freq = 1.0;
    let mut amp = 1.0;
    for _ in 0..octaves {
        sum += amp * noise(p.x * freq, p.y * freq, p.z * freq).abs();
        freq *= 2.0;
        amp *= 0.5;
    }
    sum
}

// --- SOMBREADOR PARA PLANETA ROCOSO ---

pub struct RockyPlanet;

impl PlanetShader for RockyPlanet {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, _time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Define el color base según la altura (simulando montañas, tierra, costas y océanos).
        let height = normalized_pos.y;
        let base_color = if height > 0.4 {
            Vec3::new(0.7, 0.5, 0.3) // Montañas
        } else if height > 0.0 {
            Vec3::new(0.4, 0.6, 0.3) // Tierra
        } else if height > -0.3 {
            Vec3::new(0.8, 0.7, 0.5) // Costa
        } else {
            Vec3::new(0.1, 0.3, 0.6) // Océano
        };

        // Agrega ruido para simular continentes y variaciones en el terreno.
        let continent_noise = turbulence(normalized_pos * 3.0, 3);
        let color_variation = mix_vec3(base_color, base_color * 0.8, continent_noise * 0.3);

        // Añade cráteres a la superficie.
        let crater_pattern = (normalized_pos.x * 15.0).sin() * (normalized_pos.z * 15.0).cos();
        let crater_factor = smoothstep(0.85, 0.95, crater_pattern.abs());
        let crater_color = mix_vec3(color_variation, Vec3::new(0.3, 0.3, 0.35), crater_factor * 0.3);

        // Aplica iluminación difusa y especular (brillo en los océanos).
        let light_dir = Vec3::new(1.0, 0.5, 1.0).normalize();
        let diffuse = normal.dot(&light_dir).abs() * 0.6 + 0.4;
        let specular = if height < 0.0 {
            let view_dir = Vec3::new(0.0, 0.0, 1.0);
            let half_vec = (light_dir + view_dir).normalize();
            normal.dot(&half_vec).max(0.0).powf(32.0) * 0.4
        } else {
            0.0
        };

        let final_color = crater_color * diffuse + Vec3::new(1.0, 1.0, 1.0) * specular;
        Color::from_vec3(final_color)
    }
}

// --- SOMBREADOR PARA GIGANTE GASEOSO ---

pub struct GasGiant;

impl PlanetShader for GasGiant {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Crea bandas de colores basadas en la latitud.
        let latitude = normalized_pos.y;
        let band_count = 12.0;
        let band = ((latitude + 1.0) * 0.5 * band_count).floor();
        let band_colors = [
            Vec3::new(0.9, 0.7, 0.5),
            Vec3::new(0.8, 0.5, 0.3),
            Vec3::new(0.7, 0.4, 0.2),
            Vec3::new(0.6, 0.3, 0.2),
        ];
        let base_color = band_colors[band as usize % band_colors.len()];

        // Agrega turbulencia animada para simular la atmósfera gaseosa.
        let longitude = normalized_pos.z.atan2(normalized_pos.x) / (2.0 * PI);
        let turb = noise(longitude * 8.0 + time * 0.3, latitude * 5.0, time * 0.1);
        let turbulent_color = mix_vec3(base_color, base_color * 1.2, turb * 0.4);

        // Añade una "gran mancha roja" similar a la de Júpiter.
        let spot_center = Vec3::new(0.5, -0.2, 0.0).normalize();
        let dist_to_spot = (normalized_pos - spot_center).magnitude();
        let spot_factor = smoothstep(0.25, 0.15, dist_to_spot);
        let spot_color = Vec3::new(0.8, 0.2, 0.1);
        let color_with_spot = mix_vec3(turbulent_color, spot_color, spot_factor * 0.7);

        // Aplica una iluminación suave para dar forma al planeta.
        let light_dir = Vec3::new(1.0, 0.3, 1.0).normalize();
        let terminator = smoothstep(0.0, 0.5, normal.dot(&light_dir).abs());
        let final_color = color_with_spot * (0.3 + terminator * 0.7);

        Color::from_vec3(final_color)
    }
}

// --- SOMBREADOR PARA PLANETA CRISTALINO ---

pub struct CrystalPlanet;

impl PlanetShader for CrystalPlanet {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Crea un patrón geométrico hexagonal en la superficie.
        let hex_x = normalized_pos.x * 8.0;
        let hex_z = normalized_pos.z * 8.0;
        let hex_pattern = ((hex_x * 2.0).sin() + (hex_z * 2.0).sin() + ((hex_x + hex_z) * 2.0).sin()) / 3.0;
        let geo_factor = smoothstep(-0.1, 0.1, hex_pattern);

        // Genera un color iridiscente que cambia con la posición y el tiempo.
        let hue_shift = time * 0.5 + normalized_pos.y * 2.0;
        let base_hue = (hue_shift % (2.0 * PI)) / (2.0 * PI);
        let iridescent_color = hsv_to_rgb(base_hue, 0.7, 0.9);

        // Añade líneas de energía pulsantes.
        let pulse = ((time * 3.0).sin() * 0.5 + 0.5) * 0.3;
        let energy_lines = ((normalized_pos.y * 20.0 + time * 2.0).sin() * 0.5 + 0.5) * pulse;
        let pulsing_color = iridescent_color * (1.0 + energy_lines);

        // Aplica un efecto Fresnel para que los bordes brillen.
        let view_dir = Vec3::new(0.0, 0.0, 1.0);
        let fresnel_power = fresnel(&view_dir, normal, 3.0);
        let fresnel_color = Vec3::new(0.8, 0.9, 1.0);
        let final_color = mix_vec3(pulsing_color * (0.5 + geo_factor * 0.5), fresnel_color, fresnel_power * 0.6);

        Color::from_vec3(final_color)
    }
}

// Convierte un color de formato HSV (Tono, Saturación, Valor) a RGB.
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Vec3 {
    let c = v * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Vec3::new(r + m, g + m, b + m)
}

// --- SOMBREADOR PARA PLANETA DE LAVA ---

pub struct LavaPlanet;

impl PlanetShader for LavaPlanet {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Crea un patrón de grietas animadas en la superficie.
        let crack_pattern = turbulence(normalized_pos * 5.0, 3);
        let animated_crack = crack_pattern + (time * 0.5).sin() * 0.3;
        let is_lava = animated_crack > 0.6;

        // Define el color base: lava brillante o roca oscura.
        let base_color = if is_lava {
            let intensity = (time * 4.0 + crack_pattern * 10.0).sin() * 0.5 + 0.5;
            mix_vec3(Vec3::new(1.0, 0.3, 0.0), Vec3::new(1.0, 0.8, 0.0), intensity)
        } else {
            Vec3::new(0.15, 0.1, 0.08)
        };

        // La lava emite su propia luz, mientras que la roca se ilumina de forma difusa.
        let light_dir = Vec3::new(1.0, 1.0, 1.0).normalize();
        let diffuse = normal.dot(&light_dir).abs() * 0.5 + 0.5;

        let final_color = if is_lava {
            base_color * 1.5 // La lava es más brillante.
        } else {
            base_color * diffuse
        };

        Color::from_vec3(final_color)
    }
}

// --- SOMBREADOR PARA MUNDO CONGELADO ---

pub struct IcePlanet;

impl PlanetShader for IcePlanet {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, _time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Crea un patrón de hielo y cristales usando turbulencia.
        let ice_pattern = turbulence(normalized_pos * 10.0, 4);
        let crystal_factor = smoothstep(0.4, 0.6, ice_pattern);
        let base_color = mix_vec3(Vec3::new(0.7, 0.8, 0.95), Vec3::new(0.5, 0.6, 0.8), crystal_factor);

        // Aplica iluminación difusa y un fuerte brillo especular para simular el hielo.
        let light_dir = Vec3::new(1.0, 1.0, 1.0).normalize();
        let diffuse = normal.dot(&light_dir).abs() * 0.5 + 0.5;
        let view_dir = Vec3::new(0.0, 0.0, 1.0);
        let half_vec = (light_dir + view_dir).normalize();
        let specular = normal.dot(&half_vec).max(0.0).powf(64.0);
        let final_color = base_color * diffuse + Vec3::new(1.0, 1.0, 1.0) * specular * 0.8;

        Color::from_vec3(final_color)
    }
}

// --- SOMBREADOR PARA ANILLOS ---

pub struct RingShader;

impl PlanetShader for RingShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let dist_from_center = (pos.x * pos.x + pos.z * pos.z).sqrt();

        // Crea bandas de colores alternos en el anillo.
        let band_count = 15.0;
        let band = (dist_from_center * band_count).floor();
        let _band_mix = (dist_from_center * band_count).fract();

        // Colores alternados para las bandas
        let color1 = Vec3::new(0.8, 0.7, 0.6);
        let color2 = Vec3::new(0.6, 0.5, 0.4);
        let base_color = if band as i32 % 2 == 0 {
            color1
        } else {
            color2
        };

        // Agrega ruido para dar textura de partículas al anillo.
        let noise_val = noise(pos.x * 20.0, time * 0.1, pos.z * 20.0);
        let color_with_noise = base_color * (0.8 + noise_val * 0.4);

        // Aplica iluminación simple y transparencia en los bordes del anillo.
        let light_dir = Vec3::new(1.0, 0.5, 1.0).normalize();
        let n_dot_l = normal.dot(&light_dir).abs();
        let lit_color = color_with_noise * (0.5 + n_dot_l * 0.5);

        // Transparencia en los bordes
        let alpha_inner = smoothstep(0.0, 0.05, dist_from_center - 1.3);
        let alpha_outer = smoothstep(2.2, 2.0, dist_from_center);
        let alpha = alpha_inner * alpha_outer;

        // Simula la transparencia devolviendo un color oscuro si el alfa es bajo.
        if alpha < 0.3 {
            Color::BLACK
        } else {
            Color::from_vec3(lit_color * alpha)
        }
    }
}

// --- SOMBREADOR PARA LA LUNA ---

pub struct MoonShader;

impl PlanetShader for MoonShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, _time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Crea una superficie rocosa con cráteres.
        let crater_noise = turbulence(normalized_pos * 8.0, 3);
        let crater = smoothstep(0.6, 0.8, crater_noise);
        let base_color = Vec3::new(0.4, 0.4, 0.45);
        let crater_color = Vec3::new(0.25, 0.25, 0.28);
        let surface_color = mix_vec3(base_color, crater_color, crater * 0.6);

        // Agrega una textura de ruido fino para dar detalle a la superficie.
        let detail = noise(normalized_pos.x * 30.0, normalized_pos.y * 30.0, normalized_pos.z * 30.0);
        let detailed_color = surface_color * (0.9 + detail * 0.2);

        // Aplica iluminación difusa para dar forma a la luna.
        let light_dir = Vec3::new(1.0, 0.5, 1.0).normalize();
        let diffuse = normal.dot(&light_dir).abs() * 0.7 + 0.3;

        Color::from_vec3(detailed_color * diffuse)
    }
}