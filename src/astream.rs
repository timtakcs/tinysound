use tinysound::DesktopAudioRecorder;

pub fn test() {
    use std::time::Instant;
    let mut recorder = DesktopAudioRecorder::new("Experiment").unwrap();

    let start = Instant::now();
    let mut count = 0;
    let mut d: Vec<Vec<i16>> = Vec::new();

    loop {
        match recorder.read_frame() {
            Ok(data) => {d.push(data); count += 1;},
            Err(e) => eprintln!("{}", e)
        };

        if Instant::now().duration_since(start).as_millis() > 1000 {
            break;
        }
    }

    println!("{}", d.len());

    // for i in 0..d.len() {
    //     println!("{:?}", d[i]);
    // }
}

pub struct Stream {
    recorder: DesktopAudioRecorder,
    buffer: Vec<i16>,
    buffer_size: usize,
}

impl Stream {
    pub fn new(size: usize) -> Self {
        Stream {recorder: DesktopAudioRecorder::new("Audio Stream").unwrap(), 
                buffer: Vec::<i16>::new(), 
                buffer_size: size}
    }
}