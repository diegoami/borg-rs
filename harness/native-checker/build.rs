use std::{env, fs, path::PathBuf, process::Command};
fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .join("../..")
        .canonicalize()
        .unwrap();
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let override_source = env::var_os("BORG_XCHECK_RUST_SOURCE");
    let rust_source = override_source
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("borg-rs/src/borg_formulas_calc.rs"));
    fs::write(
        out.join("calculator.rs"),
        format!(
            "#[allow(dead_code)]\n#[path = {:?}] mod borg_formulas_calc;\n",
            rust_source
        ),
    )
    .unwrap();
    println!(
        "cargo:rustc-env=XCHECK_OVERRIDE={}",
        u8::from(override_source.is_some())
    );
    println!(
        "cargo:rustc-env=XCHECK_RUST_SOURCE={}",
        rust_source.display()
    );
    println!("cargo:rerun-if-env-changed=BORG_XCHECK_RUST_SOURCE");
    println!("cargo:rerun-if-changed={}", rust_source.display());
    println!(
        "cargo:rerun-if-changed={}",
        root.join("angband/src").display()
    );
    println!("cargo:rerun-if-changed=native.c");
    let ubsan = env::var("BORG_XCHECK_UBSAN").as_deref() == Ok("1");
    println!("cargo:rerun-if-env-changed=BORG_XCHECK_UBSAN");
    if ubsan {
        println!("cargo:rustc-link-arg=-fsanitize=undefined");
        println!("cargo:rustc-link-lib=ubsan");
    }
    let sources = [
        "harness/native-checker/native.c",
        "angband/src/borg/borg-formulas.c",
        "angband/src/z-util.c",
        "angband/src/z-virt.c",
    ];
    for (i, source) in sources.iter().enumerate() {
        let object = out.join(format!("native-{i}.o"));
        let mut cc = Command::new("cc");
        cc.args([
            "-O2",
            "-DALLOW_BORG",
            "-DNDEBUG",
            "-ffunction-sections",
            "-fdata-sections",
        ])
        .arg(format!("-I{}", root.join("angband/src").display()));
        if ubsan {
            cc.args(["-fsanitize=undefined", "-fno-sanitize-recover=undefined"]);
        }
        if i == 1 {
            cc.args([
                "-Dparse_value=unused_original_parse_value",
                "-Dcalculate_from_value=unused_original_calculate_from_value",
                "-Dborg_formula_error=unused_original_formula_error",
            ]);
        }
        assert!(cc
            .arg("-c")
            .arg(root.join(source))
            .arg("-o")
            .arg(&object)
            .status()
            .unwrap()
            .success());
        println!("cargo:rustc-link-arg={}", object.display());
    }
    println!("cargo:rustc-link-arg=-Wl,--gc-sections");
}
