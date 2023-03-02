mod render;
mod astream;
mod fft;
use tinysound::DesktopAudioRecorder;

fn main() {
    let mut recorder = DesktopAudioRecorder::new("test stream").unwrap();
    let mut buffer: Vec<i16> = Vec::new();

    loop {
        match recorder.read_frame() {
            Ok(data) => {
                for i in (0..data.len()).step_by(2) {
                    buffer.push(data[i]);

                    if buffer.len() == 16 {
                        break;
                    }
                }
            },
            Err(e) => eprintln!("{}", e)
        };

        if buffer.len() == 16 {
            let freq = buffer.clone();
            buffer.drain(0..);

            let mut new_freq = fft::convert_to_complex(freq);
            fft::fft(16, &mut new_freq);
            let mut heights = fft::convert_to_int(new_freq);
            
            if let Some((w, h)) = term_size::dimensions_stdout() {
                render::print_bars(heights.as_mut_slice(), h, w);   
            } else {
                println!("Unable to get term size :(")
            }
        }
    }
}