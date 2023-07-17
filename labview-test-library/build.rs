fn main() {
    #[cfg(target_arch = "x86_64")]
    println!(
        "cargo:rustc-link-search=C:\\Program Files\\National Instruments\\LabVIEW 2020\\cintools"
    );
    #[cfg(target_arch = "x86")]
    println!(
        "cargo:rustc-link-search=C:\\Program Files (x86)\\National Instruments\\LabVIEW 2020\\cintools"
    );
    println!("cargo:rustc-link-lib=labviewv")
}
