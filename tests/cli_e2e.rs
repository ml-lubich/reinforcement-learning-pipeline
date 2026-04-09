//! End-to-end CLI tests (`rlpipe` binary).

use assert_cmd::Command;

#[test]
fn help_lists_commands_and_ascii_motif() {
    let assert = Command::cargo_bin("rlpipe")
        .expect("cargo built `rlpipe` binary")
        .arg("--help")
        .assert()
        .success();

    let hay = std::str::from_utf8(&assert.get_output().stdout).expect("utf8");
    assert!(hay.contains("R L   P I P E L I N E"));
    assert!(hay.contains("demo"));
    assert!(hay.contains("random"));
    assert!(hay.contains("greedy"));
}

#[test]
fn demo_runs_deterministic_report() {
    let assert = Command::cargo_bin("rlpipe")
        .expect("cargo built `rlpipe` binary")
        .arg("demo")
        .assert()
        .success();

    let hay = std::str::from_utf8(&assert.get_output().stdout).expect("utf8");
    assert!(hay.contains("rl pipeline sample"));
    assert!(hay.contains("random_mean"));
    assert!(hay.contains("greedy_mean"));
}

#[test]
fn random_subcommand_prints_summary() {
    let assert = Command::cargo_bin("rlpipe")
        .expect("cargo built `rlpipe` binary")
        .args([
            "random", "--arms", "4", "--best", "1", "--pulls", "40", "--seed", "3",
        ])
        .assert()
        .success();

    let hay = std::str::from_utf8(&assert.get_output().stdout).expect("utf8");
    assert!(hay.contains("mean_reward="));
}

#[test]
fn demo_verbose_flag_still_succeeds() {
    Command::cargo_bin("rlpipe")
        .expect("cargo built `rlpipe` binary")
        .args(["--verbose", "demo"])
        .assert()
        .success();
}
