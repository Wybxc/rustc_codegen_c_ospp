use std::fs::File;
use std::process::Command;

use which::which;

const TESTCASES: &[&str] = &["filename"];

#[test]
pub fn filecheck() {
    let make = which("make").expect("`make` not found");
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

    for case in TESTCASES {
        let output = Command::new(&make).arg(format!("build/{case}")).output().unwrap();
        assert!(output.status.success(), "failed to build {case}");

        let generated = std::fs::read_dir("build")
            .unwrap()
            .filter_map(|entry| entry.ok())
            .find(|entry| entry.file_name().to_string_lossy().starts_with(case));
        assert!(generated.is_some(), "could not find {case}'s generated file");

        let generated = generated.unwrap();
        let generated = File::open(generated.path()).unwrap();
        let output = Command::new(&filecheck)
            .arg(format!("example/tests/{case}.rs"))
            .stdin(generated)
            .output()
            .unwrap();
        assert!(output.status.success(), "failed to run FileCheck on {case}");
    }
}
