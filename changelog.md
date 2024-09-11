## v0.4.0

### Features

* Added support for `LVStatusCode` and retrieving error descriptions from LabVIEW.


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