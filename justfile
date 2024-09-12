set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

lv_ver := "2020"

unit-tests:
  cargo test --no-default-features --features chrono,ndarray

integration-tests-x86:
  cargo build -p labview-test-library --target i686-pc-windows-msvc
  g-cli --lv-ver {{lv_ver}} viTester -- -r x86.xml labview-test-project/rust-interop-test.lvproj

integration-tests-x64:
    cargo build -p labview-test-library
    g-cli --lv-ver {{lv_ver}} --x64 viTester -- -r x64.xml labview-test-project/rust-interop-test.lvproj

integration-tests: integration-tests-x86 integration-tests-x64

validate:
  cargo fmt --all -- --check
  # Check we can build with no features (i.e no link)
  cargo check -p labview-interop --no-default-features
  # Check no conflicts with features.
  cargo check -p labview-interop --all-features
  cargo clippy