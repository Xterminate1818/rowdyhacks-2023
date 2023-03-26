use format_num::NumberFormat;
use raylib::prelude::*;

use crate::view::{FractalView, PixelGrid};

pub struct App {
  pub handle: RaylibHandle,
  thread: RaylibThread,
  pub win_width: usize,
  pub win_height: usize,
  pub view: FractalView,
  pub pix_array: PixelGrid,
}

impl App {
  pub fn new(win_width: usize, win_height: usize) -> Self {
    let (handle, thread) = raylib::init()
      .size(win_width as i32, win_height as i32)
      .title("Fractal Explorer")
      .build();
    let pix_width = win_width;
    let pix_height = win_height;
    Self {
      handle,
      thread,
      win_width,
      win_height,
      view: FractalView {
        center_x: -1.745319884878,
        center_y: -0.000000978082,
        // center_x: 0.0,
        // center_y: 0.0,
        pix_width: win_width as i32,
        pix_height: win_height as i32,
        zoom: 100.0,
        iterations: 5000,
        threads: 8,
      },
      pix_array: PixelGrid::new(pix_width, pix_height),
    }
  }

  pub fn run(&mut self) {
    while self.handle_input() {
      self.draw_screen();
    }
  }

  pub fn handle_input(&mut self) -> bool {
    let mut state_changed = false;
    let mut isp = |key: KeyboardKey| {
      let temp = self.handle.is_key_pressed(key);
      state_changed |= temp;
      temp
    };
    use KeyboardKey::*;
    // Mosue movement
    if self
      .handle
      .is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
    {
      let mx = self.handle.get_mouse_x() - (self.win_width / 2) as i32;
      let my = self.handle.get_mouse_y() - (self.win_height / 2) as i32;

      let nx = self.view.center_x + mx as f64 / self.view.zoom;
      let ny = self.view.center_y + my as f64 / self.view.zoom;

      self.view.center_x = nx;
      self.view.center_y = ny;
      self.pix_array = self.view.generate();
    }

    // Zoom
    if isp(KEY_Q) {
      self.view.zoom *= 5.0;
    }
    if isp(KEY_A) {
      self.view.zoom *= 1.0 / 5.0;
    }
    // Iterations
    if isp(KEY_W) {
      self.view.iterations += 1000;
    }
    if isp(KEY_S) {
      self.view.iterations =
        (self.view.iterations - 1000).clamp(1000, i32::MAX);
    }
    // Threads
    if isp(KEY_E) {
      self.view.threads = (self.view.threads * 2).clamp(1, 8);
    }
    if isp(KEY_D) {
      self.view.threads = (self.view.threads / 2).clamp(1, 8);
    }

    let speed = 40.0;

    if isp(KEY_UP) {
      self.view.center_y -= speed / self.view.zoom;
    }
    if isp(KEY_DOWN) {
      self.view.center_y += speed / self.view.zoom;
    }
    if isp(KEY_LEFT) {
      self.view.center_x -= speed / self.view.zoom;
    }
    if isp(KEY_RIGHT) {
      self.view.center_x += speed / self.view.zoom;
    }

    if isp(KEY_ENTER) || state_changed {
      self.pix_array = self.view.generate();
    }

    !self.handle.window_should_close()
  }

  pub fn draw_screen(&mut self) {
    let mut d = self.handle.begin_drawing(&self.thread);
    d.clear_background(Color::BLACK);
    // Draw pixels
    for x in 0..self.view.pix_width {
      for y in 0..self.view.pix_height {
        let color = self.pix_array.get(x, y);
        d.draw_pixel(x, y, color);
      }
    }
    // Draw UI text
    let num = NumberFormat::new();
    let t_color = Color::GREEN;
    let t_size = 20;
    d.draw_text(&format!("FPS: {}", d.get_fps()), 0, 0, t_size, t_color);
    d.draw_text(
      &format!("Zoom: {}", num.format("0.1e", self.view.zoom)),
      0,
      t_size,
      t_size,
      t_color,
    );
    d.draw_text(
      &format!("Passes: {}", self.view.iterations),
      0,
      t_size * 2,
      t_size,
      t_color,
    );

    d.draw_text(
      &format!("Threads: {}", self.view.threads),
      0,
      t_size * 3,
      t_size,
      t_color,
    );
  }
}

impl Default for App {
  fn default() -> Self {
    Self::new(800, 800)
  }
}
