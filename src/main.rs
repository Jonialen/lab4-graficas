// Importaciones de módulos locales para organizar el código.
mod framebuffer; // Maneja el búfer de fotogramas para dibujar píxeles.
mod mesh; // Define estructuras y funciones para manejar mallas de objetos 3D.
mod renderer; // Contiene la lógica de renderizado principal.
mod shaders; // Define los sombreadores para diferentes apariencias de planetas.

// Usamos tipos y funciones de los módulos importados y de bibliotecas externas.
use framebuffer::{Color, Framebuffer}; // Para colores y el búfer de fotogramas.
use mesh::ObjMesh; // Para la estructura de mallas de objetos.
use nalgebra_glm::{look_at, perspective, rotate, Mat4, Vec3}; // Para matemáticas de gráficos 3D.
use raylib::prelude::*; // Para la creación de la ventana y manejo de eventos.
use renderer::Renderer; // El renderizador que dibujará todo.
use shaders::*; // Importa todos los sombreadores definidos.

// Constantes para el tamaño de la ventana.
const WIDTH: usize = 800; // Ancho de la ventana en píxeles.
const HEIGHT: usize = 600; // Alto de la ventana en píxeles.

// Estructura que representa un objeto que se puede renderizar en la escena.
struct RenderObject {
    mesh: ObjMesh, // La malla 3D del objeto.
    shader: Box<dyn PlanetShader>, // El sombreador que define cómo se colorea el objeto.
    position: Vec3, // La posición del objeto en el espacio 3D.
    scale: f32, // El tamaño del objeto.
    rotation_speed: f32, // La velocidad a la que rota el objeto.
    rotation_axis: Vec3, // El eje sobre el cual rota el objeto.
}

impl RenderObject {
    // Constructor para crear un nuevo objeto renderizable.
    fn new(
        mesh: ObjMesh,
        shader: Box<dyn PlanetShader>,
        position: Vec3,
        scale: f32,
    ) -> Self {
        RenderObject {
            mesh,
            shader,
            position,
            scale,
            rotation_speed: 1.0, // Velocidad de rotación por defecto.
            rotation_axis: Vec3::new(0.0, 1.0, 0.0), // Eje de rotación por defecto (eje Y).
        }
    }

    // Calcula y devuelve la matriz de modelo para este objeto, que incluye traslación, rotación y escala.
    fn get_model_matrix(&self, time: f32) -> Mat4 {
        let mut transform = Mat4::identity(); // Empezamos con una matriz de identidad.

        // Aplicamos la traslación para mover el objeto a su posición.
        transform = nalgebra_glm::translate(&transform, &self.position);

        // Aplicamos la rotación, que cambia con el tiempo para animar el objeto.
        transform = rotate(&transform, time * self.rotation_speed, &self.rotation_axis);

        // Aplicamos la escala para ajustar el tamaño del objeto.
        transform = nalgebra_glm::scale(&transform, &Vec3::new(self.scale, self.scale, self.scale));

        transform // Devolvemos la matriz de transformación final.
    }
}

// La función principal que se ejecuta al iniciar el programa.
fn main() {
    println!("Iniciando aplicación...");

    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Planetas con Luna y Anillos - Software Renderer")
        .build();

    rl.set_target_fps(60);

    println!("Generando geometría...");
    let sphere_mesh = ObjMesh::create_sphere(1.0, 50, 50);
    
    // Intenta cargar el modelo .obj, si falla usa la esfera procedural
    let obj_sphere = match ObjMesh::load_from_obj("assets/sphere.obj") {
        Ok(mesh) => {
            println!("✓ sphere.obj cargado exitosamente");
            Some(mesh)
        }
        Err(e) => {
            println!("⚠ No se pudo cargar sphere.obj: {}", e);
            println!("  Usando solo esfera procedural");
            None
        }
    };
    
    let ring_mesh = ObjMesh::create_ring(1.3, 2.0, 100);

    // Variable para controlar qué malla usar
    let mut use_obj_model = false;

    // Función helper para obtener la malla actual
    let get_sphere = |use_obj: bool| -> ObjMesh {
        if use_obj && obj_sphere.is_some() {
            obj_sphere.as_ref().unwrap().clone()
        } else {
            sphere_mesh.clone()
        }
    };

    // Función para crear todas las escenas
    let create_scenes = |use_obj: bool| -> Vec<Vec<RenderObject>> {
        let current_sphere = get_sphere(use_obj);
        
        vec![
            // Escena 0: Planeta Rocoso
            vec![RenderObject::new(
                current_sphere.clone(),
                Box::new(RockyPlanet),
                Vec3::new(0.0, 0.0, 0.0),
                1.0,
            )],
            
            // Escena 1: Gigante Gaseoso + Anillos
            vec![
                RenderObject::new(
                    current_sphere.clone(),
                    Box::new(GasGiant),
                    Vec3::new(0.0, 0.0, 0.0),
                    1.2,
                ),
                RenderObject {
                    mesh: ring_mesh.clone(),
                    shader: Box::new(RingShader),
                    position: Vec3::new(0.0, 0.0, 0.0),
                    scale: 1.0,
                    rotation_speed: 0.3,
                    rotation_axis: Vec3::new(0.3, 1.0, 0.1).normalize(),
                },
            ],
            
            // Escena 2: Planeta Cristalino
            vec![RenderObject::new(
                current_sphere.clone(),
                Box::new(CrystalPlanet),
                Vec3::new(0.0, 0.0, 0.0),
                1.0,
            )],
            
            // Escena 3: Planeta de Lava + Luna
            vec![
                RenderObject::new(
                    current_sphere.clone(),
                    Box::new(LavaPlanet),
                    Vec3::new(0.0, 0.0, 0.0),
                    1.0,
                ),
                RenderObject {
                    mesh: current_sphere.clone(),
                    shader: Box::new(MoonShader),
                    position: Vec3::new(0.0, 0.0, 0.0),
                    scale: 0.3,
                    rotation_speed: 0.5,
                    rotation_axis: Vec3::new(0.0, 1.0, 0.0),
                },
            ],
            
            // Escena 4: Mundo Congelado + Luna
            vec![
                RenderObject::new(
                    current_sphere.clone(),
                    Box::new(IcePlanet),
                    Vec3::new(0.0, 0.0, 0.0),
                    1.0,
                ),
                RenderObject {
                    mesh: current_sphere.clone(),
                    shader: Box::new(MoonShader),
                    position: Vec3::new(0.0, 0.0, 0.0),
                    scale: 0.25,
                    rotation_speed: 0.3,
                    rotation_axis: Vec3::new(0.0, 1.0, 0.0),
                },
            ],
        ]
    };

    // Crea las escenas iniciales
    let mut scenes = create_scenes(use_obj_model);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let renderer = Renderer::new(WIDTH, HEIGHT);

    println!("Creando textura...");
    let initial_image = Image::gen_image_color(
        WIDTH as i32,
        HEIGHT as i32,
        raylib::color::Color::BLACK,
    );

    let mut texture = rl
        .load_texture_from_image(&thread, &initial_image)
        .expect("No se pudo crear textura");

    let shader_names = vec![
        "1: Planeta Rocoso",
        "2: Gigante Gaseoso + Anillos",
        "3: Planeta Cristalino",
        "4: Planeta de Lava + Luna",
        "5: Mundo Congelado + Luna",
    ];

    let mut current_scene = 0;
    let mut paused = false;
    let mut paused_time = 0.0f32;
    let mut last_active_time = 0.0f32;

    println!("Entrando al loop principal...");

    while !rl.window_should_close() {
        let current_real_time = rl.get_time() as f32;
        
        let time = if paused {
            paused_time
        } else {
            last_active_time + (current_real_time - last_active_time)
        };

        // Cambio de escena
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) { current_scene = 0; }
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) { current_scene = 1; }
        if rl.is_key_pressed(KeyboardKey::KEY_THREE) { current_scene = 2; }
        if rl.is_key_pressed(KeyboardKey::KEY_FOUR) { current_scene = 3; }
        if rl.is_key_pressed(KeyboardKey::KEY_FIVE) { current_scene = 4; }
        
        // Toggle entre esfera procedural y .obj con la tecla M
        if rl.is_key_pressed(KeyboardKey::KEY_M) && obj_sphere.is_some() {
            use_obj_model = !use_obj_model;
            scenes = create_scenes(use_obj_model);
            println!("Cambiando a: {}", 
                if use_obj_model { "sphere.obj" } else { "Esfera Procedural" });
        }
        
        // Pausa
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            if paused {
                let pause_duration = current_real_time - paused_time;
                last_active_time = current_real_time - pause_duration;
                paused = false;
            } else {
                paused_time = time;
                paused = true;
            }
        }

        if !paused {
            last_active_time = time;
        }

        // Actualizar órbitas de lunas
        let orbit_radius = 2.5;
        let orbit_speed = 0.5;

        if current_scene == 3 || current_scene == 4 {
            if let Some(moon) = scenes[current_scene].get_mut(1) {
                moon.position = Vec3::new(
                    (time * orbit_speed).cos() * orbit_radius,
                    (time * orbit_speed * 0.7).sin() * 0.3,
                    (time * orbit_speed).sin() * orbit_radius,
                );
            }
        }
        
        let view_matrix = look_at(
            &Vec3::new(0.0, 0.0, 3.5),
            &Vec3::new(0.0, 0.0, 0.0),
            &Vec3::new(0.0, 1.0, 0.0),
        );

        let projection_matrix = perspective(
            WIDTH as f32 / HEIGHT as f32,
            60.0_f32.to_radians(),
            0.1,
            100.0,
        );

        framebuffer.clear(Color::BLACK);

        for obj in &scenes[current_scene] {
            let model_matrix = obj.get_model_matrix(time);

            renderer.render_mesh(
                &mut framebuffer,
                &obj.mesh,
                obj.shader.as_ref(),
                &model_matrix,
                &view_matrix,
                &projection_matrix,
                time,
            );
        }

        if let Err(e) = texture.update_texture(framebuffer.as_bytes()) {
            eprintln!("Error actualizando textura: {:?}", e);
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK.to_raylib());
        d.draw_texture(&texture, 0, 0, raylib::color::Color::WHITE);

        d.draw_fps(10, 10);

        let status = if paused { " [PAUSADO]" } else { "" };
        d.draw_text(
            &format!("{}{}", shader_names[current_scene], status),
            10,
            35,
            20,
            raylib::color::Color::WHITE,
        );

        // Mostrar qué tipo de malla se está usando
        let mesh_type = if use_obj_model { 
            "Modo: sphere.obj" 
        } else { 
            "Modo: Procedural" 
        };
        d.draw_text(
            mesh_type,
            10,
            60,
            16,
            raylib::color::Color::YELLOW,
        );

        // Controles actualizados
        let controls = if obj_sphere.is_some() {
            "Controles: 1-5 = Planetas, SPACE = Pausa, M = Cambiar Malla, ESC = Salir"
        } else {
            "Controles: 1-5 = Planetas, SPACE = Pausa, ESC = Salir"
        };
        
        d.draw_text(
            controls,
            10,
            HEIGHT as i32 - 25,
            16,
            raylib::color::Color::LIGHTGRAY,
        );
    }

    println!("Cerrando aplicación...");
}