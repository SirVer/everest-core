use everestrs_build::Builder;

pub fn main() {
    Builder::new("manifest.yaml", "../../..")
        .generate()
        .unwrap();

    // NOCOM(#sirver): remove
    Builder::new("manifest.yaml", "../../..")
        .out_dir("tmp/")
        .generate()
        .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
