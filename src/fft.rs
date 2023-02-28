extern crate num_complex;
use num_complex::Complex;

struct Transformer {
}

impl Transformer {
    pub fn new() -> Self {
        Transformer {}
    }

    fn convert_to_complex(frequencies: Vec<i16>) -> Vec<Complex<i16>> {
        let complex_freq: Vec<num_complex::Complex<i16>> = frequencies
            .into_iter()
            .map(|f| Complex::new(f, 0))
            .collect();

        complex_freq
    }
}

pub fn fft(n: i32, samples: Vec<Complex<i16>>, srate: i32) {
    if n <= 1 {
        return
    }

    // split arrays into odd and even
    // perform recursive fft
    // combine into one vector
}

