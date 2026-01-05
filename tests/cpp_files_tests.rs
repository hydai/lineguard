use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_cpp_files_are_checked_by_default() {
    let temp_dir = TempDir::new().unwrap();

    // Create C++ project files
    std::fs::write(
        temp_dir.path().join("main.cpp"),
        "int main() {\n    return 0;  \n}\n",
    )
    .unwrap();
    std::fs::write(
        temp_dir.path().join("utils.cc"),
        "void util() {\n    // helper  \n}\n",
    )
    .unwrap();
    std::fs::write(
        temp_dir.path().join("header.h"),
        "#ifndef HEADER_H\n#define HEADER_H  \n#endif\n",
    )
    .unwrap();
    std::fs::write(
        temp_dir.path().join("impl.hpp"),
        "#pragma once\nclass Impl {  \n};\n",
    )
    .unwrap();
    std::fs::write(temp_dir.path().join("test.cxx"), "void test() {  \n}\n").unwrap();
    std::fs::write(
        temp_dir.path().join("old.c"),
        "int old() {  \n    return 1;\n}\n",
    )
    .unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg(".");
    cmd.arg("--recursive");
    cmd.arg("--format").arg("json");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("\"files_checked\": 6"))
        .stdout(predicate::str::contains("main.cpp"))
        .stdout(predicate::str::contains("utils.cc"))
        .stdout(predicate::str::contains("header.h"))
        .stdout(predicate::str::contains("impl.hpp"))
        .stdout(predicate::str::contains("test.cxx"))
        .stdout(predicate::str::contains("old.c"));
}

#[test]
fn test_cpp_object_files_are_skipped() {
    let temp_dir = TempDir::new().unwrap();

    // Create source and object files
    std::fs::write(
        temp_dir.path().join("main.cpp"),
        "int main() {\n    return 0;\n}\n",
    )
    .unwrap();
    std::fs::write(temp_dir.path().join("main.o"), "binary content").unwrap();
    std::fs::write(temp_dir.path().join("lib.a"), "binary content").unwrap();
    std::fs::write(temp_dir.path().join("lib.so"), "binary content").unwrap();

    let mut cmd = cargo_bin_cmd!("lineguard");
    cmd.current_dir(&temp_dir);
    cmd.arg(".");
    cmd.arg("--recursive");
    cmd.arg("--format").arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"files_checked\": 1"));
    // Binary files are properly skipped
}
