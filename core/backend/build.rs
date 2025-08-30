use dotenvy::{dotenv, from_path_override};
use std::env::vars;

fn main() {
    from_path_override("../.placeholder.env").ok();
    dotenv().ok();

    for (k, v) in vars() {
        println!("cargo:rustc-env={k}={v}");
    }
}
