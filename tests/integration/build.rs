fn main() {
    let env_vars = vec![
        // The current directory for the build script is always the root directory of the package it belongs to:
        // https://doc.rust-lang.org/cargo/reference/build-scripts.html#inputs-to-the-build-script,
        // which is where the `pocket-ic`` binary gets downloaded to.
        (
            "POCKET_IC_BIN",
            format!("{}/pocket-ic", std::env::current_dir().unwrap().display()),
        ),
    ];

    for (key, value) in env_vars {
        println!("cargo:rustc-env={}={}", key, value);
    }
}
