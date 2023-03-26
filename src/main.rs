pub mod compute;
pub mod render;
pub mod view;

fn main() {
  let mut app = render::App::default();

  while app.handle_input() {
    app.draw_screen();
  }
}
