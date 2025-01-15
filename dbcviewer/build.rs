fn main() {
    // 告诉 Cargo 链接指定的 .so 库
    //println!("cargo:rustc-link-lib=dylib=xc");  // 链接 `libmylib.so`

    // 如果 .so 库不在系统的默认路径，可以设置路径
    //println!("cargo:rustc-link-search=native=./");

    // 如果需要指定其他参数，例如编译时的链接路径等
    //println!("cargo:rustc-link-search=native=./");
}