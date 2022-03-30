// required thingies for building this lib on Windows platforms
// see issue 3 for more info
// https://github.com/tazz4843/coqui-stt/issues/3

#[cfg(target_os = "windows")]
fn main() {
    // necessary libraries: libstt.so, libkenlm.so, libstt.so.if.lib and libkenlm.so.if.lib.
    println!(r"cargo:rustc-link-search=C:\stt");
    println!("cargo:rustc-link-lib=dylib=libstt.so.if");
    println!("cargo:rustc-link-lib=dylib=libkenlm.so.if");
    println!("cargo:rustc-link-lib=stt");
}

#[cfg(not(target_os = "windows"))]
fn main() {}
