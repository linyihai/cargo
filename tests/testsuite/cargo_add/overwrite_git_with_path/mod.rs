use cargo_test_support::basic_manifest;
use cargo_test_support::compare::assert_ui;
use cargo_test_support::current_dir;
use cargo_test_support::file;
use cargo_test_support::git;
use cargo_test_support::prelude::*;
use cargo_test_support::project;
use cargo_test_support::str;

#[cargo_test]
fn case() {
    cargo_test_support::registry::init();
    let git_dependnecy = git::new("dependency", |project| {
        project
            .file("Cargo.toml", &basic_manifest("cargo-list-test-fixture-dependency", "0.0.0"))
            .file("src/lib.rs", "")
    })
    .url();
    let in_project = project()
        .file(
            "Cargo.toml",
            &format!(
                "[workspace]\n\
                 \n\
                 [package]\n\
                 name = \"cargo-list-test-fixture\"\n\
                 version = \"0.0.0\"\n\
                 edition = \"2015\"\n\
                 \n\
                 [dependencies]\n\
                 cargo-list-test-fixture-dependency = {{ git = \"{git_dependnecy}\", optional = true }}\n\
                 ",
            ),
        )
        .file("src/lib.rs", "")
        .build();

    snapbox::cmd::Command::cargo_ui()
        .arg("add")
        .arg_line("cargo-list-test-fixture-dependency --path ../dependency")
        .current_dir(&in_project.root())
        .assert()
        .success()
        .stdout_matches(str![""])
        .stderr_matches(file!["stderr.term.svg"]);

    assert_ui().subset_matches(current_dir!().join("out/primary"), &in_project.root());
}
