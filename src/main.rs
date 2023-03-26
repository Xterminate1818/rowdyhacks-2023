use compute::*;

pub mod compute;
pub mod render;
pub mod view;

fn main() {
  let mut app = render::App::default();

  // app.pix_array = generate_view_par(
  //   0.0,
  //   0.0,
  //   app.pix_width as i32,
  //   app.pix_height as i32,
  //   200.0,
  // );

  while app.handle_input() {
    app.draw_screen();
  }
}
