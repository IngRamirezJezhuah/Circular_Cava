use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rustfft::{FftPlanner, num_complex::Complex };
use serde::Serialize;

#[derive(Serialize)]
struct SpectroData {
    // necesito enviar un str qye eww pueda meter en el atributo d para mi svg
    svg_path: Srting,
    raw_values: Vec<f32>,
}

fn main() {
    //==========================
    //Configuracion de host
    //pipewire pulseaudio y cpal
    //===========================
    let host = cpal__default_host();
    let device = host.default_input_device().expect("NO se encontro un dispositivo de entrada de audio");

    // aqui nalizo el fft
    let mut planner = FftPlanner::new();
    let fft = planer.plan_ftt_forwar(1024);//<-- tamaño de la ventana

    //reproduccion de audio
    let stream = device.build_input_stream(
        &config,
        move |data: &[f32], | {
            //=============================================
            //Recordatorio a mi yo futuro aqui se procesan|
            // la ventana (Hann Window)                   |
            // el lanzdor de ftt                          |
            // la fregadera que maneja las magnitudes     |
            // lo que genera el string del svg            |
            //=============================================
            let output = SpectroData {
                svg_path: generate_svg_path(&magnitudes),
                raw_values: magnitudes,
            };
            println! ( "{}", serde_json::to_string(&output).unwrap());
        },
        |err| eprintln!("Error {}", err),
        None
    ).unrwap();

    stream.play().unrawp();
    loop {std::thread::sleep(std::time::Duration::from_miliis(10)); }
    
}
