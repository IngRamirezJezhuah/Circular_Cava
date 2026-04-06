use rustfft::{FftPlanner, num_complex::Complex};
use std::f32::consts::PI;

pub struct Visualizador {
    planner: FftPlanner<f32>,
}

impl Visualizador {
    pub fn new() -> Self {
        Self { planner: FftPlanner::new() }
    }

    pub fn procesar_audio(&mut self, data: &[f32], num_barras: usize) -> Vec<f32> {
        let n = data.len();
        let mut buffer: Vec<Complex<f32>> = data.iter()
            .map(|&x| Complex { re: x, im: 0.0 })
            .collect();

        let fft = self.planner.plan_fft_forward(buffer.len());
        fft.process(&mut buffer);

        //aqui Tomo la magnitud y lo agrupo en barras
        let spectrum: Vec<f32> = buffer.iter()
            .take(n / 2)
            .map(|c| c.norm())
            .collect();

        let chunk_size = (spectrum.len() / num_barras).max(1);
        spectrum.chunks(chunk_size)
            .map(|chunk| chunk.iter().sum::<f32>() / chunk.len() as f32)
            .collect()
    }

    // Aquí está el secreto del "Cava Circular"
    // Retorna los puntos (x1, y1, x2, y2) para dibujar cada línea
    pub fn calcular_puntos_circulo(&self, barras: &[f32], centro: (f32, f32), radio_base: f32) -> Vec<(f32, f32, f32, f32)> {
        let mut puntos = Vec::new();
        let paso_angular = (2.0 * PI) / barras.len() as f32;

        for (i, &amplitud) in barras.iter().enumerate() {
            let angulo = i as f32 * paso_angular;
            
            // Punto de inicio (borde del círculo central)
            let x1 = centro.0 + radio_base * angulo.cos();
            let y1 = centro.1 + radio_base * angulo.sin();

            // Punto final (longitud de la barra basada en audio)
            let extension = (amplitud * 500.0).clamp(0.0, 80.0); // Sensibilidad ajustable
            let x2 = centro.0 + (radio_base + extension) * angulo.cos();
            let y2 = centro.1 + (radio_base + extension) * angulo.sin();

            puntos.push((x1, y1, x2, y2));
        }
        puntos
    }
}
