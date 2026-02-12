use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

pub struct AudioCapturer {
    pub buffer: Arc<Mutex<Vec<f32>>>,
}

impl AudioCapturer {
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(vec![0.0; 2048])),
//este buffer es de 2048 que significa, bueno es la resolucion de la fft
        }
    }

    pub fn iniciar(&self) {
        let host = cpal::default_host();
        // Buscamos el dispositivo de "Loopback" o Monitor para capturar lo que suena
        let device = host.default_input_device()
            .expect("No se encontró dispositivo de entrada (asegúrate de tener un monitor de PulseAudio activo)");

        let config = device.default_input_config().expect("Error al obtener la config del audio");
        let buffer_clone = Arc::clone(&self.buffer);

    std::thread::spawn(move || {
        let stream = device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &_| {
                if let Ok(mut b) = buffer_clone.lock() {
                    let len = data.len();
                    if b.len() >= data.len {
                        b.drain(0..len)
                            b.extend_from_slice(data);
                    }
                }
            },
            |err| eprintln!("error en el stream: {}", err),
            None
        ).unwrap();
        stream.play().unwrap();
        //esta chingadera de abajo mantiene el hilo vivo
        loop { std::thread::sleep(std::time::Duration::from_millis(100)); }
    });
  }
}
