# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0](https://github.com/WiresmithTech/Rust-LabVIEW-Interop/compare/labview-interop-v0.3.0...labview-interop-v0.4.0) - 2024-10-07

### Added

- Print Error on memory API error
- Add Error Wrap with Return Status
- Function to Wrap Function in Error
- Accessors for Array Dimensions
- Convert from Chrono Format
- Create Empty Owned Arrays
- try_to_owned and clone for Owned Handles
- Debug Value Printing for UHandle and LvOwned
- LvCopy Trait for Save Type Copys
- Clone from Pointer Restrictions
- Owned Handles and Strings

### Fixed

- Support LVRT Loading
- Conditionally Include Format Error Source
- Replace CTor with lazylock
- Expose NumericArrayResizable trait to users
- Error formatting tests hidden from unit test
- Allow Debug for Unsized Types

### Other

- Clear Warnings and Format
- Validate Build with No Link
- Added Test for LabVIEW Descriptions
- Make LVStatusCode an LV Type
- Added derives to Error Types
- Merge branch 'main' into johannes/error
- Bump ndarray to 0.16.1
- Update DLOpen Dependency
- Update auto-updatable dependencies
- BREAKING CHANGE: Renamed LvOwned
- Move Memory Types to Submodules
- Testing and Docs for UHandle from LvOwned
- Remove ToOwned and Borrow From UHandle
- Progress, not working
- Something is not right
- Added lifetimes to lv_errors
- Merge branch 'main' into johannes/owned-handle
- Moving ToOwned impl
- Adding lifetimes to secondary objects
- Send Sync impl
- From/Clone on LvOwned, Borrow/ToOwned on UHandle
- impl of Clone for LvOwned and Borrow/ToOwned for UHandle
- Add fn handle to LvOwned
- Added lifetime to UHandle
- Some rough first tests for owned Handles and PostLVUserEvent
- Added Drop, removed Copy from UHandle, Added new to UPtr
- Added owned LStrHandle
- Refactored size determination of LStr
- Fixed mac->macos target_os
