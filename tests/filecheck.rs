use std::fs::File;
use std::process::Command;

const TESTCASES: &[&str] = &["filename"];

#[test]
pub fn filecheck() {
    for case in TESTCASES {
        let output = Command::new("make").arg(format!("build/{case}")).output().unwrap();
        assert!(output.status.success(), "failed to build {case}");

        let generated = std::fs::read_dir("build")
            .unwrap()
            .filter_map(|entry| entry.ok())
            .find(|entry| entry.file_name().to_string_lossy().starts_with(case));
        assert!(generated.is_some(), "could not find {case}'s generated file");

        let generated = generated.unwrap();
        let generated = File::open(generated.path()).unwrap();
        let output = Command::new("FileCheck")
            .arg(format!("example/tests/{case}.rs"))
            .stdin(generated)
            .output()
            .unwrap();
        assert!(output.status.success(), "failed to run FileCheck on {case}");
    }
}
