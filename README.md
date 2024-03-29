# Rust-LabVIEW-Interop

A crate to make it easy to work between Rust and LabVIEW.

This is under active development and should be considered unstable. Key types and features are now implemented but APIs could change in future versions depending on feedback.

Desired next steps:

* Custom LabVIEW references. I've seen this used in the openG zip tools and allows you to create a RAII reference in LabVIEW which could be very helpful. Could we make this as a smart pointer in Rust.
* More memory functions e.g. resize array.
* EDVR access

## Structure

The goal is that there will be a module and feature for each of these areas


## Support

The goal is for 32-bit and 64-bit support on Windows and 64-bit support on Linux.

### 32 Bit Clusters

LabVIEW uses cluster packing in 32-bit mode which prevents getting a reference to data in the cluster in Rust since all references must be aligned.

See https://doc.rust-lang.org/std/ptr/fn.read_unaligned.html to see how to read these values.

Because of this limitation, I would recommend using 64-bit LabVIEW whenever possible.

## Contributing

I am very open to contributions in different areas. Please create an issue and discuss what you want to add so we can make sure we are duplicating effort.
