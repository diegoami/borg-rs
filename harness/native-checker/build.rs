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
        println!(
            "cargo:rustc-link-arg-bin=borg-native-checker={}",
            object.display()
        );
    }
    println!("cargo:rustc-link-arg=-Wl,--gc-sections");
    assert!(Command::new("python3")
        .arg(root.join("harness/native-checker/extract_spatial.py"))
        .arg(&root)
        .arg(&out)
        .status()
        .unwrap()
        .success());
    let spatial_override = [
        "BORG_XCHECK_FLOW_SOURCE",
        "BORG_XCHECK_PROJECTION_SOURCE",
        "BORG_XCHECK_COMBAT_SOURCE",
        "BORG_XCHECK_GLYPH_SOURCE",
    ]
    .iter()
    .any(|name| env::var_os(name).is_some());
    println!(
        "cargo:rustc-env=SPATIAL_OVERRIDE={}",
        u8::from(spatial_override)
    );
    for name in [
        "BORG_XCHECK_FLOW_SOURCE",
        "BORG_XCHECK_PROJECTION_SOURCE",
        "BORG_XCHECK_COMBAT_SOURCE",
        "BORG_XCHECK_GLYPH_SOURCE",
    ] {
        println!("cargo:rerun-if-env-changed={name}");
        if let Some(path) = env::var_os(name) {
            println!("cargo:rerun-if-changed={}", PathBuf::from(path).display());
        }
    }
    println!("cargo:rerun-if-changed=extract_spatial.py");
    println!("cargo:rerun-if-changed=native_spatial.c");
    for source in [
        "borg_projection.rs",
        "borg_flow.rs",
        "borg_cave.rs",
        "borg_fight_attack.rs",
        "borg_flow_glyph.rs",
    ] {
        println!(
            "cargo:rerun-if-changed={}",
            root.join("borg-rs/src").join(source).display()
        );
    }
    for (i, source) in [
        "harness/native-checker/native_spatial.c",
        "angband/src/borg/borg-cave-util.c",
        "angband/src/z-util.c",
        "angband/src/z-virt.c",
        "angband/src/z-form.c",
        "angband/src/z-bitflag.c",
        "angband/src/z-type.c",
        "angband/src/cave.c",
        "angband/src/cave-view.c",
        "angband/src/borg/borg-flow-misc.c",
    ]
    .iter()
    .enumerate()
    {
        let object = out.join(format!("spatial-{i}.o"));
        let mut cc = Command::new("cc");
        cc.args([
            "-O2",
            "-DALLOW_BORG",
            "-DNDEBUG",
            "-ffunction-sections",
            "-fdata-sections",
            "-Dborg_extract_dir=xc_borg_extract_dir",
            "-Dborg_goto_dir=xc_borg_goto_dir",
            "-Dborg_flow_clear=xc_borg_flow_clear",
            "-Dborg_flow_enqueue_grid=xc_borg_flow_enqueue_grid",
            "-Dborg_flow_spread=xc_borg_flow_spread",
            "-Dborg_flow_commit=xc_borg_flow_commit",
        ])
        .arg(format!("-I{}", root.join("angband/src").display()))
        .arg(format!("-I{}", out.display()));
        if ubsan {
            cc.args(["-fsanitize=undefined", "-fno-sanitize-recover=undefined"]);
        }
        if *source == "angband/src/cave.c" {
            cc.arg("-Dcave=unused_environment_cave");
        }
        assert!(cc
            .arg("-c")
            .arg(root.join(source))
            .arg("-o")
            .arg(&object)
            .status()
            .unwrap()
            .success());
        println!("cargo:rustc-link-arg-bin=spatial={}", object.display());
    }
}
