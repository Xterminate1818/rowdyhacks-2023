use raylib::prelude::*;

use crate::view::FractalView;

pub struct App {
  pub handle: RaylibHandle,
  thread: RaylibThread,
  pub win_width: usize,
  pub win_height: usize,
  pub view: FractalView,
  pub pix_array: Vec<Color>,
}

impl App {
  pub fn new(win_width: usize, win_height: usize, pix_size: usize) -> Self {
    let (handle, thread) = raylib::init()
      .size(win_width as i32, win_height as i32)
      .title("Fractal Explorer")
      .build();
    let pix_width = win_width / pix_size;
    let pix_height = win_height / pix_size;
    let mut ret = Self {
      handle,
      thread,
      win_width,
      win_height,
      view: FractalView {
        center_x: -1.745319884878,
        center_y: -0.000000978082,
        pix_width: win_width as i32,
        pix_height: win_height as i32,
        zoom: 1000000000000.0,
        iterations: 10000,
      },
      pix_array: vec![Color::BLACK; pix_width * pix_height],
    };
    ret.pix_array = ret.view.generate_par();
    ret
  }

  pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
    let normal_x = x.rem_euclid(self.view.pix_width) as usize;
    let normal_y = y.rem_euclid(self.view.pix_height) as usize;
    self.pix_array[normal_y * self.view.pix_height as usize + normal_x] = color;
  }

  pub fn run(&mut self) {
    while self.handle_input() {
      self.draw_screen();
    }
  }

  pub fn handle_input(&mut self) -> bool {
    !self.handle.window_should_close()
  }

  pub fn draw_screen(&mut self) {
    let mut d = self.handle.begin_drawing(&self.thread);
    d.clear_background(Color::BLACK);
    for x in 0..self.view.pix_width as usize {
      for y in 0..self.view.pix_height as usize {
        let color = self.pix_array[y * self.view.pix_height as usize + x];
        d.draw_pixel(x as i32, y as i32, color);
      }
    }
  }
}

impl Default for App {
  fn default() -> Self {
    Self::new(600, 600, 1)
  }
}
