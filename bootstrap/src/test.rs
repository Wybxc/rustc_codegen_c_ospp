use std::fs::File;
use std::path::{Path, PathBuf};

use anstream::{eprint as print, eprintln as println};
use clap::Args;
use color_print::{cprint, cprintln};
use glob::glob;
use which::which;

use crate::manifest::Manifest;

/// Run tests
#[derive(Args, Debug)]
pub struct TestCommand {}

impl TestCommand {
    pub fn run(&self, manifest: &Manifest) {
        manifest.prepare();

        std::panic::set_hook(Box::new(|info| {
            cprintln!("<r,s>Test failed</r,s>: {}", info);
        }));

        let testcases = self.collect_testcases(manifest);
        cprintln!("<b>[TEST]</b> found {} testcases", testcases.len());

        let filechecker = FileChecker::new();
        for testcase in testcases {
            cprint!("Testing {}...", testcase.name);
            match testcase.test {
                TestType::FileCheck => {
                    testcase.build(manifest);
                    filechecker.run(&testcase.source, &testcase.output);
                }
            }
            cprintln!("<g>OK</g>");
        }
    }

    pub fn collect_testcases(&self, manifest: &Manifest) -> Vec<TestCase> {
        let mut result = vec![];

        // Codegen tests
        for case in glob("tests/codegen/*.rs").unwrap() {
            let case = case.unwrap();
            let filename = case.file_stem().unwrap();
            let name = format!("codegen/{}", filename.to_string_lossy());
            let output = manifest.out_dir.join("tests/codegen").join(Path::new(filename));
            result.push(TestCase { name, source: case, output, test: TestType::FileCheck })
        }

        result
    }
}

pub enum TestType {
    FileCheck,
}

pub struct TestCase {
    pub name: String,
    pub source: PathBuf,
    pub output: PathBuf,
    pub test: TestType,
}

impl TestCase {
    pub fn build(&self, manifest: &Manifest) {
        std::fs::create_dir_all(self.output.parent().unwrap()).unwrap();
        manifest
            .rustc()
            .args(["--crate-type", "bin"])
            .arg("-O")
            .arg(&self.source)
            .arg("-o")
            .arg(&self.output)
            .status()
            .unwrap();
    }
}

struct FileChecker {
    filecheck: PathBuf,
}

impl FileChecker {
    pub fn new() -> Self {
        let filecheck = [
            "FileCheck",
            "FileCheck-14",
            "FileCheck-15",
            "FileCheck-16",
            "FileCheck-17",
            "FileCheck-18",
        ]
        .into_iter()
        .find_map(|filecheck| which(filecheck).ok())
        .expect("`FileCheck` not found");

        Self { filecheck }
    }

    fn run(&self, source: &Path, output: &Path) {
        let case = source.file_stem().unwrap().to_string_lossy();
        let generated = std::fs::read_dir(output.parent().unwrap())
            .unwrap()
            .filter_map(|entry| entry.ok())
            .find(|entry| {
                let filename = entry.file_name();
                let filename = filename.to_string_lossy();
                filename.ends_with(".c") && filename.starts_with(case.as_ref())
            });

        assert!(generated.is_some(), "could not find {case}'s generated file");
        let generated = generated.unwrap();

        let generated = File::open(generated.path()).unwrap();
        let output = std::process::Command::new(&self.filecheck)
            .arg(source)
            .stdin(generated)
            .output()
            .unwrap();
        assert!(output.status.success(), "failed to run FileCheck on {case}");
    }
}
