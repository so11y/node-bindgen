use std::{path::PathBuf, process::Command};
fn main() {
    let node_full_version =
        String::from_utf8(Command::new("node").arg("-v").output().unwrap().stdout).unwrap();
    let node_exec_path = String::from_utf8(
        Command::new("node")
            .arg("-e")
            .arg("console.log(process.execPath)")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    let node_exec_path = node_exec_path.trim_end();
    let mut node_lib_file_dir = PathBuf::from(node_exec_path)
        .parent()
        .unwrap()
        .to_path_buf();
    let node_lib_dir = PathBuf::from(&node_lib_file_dir);
    node_lib_file_dir.push(format!("node-{}.lib", node_full_version.trim_end()));

    
    println!(
        "cargo:rustc-link-lib={}",
        &node_lib_file_dir.file_stem().unwrap().to_str().unwrap()
    );
    println!("cargo:rustc-link-search={}", node_lib_dir.to_str().unwrap());

    let node_runtime_env = "npm_config_runtime";
    println!("cargo:rerun-if-env-changed={}", node_runtime_env);
}
