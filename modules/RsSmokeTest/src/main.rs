use std::{thread, time};
use std::path::PathBuf;
use std::env;

fn main() {
    let module = everestrs::Module::from_commandline();

    let dt = time::Duration::from_secs(120);
    thread::sleep(dt);
}
