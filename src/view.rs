use super::f64_bc;
use super::get_fractal;
use raylib::prelude::Color;

pub struct FractalView {
  pub center_x: f64,
  pub center_y: f64,
  pub pix_width: i32,
  pub pix_height: i32,
  pub zoom: f64,
  pub iterations: i32,
}

impl FractalView {
  pub fn generate(&self) -> Vec<Color> {
    let mut ret: Vec<Color> = vec![];

    let half_width = self.pix_width / 2;
    let half_height = self.pix_height / 2;

    for y in -half_height..half_height {
      for x in -half_width..half_width {
        let scaled_x = (x as f64 / self.zoom) + self.center_x;
        let scaled_y = (y as f64 / self.zoom) + self.center_y;
        let complex = f64_bc(scaled_x, scaled_y);
        let frac_value = get_fractal(complex, self.iterations);
        let normal_value = (frac_value as f64 / self.iterations as f64) * 255.0;
        ret.push(Color::new(255, 255, 255, normal_value as u8));
      }
    }
    ret
  }

  pub fn generate_par(&self) -> Vec<Color> {
    let half_width = self.pix_width / 2;
    let half_height = self.pix_height / 2;
    let size = self.pix_width * self.pix_height;
    (0..size)
      .map(|index| -> Color {
        let x = index.rem_euclid(self.pix_width) - half_width;
        let scaled_x = x as f64 / self.zoom + self.center_x;
        let y = index / self.pix_height - half_height;
        let scaled_y = y as f64 / self.zoom + self.center_y;
        let complex = f64_bc(scaled_x, scaled_y);
        let frac_value = get_fractal(complex, self.iterations);
        let normal_value = (frac_value as f64 / self.iterations as f64) * 255.0;
        Color::new(255, 255, 255, normal_value as u8)
      })
      .collect()
  }
}
