use anstream::eprintln as println;
use color_print::cprintln;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Manifest {
    pub release: bool,
    pub out_dir: PathBuf,
}

impl Manifest {
    /// Builds the rustc codegen c library and mini_core
    pub fn prepare(&self) {
        cprintln!("<b>[BUILD]</b> codegen backend");
        let mut command = Command::new("cargo");
        command.arg("build").args(["--manifest-path", "crates/Cargo.toml"]);
        if self.release {
            command.arg("--release");
        }
        command.status().unwrap();

        cprintln!("<b>[BUILD]</b> mini_core");
        self.rustc()
            .arg("example/mini_core.rs")
            .args(["--crate-type", "lib"])
            .arg("--out-dir")
            .arg(&self.out_dir)
            .status()
            .unwrap();
    }

    /// The path to the rustc codegen c library
    pub fn codegen_backend(&self) -> &'static Path {
        if self.release {
            Path::new("crates/target/release/librustc_codegen_c.so")
        } else {
            Path::new("crates/target/debug/librustc_codegen_c.so")
        }
    }

    /// The command to run rustc with the codegen backend
    pub fn rustc(&self) -> Command {
        let mut command = Command::new("rustc");
        command
            .arg("-Z")
            .arg(format!("codegen-backend={}", self.codegen_backend().display()))
            .args(["-C", "panic=abort"])
            .arg(format!("-Lall={}", self.out_dir.display()));
        command
    }
}
