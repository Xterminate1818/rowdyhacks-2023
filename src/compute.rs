use num::rational::BigRational;
use num::Complex;
use num::FromPrimitive;
use num::ToPrimitive;

pub type BigComplex = Complex<f64>;

pub fn fractal_iterate(val: BigComplex, constant: BigComplex) -> BigComplex {
  let sq = val.powi(2);
  sq + constant
}

pub fn f64_bc(re: f64, im: f64) -> BigComplex {
  // let new_re = Ratio::<i128>::from_f64(re).unwrap();
  // let new_im = Ratio::<i128>::from_f64(im).unwrap();
  BigComplex::new(re, im)
}

pub fn f64_rat(frac: f64) -> BigRational {
  BigRational::from_f64(frac).unwrap()
}

pub fn complex_abs(val: &BigComplex) -> f64 {
  val.norm_sqr().to_f64().unwrap()
}

pub fn iterate(last: BigComplex, constant: BigComplex) -> BigComplex {
  last.powi(2) + constant
}

pub fn get_fractal(val: BigComplex, max_it: i32) -> i32 {
  let mut it = 0;

  let mut last = f64_bc(0.0, 0.0);
  while it < max_it && complex_abs(&last) <= 4.0 {
    last = iterate(last, val);
    it += 1;
  }
  it
}
