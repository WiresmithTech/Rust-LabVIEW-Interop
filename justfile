set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

lv_ver := "2020"

unit-tests:
  cargo test --features chrono

integration-tests:
  cargo build -p labview-test-library
  cargo build -p labview-test-library --target i686-pc-windows-msvc
  g-cli --lv-ver {{lv_ver}} viTester -- labview-test-project/rust-interop-test.lvproj
  g-cli --lv-ver {{lv_ver}} --x64 viTester -- labview-test-project/rust-interop-test.lvproj

validate:
  # Check we can build with no features (i.e no link)
  cargo check -p labview-interop --no-default-features
  cargo clippy