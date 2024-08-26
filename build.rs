use std::env;
use std::process::Command;

fn build_docker_image() -> Result<(), String> {
    let project_name = env!("NAME", "project name not defined in config.toml.");
    let project_version = env!("VERSION", "project version not defined in config.toml.");
    let docker_image_name = format!("{project_name}:{project_version}-latest");

    let mut docker_build = Command::new("docker")
        .args(["build", "-t", &docker_image_name, "."])
        .spawn()
        .map_err(|err| format!("failed to spawn docker build: {}", err))?;

    match docker_build.wait() {
        Ok(exitcode) => {
            if exitcode.success() {
                println!("build successful");
                Ok(())
            } else {
                Err(format!("docker build failed with exit code {}", exitcode))
            }
        }
        Err(err) => Err(format!("failed to wait on docker build: {}", err)),
    }
}

fn main() -> Result<(), String> {
    build_docker_image()
}
