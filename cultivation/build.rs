use cc::Build;

fn main() {
    Build::new()
        .cpp(true)
        .flag("/std:c++latest")
        .file("src-cpp/injector.cpp")
        .compile("injector");
}
