pub struct MyComplex {
  pub re: f64,
  pub im: f64,
}

impl MyComplex {
  pub fn new(re: f64, im: f64) -> Self {
    Self { re, im }
  }
}

pub fn get_dbail(val: MyComplex, max_dvt: f64, max_iter: i32) -> i32 {
  let max_dvt = max_dvt.powi(2);
  let mut it = 0;
  let mut last = MyComplex::new(0.0, 0.0);
  let mut squared = MyComplex::new(0.0, 0.0);
  let mut deriv = MyComplex::new(0.0, 0.0);
  while squared.re + squared.im <= 4.0
    && deriv.re.powi(2) + deriv.im.powi(2) <= max_dvt
    && it <= max_iter
  {
    deriv.re = 2.0 * (deriv.re * val.re - deriv.im * val.im);
    deriv.im = 2.0 * (deriv.re * val.im + deriv.im * val.re);
    let im = 2.0 * last.re * last.im + val.im;
    let re = squared.re - squared.im + val.re;
    last = MyComplex { re, im };
    squared.re = re.powi(2);
    squared.im = im.powi(2);
    it += 1;
  }
  it
}

pub fn get_fractal(val: MyComplex, max_it: i32) -> i32 {
  let mut it = 0;
  let mut last = MyComplex::new(0.0, 0.0);
  let mut squared = MyComplex::new(0.0, 0.0);
  while squared.re + squared.im <= 4.0 && it < max_it {
    let im = 2.0 * last.re * last.im + val.im;
    let re = squared.re - squared.im + val.re;
    last = MyComplex { re, im };
    squared.re = re.powi(2);
    squared.im = im.powi(2);
    it += 1;
  }
  it
}
