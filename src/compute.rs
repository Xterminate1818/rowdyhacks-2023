pub struct MyComplex {
  pub re: f64,
  pub im: f64,
}

impl MyComplex {
  pub fn new(re: f64, im: f64) -> Self {
    Self { re, im }
  }
}

pub fn get_fractal(val: MyComplex, max_it: i32) -> i32 {
  let mut it = 0;
  let mut last = MyComplex::new(0.0, 0.0);
  let mut squared = MyComplex::new(0.0, 0.0);
  while squared.re + squared.im <= 4.0 && it < max_it {
    let im = 2.0 * last.re * last.im + val.im;
    let re = squared.re - squared.im + val.re;
    last = MyComplex::new(re, im);
    squared.re = re.powi(2);
    squared.im = im.powi(2);
    it += 1;
  }
  it
}
