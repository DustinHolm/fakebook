fn main() {
    // Necessary due to https://github.com/rust-db/refinery/issues/309
    println!("cargo:rerun-if-changed=./migrations");
}
