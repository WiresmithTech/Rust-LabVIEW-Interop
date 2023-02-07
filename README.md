# Rust-LabVIEW-Interop

A crate to make it easy to work between Rust and LabVIEW.

This will slowly evolve as features are added. Right now I'm creating it for timestamp types but it should evolve to include:

* LabVIEW Handle management.
* Struct layout macros (Mostly this is just #[repr(C)] now but can we use the lv_prolog to support different platforms)
* User event support.
* LabVIEW memory manager.
* Custom LabVIEW references. I've seen this used in the openG zip tools and allows you to create a RAII reference in LabVIEW which could be very helpful. Could we make this as a smart pointer in rust.

## Structure

The goal is that there will be a module and feature for each of these areas.

## Contributing

I am very open to contributions in different areas. Please create an issue and discuss what you want to add so we can make sure we are duplicating effort.
