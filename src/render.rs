use std::vec;
use rand::Rng;
use std::{thread, time};

extern crate term_size;

fn print_bars(heights: &[i32], h: usize, w: usize) {
    let mut height_strings: Vec<String> = Vec::new();

    let num = heights.len();

    let mut outstr = String::from("");
    let mut idx = 0;
    let bar_width = w / (2 * num + 1);

    for i in 0..h {
        for j in 0..w -  bar_width{
            idx = j / 12;
            
            if idx % 2 == 1  && 60 - heights[idx / 2] <= i.try_into().unwrap(){
                outstr.push('.');
            } else {
                outstr.push(' ');
            }    
        }
        outstr.push('\n');
    }

    print!("{}", outstr);
}

pub fn test() {
    let mut rng = rand::thread_rng();
    let ten_millis = time::Duration::from_millis(50);
    let mut heights = vec![33, 12, 39, 5, 9, 27, 13, 17];

    if let Some((w, h)) = term_size::dimensions_stdout() {
        for i in 0..60 {
            print_bars(heights.as_slice(), h, w);

            for i in 0..heights.len() {
                let factor = 1;

                heights[i] += 2 * (factor - 2 * rng.gen_range(0..2));
            }

            std::thread::sleep(ten_millis);
        }
    } else {
        println!("Unable to get term size :(")
    }
}
