lv_ver := "2020"

integration-tests:
  cargo build -p labview-test-library
  cargo build -p labview-test-library --target i686-pc-windows-msvc
  g-cli --lv-ver {{lv_ver}} viTester -- labview-test-project/rust-interop-test.lvproj
  g-cli --lv-ver {{lv_ver}} --x64 viTester -- labview-test-project/rust-interop-test.lvproj