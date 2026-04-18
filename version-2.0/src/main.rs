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
        .size(450, 450)
        .title("Circular Cava v2.0")
        .transparent() // Ventana transparente
        .undecorated()  // Sin bordes de ventana (tipo Glava)
        .vsync()
        .build();
    //Configuracion para que se comporte como un widget de escritorio

    rl.set_window_position(750, 300); // <--lugar donde esta su ubi 
    rl.add_window_state(ConfigFlags::FLAG_WINDOW_MOUSE_PASSTHROUGH);

    let mut vis = Visualizador::new();

    while !rl.window_should_close() {
        // Obtener datos actuales
        let datos_raw ={
            let lock = capturador.buffer.lock().unwrap();
            lock.clone()
        };

        let barras = vis.procesar_audio(&datos_raw, 80); // 80 barras circulares

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLANK); // Fondo totalmente transparente

        let centro = (255.0, 255.0);
        let radio_base = 100.0; //espacio para que el circulo se vea dentro

        let lineas = vis.calcular_puntos_circulo(&barras, centro, radio_base);

        for i in 0..lineas.len() {
            let (x1, y1, x2, y2) = lineas[i];
            
            let color_linea = if i % 2 == 0 { Color::VIOLET } else { Color::PURPLE };
            d.draw_line_ex(
                Vector2::new(x1, y1),
                Vector2::new(x2, y2),
                3.0, // Grosor de la barra
                color_linea // Color ricing inicial
            );
        }        
    }
}
