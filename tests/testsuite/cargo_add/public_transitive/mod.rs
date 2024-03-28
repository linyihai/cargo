use cargo_test_support::compare::assert_ui;
use cargo_test_support::current_dir;
use cargo_test_support::file;
use cargo_test_support::prelude::*;
use cargo_test_support::registry::Dependency;
use cargo_test_support::str;
use cargo_test_support::Project;

#[cargo_test]
fn case() {
    cargo_test_support::registry::init();
    cargo_test_support::registry::Package::new("foo", "1.1.0").publish();
    cargo_test_support::registry::Package::new("foo", "2.2.1").publish();

    cargo_test_support::registry::Package::new("bar", "1.1.1")
        .add_dep(Dependency::new("foo", "1.1.0").public(true))
        .publish();

    let project = Project::from_template(current_dir!().join("in"));
    let project_root = project.root();
    let cwd = &project_root;

    snapbox::cmd::Command::cargo_ui()
        .arg("add")
        .arg_line("bar@1.1.1")
        .current_dir(cwd)
        .masquerade_as_nightly_cargo(&["public-dependency"])
        .assert()
        .success();

    snapbox::cmd::Command::cargo_ui()
        .arg("add")
        .arg_line("foo")
        .current_dir(cwd)
        .masquerade_as_nightly_cargo(&["public-dependency"])
        .assert()
        .success()
        .stdout_matches(str![""])
        .stderr_matches(file!["stderr.term.svg"]);

    assert_ui().subset_matches(current_dir!().join("out"), &project_root);
}
