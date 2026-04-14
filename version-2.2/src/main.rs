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

        let mut d_path = String::new();
        for ( x1, x2, y1, y2 ) in lineas {
            d_path.push_str(&format!("M {:.1}, {:.1} L {:.1}. {:.1} ", x1, y1 , x1, x2));
        }
        println!("{}", serde_json::json!({"path": d_path }).to_string());
        thread::sleep(Duration::from_millis(16));
    }
}
