mod captura_audio;
mod visualizador;

use raylib::prelude::*;
use captura_audio::AudioCapturer;
use visualizador::Visualizador;

fn main() {
    // 1. Inicializar Audio
    let capturador = AudioCapturer::new();
    capturador.iniciar();

    // 2. Inicializar Gráficos (Raylib)
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("Circular Cava v2.0")
        .transparent() // Ventana transparente
        .undecorated()  // Sin bordes de ventana (tipo Glava)
        .build();

    let mut vis = Visualizador::new();

    while !rl.window_should_close() {
        // Obtener datos actuales
        let datos_raw = capturador.buffer.lock().unwrap().clone();
        let barras = vis.procesar_audio(&datos_raw, 80); // 80 barras circulares

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLANK); // Fondo totalmente transparente

        let centro = (400.0, 400.0);
        let radio_base = 120.0;

        let lineas = vis.calcular_puntos_circulo(&barras, centro, radio_base);

        for (x1, y1, x2, y2) in lineas {
            d.draw_line_ex(
                Vector2::new(x1, y1),
                Vector2::new(x2, y2),
                3.0, // Grosor de la barra
                Color::VIOLET // Color ricing inicial
            );
        }
    }
}
