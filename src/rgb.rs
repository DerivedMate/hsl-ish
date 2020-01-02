use crate::hsl::Hsl;

#[derive(Debug)]
pub struct Rgb {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl Rgb {
  pub fn new(r: u8, g: u8, b: u8) -> Self {
    Rgb { r, g, b }
  }

  /// Reduces RGB range of [0, 255] to sRGB range of [0.0, 1.0]
  pub fn s_rgb(&self) -> [f64; 3] {
    let (r, g, b) = (self.r as f64, self.g as f64, self.b as f64);
    [r / 255.0, g / 255.0, b / 255.0]
  }
}

fn hue2rgb(p: f64, q: f64, t: f64) -> f64 {
  let mut t = t;
  if t < 0.0 {
    t += 1.0;
  } else if t > 1.0 {
    t -= 1.0;
  }

  if t < 1.0 / 6.0 {
    p + (q - p) * 6.0 * t
  } else if t < 1.0 / 2.0 {
    q
  } else if t < 2.0 / 3.0 {
    p + (q - p) * (2.0 / 3.0 - t) * 6.0
  } else {
    p
  }
}

impl From<Hsl> for Rgb {
  /// Converts from the HSL to the RGB color space.
  /// # Example
  /// ```
  /// use hsl_ish::{Hsl, Rgb};
  /// let hsl = Hsl::new(120.0, 1.0, 0.5);
  /// let rgb = Rgb::from(hsl);
  /// 
  /// assert_eq!((rgb.r, rgb.g, rgb.b), (0, 255, 0));
  /// ```
  fn from(hsl: Hsl) -> Self {
    /*
    h: 0 - 360,
    s: 0 - 1.0,
    l: 0 - 1.0,
    */
    let (h, s, l) = (hsl.h / 360.0, hsl.s, hsl.l);
    let ref mut r = 0.0;
    let ref mut g = 0.0;
    let ref mut b = 0.0;

    if s == 0.0 {
      *r = l;
      *g = l;
      *b = l;
    } else {
      let q = if l < 0.5 {
        l * (1.0 + s)
      } else {
        l + s - l * s
      };

      let p = 2.0 * l - q;
      *r = hue2rgb(p, q, h + 1.0 / 3.0);
      *g = hue2rgb(p, q, h);
      *b = hue2rgb(p, q, h - 1.0 / 3.0);
    }

    Self {
      r: (*r * 255.0).round() as u8,
      g: (*g * 255.0).round() as u8,
      b: (*b * 255.0).round() as u8,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  macro_rules! tests {
    ($($name:ident: $hsl:expr, $rgb:expr;)*) => {
    $(
        #[test]
        fn $name() {
          let (h, s, l) = $hsl;
          let (r, g, b) = $rgb;
          let c_hsl = Hsl::new(h, s, l);
          let c_rgb = Rgb::from(c_hsl);
          assert_eq!(c_rgb.r, r);
          assert_eq!(c_rgb.g, g);
          assert_eq!(c_rgb.b, b);
        }
    )*
    }
  }

  tests! {
    test0:  (0.0  , 0.0, 0.0),  (0,0,0);
    test1:  (0.0  , 0.0, 1.0),  (255,255,255);
    test2:  (0.0  , 1.0, 0.5),  (255,0,0);
    test3:  (120.0, 1.0, 0.5),  (0,255,0);
    test4:  (240.0, 1.0, 0.5),  (0,0,255);
    test5:  (60.0 , 1.0, 0.5),  (255,255,0);
    test6:  (180.0, 1.0, 0.5),  (0,255,255);
    test7:  (300.0, 1.0, 0.5),  (255,0,255);
    test8:  (0.0  , 0.0, 0.75), (191,191,191);
    test9:  (0.0  , 0.0, 0.5),  (128,128,128);
    test10: (0.0  , 1.0, 0.25), (128,0,0);
    test11: (60.0 , 1.0, 0.25), (128,128,0); //
    test12: (120.0, 1.0, 0.25), (0,128,0);
    test13: (300.0, 1.0, 0.25), (128,0,128); //
    test14: (180.0, 1.0, 0.25), (0,128,128); //
    test15: (240.0, 1.0, 0.25), (0,0,128);
  }
}
