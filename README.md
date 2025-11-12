# Laboratorio de Shaders - Cuerpos Celestes

## Objetivo
Este laboratorio tiene como objetivo practicar la creación de shaders interesantes variando únicamente colores utilizando los parámetros disponibles, sin hacer uso de texturas o materiales externos.

## Descripción
Se han creado tres cuerpos celestes diferentes utilizando exclusivamente shaders:

1. **Estrella (Sol)** - Una estrella que sirve como el sol del sistema solar
2. **Planeta Rocoso** - Un planeta con características terrestres y superficie sólida
3. **Gigante Gaseoso** - Un planeta masivo compuesto principalmente de gases

## Características Implementadas

### Estrella (Sol)
- Patrones de llamaradas solares animadas
- Efectos de corona solar
- Gradientes de color que simulan la intensidad del calor
- Animación de superficie turbulenta

### Planeta Rocoso
- Patrones de continentes y océanos
- Efectos de atmósfera tenue
- Variaciones de terreno (montañas, llanuras)
- Colores naturales que simulan tierra y agua

### Gigante Gaseoso
- Bandas atmosféricas características
- Patrones de tormentas y remolinos
- Efectos de profundidad atmosférica
- Colores vibrantes y dinámicos

## Tecnologías Utilizadas
- WebGL
- GLSL (OpenGL Shading Language)
- JavaScript
- HTML5 Canvas

## Instrucciones de Ejecución

1. Clonar el repositorio:
```bash
git clone [URL del repositorio]
```

2. Abrir el archivo `index.html` en un navegador web compatible con WebGL

3. Utilizar los botones de la interfaz para alternar entre los diferentes shaders:
   - Botón "Sol" - Muestra la estrella
   - Botón "Planeta Rocoso" - Muestra el planeta terrestre
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
Nery Molina
