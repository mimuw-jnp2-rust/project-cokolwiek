// This build script sets environment variables for the stt library to be
// visible during compilation, linking and running of the program.

fn main() {
    println!("cargo:rustc-env=LIBRARY_PATH=libstt");
    println!("cargo:rustc-env=LD_LIBRARY_PATH=libstt");
}
