extern crate num_complex;
use num_complex::Complex;
use std::{f64::consts, f32::consts::{PI}};

static I: Complex<f32> = Complex::<f32>{re: 0.0, im: 1.0};

struct Transformer;

impl Transformer {
    pub fn new() -> Self {
        Transformer
    }

    fn convert_to_complex(frequencies: Vec<i16>) -> Vec<Complex<f32>> {
        let complex_freq: Vec<num_complex::Complex<f32>> = frequencies
            .into_iter()
            .map(|f| Complex::new((f as f32), 0.0))
            .collect();

        complex_freq
    }

    pub fn fft(&self, n: usize, samples: &mut Vec<Complex<f32>>) {
        if n <= 1 {
            return
        }
    
        if (n != 0) && ((n & (n - 1)) == 0) {
            panic!("The number of bins can only be a power of two, found {}", n);
        }
    
        let mut even: Vec<Complex<f32>> = Vec::new();
        let mut odd: Vec<Complex<f32>> = Vec::new();
    
        for i in 0..n {
            if i % 2 == 0 {
                even.push(samples[i]);
            } else {
                odd.push(samples[i]);
            }
        }
    
        self.fft(n/2, &mut even);
        self.fft(n/2, &mut odd);
    
        for k in 0..n/2 {
            let t = (-I * PI * (k as f32) / (n as f32)).exp() * odd[k];
            samples[k] = even[k] + t;
            samples[k + n/2] = even[k] - t; 
        }
    }
}


