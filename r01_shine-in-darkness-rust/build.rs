// build.rs

fn main() {
  println!("cargo:rustc-link-arg-bins=-Tlinker.ld");
}