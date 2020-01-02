/// A dead simple implementation of the Hsl color space for degrees.
#[derive(Debug)]
pub struct Hsl {
  /// Hue [0.0, 360.0] deg
  pub h: f64,
  /// Saturation [0.0, 1.0]
  pub s: f64,
  /// Lumination [0.0, 1.0]
  pub l: f64,
}

impl Hsl {
  pub fn new(h: f64, s: f64, l: f64) -> Hsl {
    Hsl { h, s, l }
  }
}
