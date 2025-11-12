# Laboratorio 5 - Shaders Procedurales de Cuerpos Celestes

## Objetivo
Este laboratorio tiene como objetivo practicar la creación de shaders interesantes variando únicamente colores utilizando los parámetros disponibles, **sin hacer uso de texturas o materiales externos**. Todo el coloreado se genera proceduralmente mediante cálculos matemáticos en el shader.

## Descripción
Se han creado tres cuerpos celestes diferentes utilizando exclusivamente shaders procedurales:

1. **Estrella (Sol)** - Una estrella brillante y pulsante con efectos de llamaradas
2. **Planeta Rocoso** - Un planeta similar a la Tierra con continentes, océanos y atmósfera
3. **Gigante Gaseoso** - Un planeta masivo con bandas de gases y una gran tormenta

## Tecnologías Utilizadas
- **Rust** - Lenguaje de programación principal
- **nalgebra_glm** - Matemáticas vectoriales y matriciales
- **minifb** - Ventana y manejo de píxeles
- **Software Rendering** - Renderizado píxel por píxel sin OpenGL/Vulkan

## Implementación de Shaders

### 1. Estrella (Sol)
**Shader:** `shade_star(point: Vec3, time: f32) -> Vec3`

#### Capas implementadas (4 capas):
1. **Núcleo brillante:** Gradiente radial desde el centro con color amarillo intenso que se atenúa hacia los bordes
2. **Gradiente de temperatura:** Transición de color desde amarillo brillante en el núcleo hacia naranja en la superficie
3. **Turbulencia de llamaradas (FBM):** Ruido procedural fractal que simula la superficie convectiva y las llamaradas solares. Se anima con el tiempo para crear un efecto dinámico de plasma
4. **Pulsación animada:** Modulación sinusoidal del brillo que crea un efecto de pulsación sutil de la estrella

#### Técnicas utilizadas:
- Gradientes radiales con funciones de potencia
- Fractional Brownian Motion (FBM) para turbulencia
- Interpolación lineal (lerp) entre colores
- Animación temporal con funciones sinusoidales

### 2. Planeta Rocoso
**Shader:** `shade_rocky(point: Vec3, time: f32) -> Vec3`

#### Capas implementadas (4 capas):
1. **Ruido base (FBM):** Generación de un mapa de altura usando ruido procedural con 6 octavas para definir la topografía del planeta
2. **Continentes y océanos:** Separación por umbral (threshold) del ruido base. Los valores por encima del umbral son tierra, los valores por debajo son agua. Interpolación de colores para océanos profundos/superficiales y tierras bajas/altas
3. **Detalles de terreno:** Capa de ruido de alta frecuencia que agrega detalles finos como montañas nevadas en las zonas terrestres
4. **Atmósfera (Efecto Fresnel):** Halo azulado en los bordes del planeta que simula la dispersión atmosférica. La intensidad aumenta en el limbo del planeta

#### Técnicas utilizadas:
- FBM con múltiples octavas para terreno natural
- Threshold para separación tierra/agua
- Interpolación de colores basada en altura
- Efecto Fresnel para atmósfera realista

### 3. Gigante Gaseoso
**Shader:** `shade_gas_giant(point: Vec3, time: f32) -> Vec3`

#### Capas implementadas (4 capas):
1. **Franjas de gases:** Bandas horizontales generadas con funciones sinusoidales combinadas con ruido. Se desplazan con el tiempo simulando el movimiento atmosférico. Alternancia entre colores crema y marrón
2. **Textura gaseosa:** Ruido FBM de alta frecuencia que agrega una textura granulada sobre las bandas, dando una apariencia más caótica y turbulenta
3. **Gran Tormenta:** Vórtice circular en una posición específica del planeta. El ruido se aplica de forma rotacional para simular el ojo de una tormenta gigante (similar a la Gran Mancha Roja de Júpiter). Colores rojizos y anaranjados que contrastan con las bandas
4. **Oscurecimiento polar:** Atenuación del brillo en los polos para simular iluminación menos directa y añadir realismo esférico

#### Técnicas utilizadas:
- Funciones seno para patrones de bandas
- FBM multicapa para turbulencia atmosférica
- Coordenadas polares para la tormenta giratoria
- Gradientes basados en latitud

## Funciones Auxiliares de Ruido

### `noise(p: Vec3) -> f32`
Implementa ruido de Perlin interpolado usando coordenadas 3D. Genera valores pseudoaleatorios suaves y continuos en el espacio 3D.

### `fbm(p: Vec3, octaves: i32, persistence: f32, lacunarity: f32) -> f32`
Fractional Brownian Motion - Combina múltiples octavas de ruido con diferentes frecuencias y amplitudes para crear patrones naturales y complejos.

**Parámetros:**
- `octaves`: Número de capas de ruido a combinar
- `persistence`: Control de amplitud entre octavas (típicamente 0.5)
- `lacunarity`: Control de frecuencia entre octavas (típicamente 2.0)

## Controles

### Cambio de Shader
- **Tecla 1:** Mostrar Estrella (Sol)
- **Tecla 2:** Mostrar Planeta Rocoso
- **Tecla 3:** Mostrar Gigante Gaseoso

### Rotación
- **A/D:** Rotar en eje Y (horizontal)
- **W/S:** Rotar en eje X (vertical)

### General
- **ESC:** Salir de la aplicación

## Compilación y Ejecución

1. **Clonar el repositorio:**
```bash
git clone [URL del repositorio]
cd Lab-5-Shaders
```

2. **Compilar el proyecto:**
```bash
cargo build --release
```

3. **Ejecutar:**
```bash
cargo run --release
```

## Estructura del Proyecto

```
Lab-5-Shaders/
├── src/
│   ├── main.rs          # Bucle principal y lógica de renderizado
│   ├── shaders.rs       # Shaders procedurales y funciones de ruido
│   ├── triangle.rs      # Rasterización de triángulos
│   ├── vertex.rs        # Estructura de vértices
│   ├── fragment.rs      # Estructura de fragmentos
│   ├── framebuffer.rs   # Búfer de píxeles
│   ├── obj.rs           # Cargador de modelos OBJ
│   └── color.rs         # Estructura de colores
├── assets/
│   └── sphere.obj       # Modelo de esfera
├── Cargo.toml           # Dependencias del proyecto
└── README.md            # Este archivo
```

## Screenshots

### Estrella (Sol)
![Estrella](screenshots/star.png)
*Estrella con núcleo brillante, turbulencia y pulsación animada*

### Planeta Rocoso
![Planeta Rocoso](screenshots/rocky.png)
*Planeta con continentes, océanos y atmósfera*

### Gigante Gaseoso
![Gigante Gaseoso](screenshots/gas_giant.png)
*Gigante gaseoso con bandas atmosféricas y gran tormenta*

## Criterios de Evaluación Cumplidos

✅ **30 puntos** - Creatividad del diseño (3 cuerpos celestes únicos y visualmente distintos)  
✅ **40 puntos** - Complejidad de shaders (4 capas por shader)
- Estrella: 4 capas ✓
- Planeta Rocoso: 4 capas ✓
- Gigante Gaseoso: 4 capas ✓

✅ **Sistema de cambio de shaders** - Teclas 1, 2, 3 para cambiar entre planetas  
✅ **Animación temporal** - Todos los shaders utilizan el parámetro `time` para efectos dinámicos  
✅ **Sin texturas ni materiales** - 100% generación procedural de colores  

## Autor

**Nery**  
Universidad del Valle de Guatemala  
Gráficas por Computadora - 2025

## Licencia

Este proyecto es parte de un laboratorio académico.
   - Botón "Gigante Gaseoso" - Muestra el planeta gaseoso

## Estructura del Proyecto
```
/
├── index.html          # Archivo principal
├── js/
│   ├── renderer.js     # Motor de renderizado
│   ├── shaders.js      # Gestión de shaders
│   └── main.js         # Lógica principal
├── glsl/
│   ├── star.frag       # Shader de la estrella
│   ├── rocky.frag      # Shader del planeta rocoso
│   └── gasgiant.frag   # Shader del gigante gaseoso
└── README.md
```

## Requisitos del Sistema
- Navegador web moderno con soporte para WebGL
- JavaScript habilitado

## Capturas de Pantalla

### Estrella (Sol)
![Sol](screenshots/star.png)

### Planeta Rocoso
![Planeta Rocoso](screenshots/rocky_planet.png)

### Gigante Gaseoso
![Gigante Gaseoso](screenshots/gas_giant.png)

## Notas de Desarrollo
- Todos los efectos visuales se logran exclusivamente mediante programación de shaders
- No se utilizaron texturas externas ni materiales predefinidos
- Los patrones y colores se generan proceduralmente usando noise functions y algoritmos matemáticos

## Autor
Nery Molina 23218
