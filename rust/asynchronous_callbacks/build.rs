fn main() {
    println!("cargo:rustc-link-search=native={}", "../../c/lib");
}
