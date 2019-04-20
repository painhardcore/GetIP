extern crate clap;
use clap::App;

fn main() {
  App::new("getip").version("v0.0.1-beta").get_matches();
}