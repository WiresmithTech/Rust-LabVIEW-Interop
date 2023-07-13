# Rust-LabVIEW-Interop

A crate to make it easy to work between Rust and LabVIEW.

This is under active development and should be considered unstable.

This will slowly evolve as features are added. Right now I'm creating it for timestamp types but it should evolve to include:

* LabVIEW Handle management.
* Struct layout macros (Mostly this is just #[repr(C)] now but can we use the lv_prolog to support different platforms)
* User event support.
* LabVIEW memory manager.
* Custom LabVIEW references. I've seen this used in the openG zip tools and allows you to create a RAII reference in LabVIEW which could be very helpful. Could we make this as a smart pointer in Rust.

## Structure

The goal is that there will be a module and feature for each of these areas.

## Linking to LabVIEW

This is under-review but right now it requires to to provide a link path to LabVIEW in the project using it. e.g. the following build.rs

```
fn main() {
    println!(
        "cargo:rustc-link-search=C:\\Program Files\\National Instruments\\LabVIEW 2020\\cintools"
    );
    println!("cargo:rustc-link-lib=labviewv")
}
```

There are open questions about this:

1. Can this be done in a cross-version compatible way (within reason) - perhaps by dynamically loading?
2. Is this still required if you don't use functions that call to these? We should probably put these features behind feature flags.


## Support

The goal is for 32-bit and 64-bit support on Windows and 64-bit support on Linux.

### 32 Bit Clusters

LabVIEW uses cluster packing in 32-bit mode which prevents getting a reference to data in the cluster in Rust since all references must be aligned.

See https://doc.rust-lang.org/std/ptr/fn.read_unaligned.html to see how to read these values.

Because of this limitation, I would recommend using 64-bit LabVIEW whenever possible.

## Contributing

I am very open to contributions in different areas. Please create an issue and discuss what you want to add so we can make sure we are duplicating effort.
