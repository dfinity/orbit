fn main() {
    let env_vars = vec![(
        "POCKET_IC_BIN",
        format!("{}/pocket-ic", std::env::current_dir().unwrap().display()),
    )];

    for (key, value) in env_vars {
        println!("cargo:rustc-env={}={}", key, value);
    }
}
