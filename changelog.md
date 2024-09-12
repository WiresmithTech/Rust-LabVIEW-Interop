## v0.4.0

### Breaking Changes

* MSRV has been bumped to 1.80.0 for `LazyLock` support.

### Features

* Added support for `LVStatusCode` and retrieving error descriptions from LabVIEW.
* Added api for accessing dimensions of arrays.
* Add API to error clusters to execute the rust code following the LabVIEW error semantics.
* Lazily load LabVIEW functions to allow time for LabVIEW to load.
#### Fixes

* Expose NumericArrayResize trait to users of the Library.

## v0.3.0

### Features

* Added support for error clusters.
* Added support for NDArray Views (behind the `ndarray` feature flag).
* Added numeric array resizing.
* Added an unsafe set method to set array values on 32 bit.

### Fixes

* Altered LVTime seconds component to i64 instead of u64

## v0.2.1

### Fixes

* Add unsafe to ctor elements in LabVIEW interface as some versions of rust threw errors.

## v0.2.0

### Features

* Boolean support
* String support
* Improved error handling.
* User event and occurance support.

## v0.1.0

### Features

* Timestamp support
* Cluster and array layout support.