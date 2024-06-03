fn windows_msvc() {
    println!(r"cargo:rustc-link-search=vendored-deps/msvc/libs");
}

fn main() {
    #[cfg(all(target_os = "windows", target_env="msvc"))]
    windows_msvc();
}
