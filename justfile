set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

lv_ver := "2020"

unit-tests:
  cargo test --no-default-features --features chrono,ndarray

integration-tests:
  cargo build -p labview-test-library
  cargo build -p labview-test-library --target i686-pc-windows-msvc
  g-cli --lv-ver {{lv_ver}} viTester -- -r x86.xml labview-test-project/rust-interop-test.lvproj
  g-cli --lv-ver {{lv_ver}} --x64 viTester -- -r x64.xml labview-test-project/rust-interop-test.lvproj

validate:
  # Check we can build with no features (i.e no link)
  cargo check -p labview-interop --no-default-features
  cargo clippy