mod captura_audio;
mod visualizador;

use captura_audio::AudioCapturer;
use visualizador::Visualizador;
use std::time::Duration;
use std::thread;


fn main() {
    // Esta cosa inicia el audio
    let capturador = AudioCapturer::new();
    capturador.iniciar();

    //conf del canvas sgv
    let mut vis = Visualizador::new();
    let svg_size = 400.0;
    let centro = (svg_size /2.0, svg_size /2.0);
    let radio_base = 120.0;
    let num_barras = 60;

    loop {
        let datos_raw = capturador.buffer.lock().unwrap().clone();
        let barras = vis.procesar_audio(&datos_raw, num_barras);
        let lineas = vis.calcular_puntos_circulo(&barras, centro, radio_base);

        let mut svg = format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">", svg_size, svg_size);

        for (x1, y1, x2, y2) in lineas {
            svg.push_str(&format!(
        //manejo las cordenadas desde aqui consumiendo el archivo de vizualizador.rs
                    "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#a855f7\" stroke-width=\"4\" stroke-linecap=\"round\" />",
                    x1, y1, x2, y2
            ));
        } 
        svg.push_str("</svg>");
        // la wea que imprime el stdout
        std::fs::write("/tmp/espectro.svg", &svg).unwrap();
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        println!("/tmp/espectro.svg?v={}", timestamp);
        thread::sleep(Duration::from_millis(16));
    }
}
