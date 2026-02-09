use libpulse_binding as pulse; //Renombramos para que coincida en la importacion
use libpulse_simple_binding as psimple;
use pulse::sample::{Spec, Format}; // Esta chingadera abre flujo de PulseAudio Apuntando a la
use pulse::stream::Direction;   // salida por defecto (Monitor)
use psimple::Simple;
use rustfft::{FftPlanner, num_complex::Complex};

fn main() {
//Configuracion  de audio La pince madre esa de hr  y no se que mas
    let spect = Spec {
        format : Format::S16le,
        channels: 2,
        rate: 48000, //acomodarlo segun su monitor
// si no sabes que monitor usas pon el sig comando
// pactl list sources short | grep monitor wacha la frecuencia
// y la pones en rate
    };
// Esta chingadera que se conecta al monitor con la tarjeta de sonido
    let s = Simple::new(
        None, // <-- esta mmda es el server por defecto
        "circularCava", // <-- esta mmda es el nombre del app
        Direction::Record, // <--lo que queremos que suene
        Some("alsa_output.pci-0000_00_1f.3.analog-stereo.monitor"), //Nombre exacto
        //None, // <-- esta es el dispositivo por defecto
        "Visualizer", // <-- descripcion
        &spect, //<-- esta chingadera no se que es
        None, // la fregadera del canal de el mapeoo
        None, // Esta madre toma los atributos del buffer
    ).expect("No se puede conectar con el puto Pulse audio por que no se, ¿Checa que esa chingadera este jalando? ");
    const BUFFER_SIZE: usize = 1024; //<-- el tamaño que tiene
    let mut buffer = vec![0i16; BUFFER_SIZE * 2]; // que use los *2 para Steteo
    let mut planner = FftPlanner::new(); //esa chincgadera crea un nuevo plano
    let fft = planner.plan_fft_forward(BUFFER_SIZE); // aqui agarramos y usamos el plano con la info del plano
    println!("Wachando el audio... Presiona Ctrl+C para pararlo,");

    loop{
//Truco de memoria, convierte el buffer de i16 a u8 temporalmente para PulseAudio
	unsafe {
	    let ptr = buffer.as_mut_ptr() as *mut u8;
 	    let len = buffer.len()* std::mem::size_of::<i16>();
	    let slice = std::slice::from_raw_parts_mut(ptr, len);
	    s.read(slice).unwrap(); //<- lee los datos de pulseaudio, no siempre le dire chingadera
	}
//Ahora si esta chingadera hace que los numeros sean flowat y promedia los canales (Stereo -> Mono) para la fft pueda leerlo
        let mut input: Vec<Complex<f32>> = buffer
            .chunks_exact(2)
            .map(|chunk|{
                let mono = (chunk[0] as f32 + chunk[1] as f32)/ 2.0;
		Complex {re: mono, im: 0.0} // <- retorna el complejo
            }).collect();

        fft.process(&mut input); //<-aplica la fft
// esta chingadera hace la magnitud de las frecuencias (solo la primera mitas es util, las demas a
// la chingada)
        let spectrum: Vec<f32> = input.iter()
            .take(BUFFER_SIZE / 2)
            .map(|c| c.norm())
            .collect();
// --- Esta es la pinche salida Horizontal (pruebas) ---
// agrupa el espectro en 10 barras simples para la terminal
        let num_barras = 60; //<- se podran cambiar a voluntas o lo que aguante tu pc
        let chunk_size = spectrum.len() / num_barras;
        let mut barras = Vec::new();

        for chunk in spectrum.chunks(chunk_size){
            let sum: f32 = chunk.iter().sum::<f32>();
            let avg = sum / (chunk.len() as f32 + 1.0);
// Esta madre hay que normalizarla asi que el valor lo haremos legible (ajustar el 500.0 segun sea el volumen)
            let val = (avg / 50.0).min(100.0) as usize;
            barras.push(val.to_string());
        }
        println!("[{}]", barras.join(","));
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
    }
}
