use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

pub struct AudioCapturer {
    pub buffer: Arc<Mutex<Vec<f32>>>,
}

impl AudioCapturer {
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(vec![0.0; 2048])),
        }
    }

    pub fn iniciar(&self) {
        let host = cpal::default_host();
        // Buscamos el dispositivo de "Loopback" o Monitor para capturar lo que suena
        let device = host.default_input_device()
            .expect("No se encontró dispositivo de entrada (asegúrate de tener un monitor de PulseAudio activo)");

        let config = device.default_input_config().unwrap();
        let buffer_clone = Arc::clone(&self.buffer);

        let stream = device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &_| {
                if let Ok(mut b) = buffer_clone.lock() {
                    // Copiamos los datos del sistema a nuestro buffer interno
                    *b = data.to_vec();
                }
            },
            |err| eprintln!("Error en el stream de audio: {}", err),
            None
        ).unwrap();

        stream.play().unwrap();
        // Nota: El stream debe vivir para seguir capturando
        std::mem::forget(stream); 
    }
}
