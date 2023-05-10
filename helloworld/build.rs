use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

/*
use flate2::read::GzDecoder;
use tar::Archive;
*/

fn main() {
    // libhermit-rs source dir
	let mut src_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
	src_dir.set_file_name("libhermit-rs");
	if ! src_dir.exists() {
        panic!("no libhermit-rs source dir!");
    }

    // libhermit-rs target dir
	let target_dir = target_dir();
    let manifest_path = src_dir.join("Cargo.toml");
    assert!(
        manifest_path.exists(),
        "kernel manifest path `{}` does not exist",
        manifest_path.display()
    );

	let arch = env::var_os("CARGO_CFG_TARGET_ARCH").unwrap();
	let profile = env::var("PROFILE").expect("PROFILE was not set");

	let mut cmd = Command::new("cargo");

	// Remove rust-toolchain-specific environment variables from kernel cargo
	cmd.env_remove("LD_LIBRARY_PATH");
	env::vars()
        .filter(|(key, _value)| key.starts_with("CARGO") || key.starts_with("RUST"))
        .for_each(|(key, _value)| {
            cmd.env_remove(&key);
        });

    cmd.current_dir(&src_dir)
        .arg("run")
        .arg("--package=xtask")
        .arg("--target-dir")
        .arg(&target_dir)
        .arg("--")
        .arg("build")
        .arg("--arch")
        .arg(&arch)
        .args([
            "--profile",
            match profile.as_str() {
                "debug" => "dev",
                profile => profile,
            },
        ])
        .arg("--target-dir")
			.arg(&target_dir);

    // Control enabled features via this crate's features
    cmd.arg("--no-default-features");
    forward_features(
        &mut cmd,
        [
            "acpi", "dhcpv4", "fsgsbase", "pci", "pci-ids", "smp", "tcp", "trace", "vga",
        ]
        .into_iter(),
    );

    let status = cmd.status().expect("failed to start kernel build");
    assert!(status.success());

    let lib_location = target_dir
        .join(&arch)
        .join(&profile)
        .canonicalize()
        .unwrap();

    eprintln!("lib_location {:?}", lib_location);
    println!("cargo:rustc-link-search=native={}", lib_location.display());
    println!("cargo:rustc-link-lib=static=hermit");

    println!("cargo:rerun-if-changed={}", src_dir.display());
    // HERMIT_LOG_LEVEL_FILTER sets the log level filter at compile time
    println!("cargo:rerun-if-env-changed=HERMIT_LOG_LEVEL_FILTER");

    eprintln!("###### cmd: {:?}", cmd);
}

fn target_dir() -> PathBuf {
	let mut target_dir: PathBuf = env::var_os("PWD").unwrap().into();
	target_dir.pop();
	target_dir.push("libhermit-rs");
	target_dir
}

fn has_feature(feature: &str) -> bool {
	let mut var = "CARGO_FEATURE_".to_string();

	var.extend(feature.chars().map(|c| match c {
		'-' => '_',
		c => c.to_ascii_uppercase(),
	}));

	env::var_os(&var).is_some()
}

fn forward_features<'a>(cmd: &mut Command, features: impl Iterator<Item = &'a str>) {
	let features = features.filter(|f| has_feature(f)).collect::<Vec<_>>();
	if !features.is_empty() {
		cmd.args(["--features", &features.join(" ")]);
	}
}
