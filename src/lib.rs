use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Command {command} exited with return code {code}")]
pub struct NonZeroReturnCode {
    command: String,
    code: i32,
}

pub fn exec(command: &mut Command) -> Result<(), NonZeroReturnCode> {
    println!("{command:?}");
    let status = command.status().unwrap();
    if !status.success() {
        Err(NonZeroReturnCode {
            command: format!("{command:?}"),
            code: status.code().unwrap(),
        })
    } else {
        Ok(())
    }
}

pub fn build_image(dir: PathBuf, docker_config: PathBuf, image: String) {
    exec(
        Command::new("podman")
            .arg("run")
            .arg("--rm")
            .arg("-v")
            .arg(format!(
                "{}:/workspace",
                dir.as_path().canonicalize().unwrap().display()
            ))
            .arg("-v")
            .arg(format!(
                "{}:/home/cnb/.docker/config.json",
                docker_config.as_path().canonicalize().unwrap().display()
            ))
            .arg("--security-opt")
            .arg("label=disable")
            .arg("docker.io/paketobuildpacks/builder-jammy-full")
            .arg("/cnb/lifecycle/creator")
            .arg("-app")
            .arg("/workspace")
            .arg(image),
    )
    .unwrap();
}
