fn main() {
    // 指定库搜索路径
    println!("cargo:rustc-link-search=native=/home/mengen.tao/taomengen/openvino_dcn_inference/lib");
    // 指定要链接的库名称（不需要带 lib 前缀和 .so 后缀）
    println!("cargo:rustc-link-lib=dylib=sum");

    cc::Build::new()
        .cpp(true)                // 启用 C++ 编译
        .file("src/example.cpp")  // 指定 C++ 源文件
        .compile("example");      // 输出库名为 `libexample.a`（或 `.lib`）
}
