mod hsl;
mod rgb;
use hsl::Hsl;
use rgb::Rgb;
fn main() {
  let h = Hsl::new(120.0, 1.0, 0.5);
  let c = Rgb::from(h);

  println!("{:?}", c);
}