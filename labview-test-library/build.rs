fn main() {
    println!(
        "cargo:rustc-link-search=C:\\Program Files\\National Instruments\\LabVIEW 2020\\cintools"
    );
    println!("cargo:rustc-link-lib=labviewv")
}
