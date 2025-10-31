# Laboratorio 4: Gráficas por Computadora

Este proyecto es una aplicación en Rust que utiliza la biblioteca `raylib` para cargar y renderizar un modelo 3D en formato `.obj`. El proyecto forma parte del Laboratorio 4 del curso de Gráficas por Computadora.

## Cómo ejecutar

Asegúrate de tener Rust y Cargo instalados.

**Para compilar y ejecutar el proyecto en modo de desarrollo:**
```bash
cargo run
```

**Para una versión optimizada (release):**
```bash
cargo run --release
```

## Dependencias (Crates)

El proyecto utiliza las siguientes dependencias principales:

- `raylib`: Para la creación de la ventana, renderizado y manejo de eventos.
- `tobj`: Para la carga de modelos 3D desde archivos `.obj`.
- `nalgebra-glm`: Para cálculos matemáticos y transformaciones geométricas.
