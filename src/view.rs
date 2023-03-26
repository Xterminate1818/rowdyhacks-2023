use std::thread::{self, JoinHandle};

use raylib::prelude::Color;

use crate::compute::{get_fractal, MyComplex};

pub struct PixelGrid {
  pub data: Vec<Color>,
  pub width: usize,
  pub height: usize,
}

impl PixelGrid {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      data: vec![Color::BLACK; width * height],
      width,
      height,
    }
  }

  pub fn set(&mut self, x: i32, y: i32, color: Color) {
    self.data[y as usize * self.width + x as usize] = color
  }

  pub fn get(&self, x: i32, y: i32) -> Color {
    self.data[y as usize * self.width + x as usize]
  }
}

pub struct FractalView {
  pub center_x: f64,
  pub center_y: f64,
  pub pix_width: i32,
  pub pix_height: i32,
  pub zoom: f64,
  pub iterations: i32,
  pub threads: i32,
}

impl FractalView {
  pub fn generate_linear(&self) -> PixelGrid {
    let mut ret: Vec<Color> = vec![];
    let half_width = self.pix_width / 2;
    let half_height = self.pix_height / 2;
    let size = self.pix_width * self.pix_height;
    for index in 0..size {
      let x = index.rem_euclid(self.pix_width) - half_width;
      let scaled_x = x as f64 / self.zoom + self.center_x;
      let y = index / self.pix_height - half_height;
      let scaled_y = y as f64 / self.zoom + self.center_y;
      let complex = MyComplex::new(scaled_x, scaled_y);
      let frac_value = get_fractal(complex, self.iterations);
      let normal_value = (frac_value as f64 / self.iterations as f64) * 255.0;
      ret.push(Color::new(255, 255, 255, normal_value as u8));
    }
    PixelGrid {
      data: ret,
      width: self.pix_width as usize,
      height: self.pix_height as usize,
    }
  }

  pub fn generate(&self) -> PixelGrid {
    let mut ret: Vec<Color> = vec![];

    let width = self.pix_width;
    let height = self.pix_height;

    let half_width = width / 2;
    let half_height = height / 2;

    let center_x = self.center_x;
    let center_y = self.center_y;

    let subdivisions = self.threads;

    let zoom = self.zoom;
    let size = self.pix_width * self.pix_height / subdivisions;
    let iterations = self.iterations;

    // God my head hurts
    let mut handles: Vec<JoinHandle<Vec<Color>>> = vec![];
    for h in 0..subdivisions {
      handles.push(thread::spawn(move || -> Vec<Color> {
        let mut ret: Vec<Color> = vec![];
        for index in 0..size {
          let x = index.rem_euclid(width) - half_width;
          let scaled_x = x as f64 / zoom + center_x;
          let y =
            ((index / height) + (h * height / subdivisions)) - half_height;
          let scaled_y = y as f64 / zoom + center_y;
          let complex = MyComplex::new(scaled_x, scaled_y);
          let frac_value = get_fractal(complex, iterations);
          let normal_value = (frac_value as f64 / iterations as f64) * 255.0;
          ret.push(Color::new(255, 255, 255, normal_value as u8));
        }
        ret
      }));
    }

    for h in handles {
      ret.append(&mut h.join().unwrap());
    }
    PixelGrid {
      data: ret,
      width: self.pix_width as usize,
      height: self.pix_height as usize,
    }
  }
}
