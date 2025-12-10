use std::env;
use std::path::PathBuf;
use std::process::Command;

fn is_clippy() -> bool {
    std::env::var_os("CLIPPY_ARGS").is_some()
}

fn main() {
    println!("cargo:rerun-if-changed=flint/");
    println!("cargo:rerun-if-changed=build.rs");

    if is_clippy() {
        println!("cargo:warning=skipping native build for clippy");
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let flint_src = PathBuf::from("flint").canonicalize().unwrap();
    let build_dir = out_dir.join("build");

    std::fs::create_dir_all(&build_dir).unwrap();

    // Step 1: Verify configure script exists
    // (It should be committed to version control by the maintainer)
    if !flint_src.join("configure").exists() {
        panic!(
            "flint/configure script is missing!
             This is a packaging error in flint-lll-sys.\
             If you are a maintainer, regenerate it with:
             cd flint && ./bootstrap.sh
             ");
    }

    // Configure FLINT (only if not already configured)
    if !build_dir.join("config.status").exists() {
        // gmp-mpfr-sys guarantees these env vars are set
        let gmp_include = env::var("DEP_GMP_INCLUDE_DIR").unwrap();
        let gmp_lib = env::var("DEP_GMP_LIB_DIR").unwrap();
        let gmp_out = env::var("DEP_GMP_OUT_DIR").unwrap();

        let configure_status = Command::new(flint_src.join("configure"))
            .arg(format!("--prefix={}", out_dir.display()))
            .arg("--disable-shared")
            .arg("--enable-reentrant")
            .arg(format!("--with-gmp-include={}", gmp_include))
            .arg(format!("--with-gmp-lib={}", gmp_lib))
            .arg(format!("--with-mpfr={}", gmp_out))
            .env("CFLAGS", "-O3 -march=native")
            .current_dir(&build_dir)
            .status()
            .unwrap();
        if !configure_status.success() {
            panic!("configure failed");
        }
    }

    // Build FLINT
    let mut cmd = Command::new("make");
    cmd.current_dir(&build_dir);
    cmd.arg("MAINTAINER_MODE=no");  // prevent regenerating ./configure when "stale"
    if let Ok(makeflags) = env::var("CARGO_MAKEFLAGS") && !makeflags.is_empty() {
        // Hook into Cargo's jobserver if available.
        cmd.env("MAKEFLAGS", makeflags);
    }
    let make_status = cmd.status().unwrap();
    if !make_status.success() {
        panic!("make failed");
    }

    /*
    // TODO: fails, dunno why:
    // ld: library 'flint' not found
    let check_status = Command::new("make")
        .arg("check")
        .current_dir(&build_dir)
        .status()
        .unwrap();
    if !check_status.success() {
        panic!("make check failed");
    }*/


    // Install to OUT_DIR
    let install_status = Command::new("make")
        .arg("install")
        .current_dir(&build_dir)
        .arg("MAINTAINER_MODE=no")  // prevent regenerating ./configure when "stale"
        .status()
        .unwrap();
    if !install_status.success() {
        panic!("make install failed");
    }

    // Link the library
    println!("cargo:rustc-link-search=native={}/lib", out_dir.display());
    println!("cargo:rustc-link-lib=static=flint");
    println!("cargo:rustc-link-lib=gmp");
    println!("cargo:rustc-link-lib=mpfr");
}
