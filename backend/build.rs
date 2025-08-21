use dotenvy::dotenv;
use std::env::vars;

fn main() {
    dotenv().ok();

    for (k, v) in vars() {
        println!("cargo:rustc-env={k}={v}");
    }
}
