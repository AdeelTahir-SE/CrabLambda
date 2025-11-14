use std::process::Command;
use crate::utils::operating_system;
use  anyhow::Result;

pub fn initialize_node_container(packages: Vec<&str>,port: &str,function: &str)->Result<()>{
    let operating_system=operating_system::check_os();
    let mut node_type="node:20-alpine";
    match operating_system.as_str() {
        "linux" => {
            node_type="node:20-alpine";
        }
        "macos" => {
            node_type="node:20-alpine";
        }
        "windows" => {
            node_type="node:20-alpine";
        }
        _ => {
            return Err(anyhow::anyhow!("Only Linux, MacOS and Windows are supported"));
        }
    }
     let status = Command::new("docker")
        .args(&[
            "run",
            "--rm",
            node_type,
            "sh",
            "-c",
            &format!("npm install {} && node -e {}", packages.join(" "),function)
            ,"-p"
            ,port
        ])
        .status()?;  
    if !status.success() {
        return Err(anyhow::anyhow!("Failed to initialize node container: got status error {:?}",status));
    }


    Ok(())
}

