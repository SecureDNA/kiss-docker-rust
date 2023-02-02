extern crate serde;
extern crate serde_json;

use crate::command;
use std::collections::HashMap;

pub struct Container {
    pub repo: String,
    pub tag: String,
    pub volumes: Vec<String>,
    pub env: HashMap<String, String>,
    pub cmd: String,
    pub port_expose: usize,
    pub port_internal: usize,
    pub blocking: bool,
    pub ops: Vec<String>,
}

pub trait ContainerImpl {
    fn start(&self) -> String;
    fn get_image(&self) -> String;
    fn get_env(&self) -> Vec<String>;
}

pub fn stop_container(id: String) -> String {
    command::docker_exec(vec!["stop", id.as_str()]).unwrap()
}

pub fn list_running() -> String {
    command::docker_exec(vec!["ps", "-a", "-f", "status=running"]).unwrap()
}

impl ContainerImpl for Container {
    fn start(&self) -> String {
        let mut cmd = vec!["run"];
        let img = self.get_image();
        let e = self.get_env();
        let env: Vec<&str> = e.iter().map(|s| s.as_str()).collect();

        if self.blocking {
            cmd.extend(vec!["-a", "-rm"]);
        } else {
            cmd.extend(vec!["-d"]);
        }

        let port_expose = self.port_expose.to_string();
        let port_internal = self.port_internal.to_string();
        let ports = format!("{}:{}", port_expose, port_internal);
        let ports_str = ports.as_str();

        if self.port_internal != 0 && self.port_expose != 0 {
            cmd.extend(vec!["-p", ports_str]);
        }

        if !self.volumes.is_empty() {
            for vol in &self.volumes {
                cmd.extend(vec!["-v", vol.as_str()]);
            }
        }

        if !self.env.is_empty() {
            cmd.extend(env);
        }

        cmd.extend(vec![img.as_str()]);

        if !self.ops.is_empty() {
            let ops_str: Vec<&str> = self.ops.iter().map(|s| &**s).collect();
            cmd.extend(ops_str);
        }

        command::docker_exec(cmd).unwrap()
    }

    fn get_image(&self) -> String {
        let mut cmd = self.repo.clone();
        if !self.tag.is_empty() {
            cmd.push_str(format!(":{}", self.tag).as_str());
        }
        cmd
    }

    fn get_env(&self) -> Vec<String> {
        let mut env: Vec<String> = vec![];
        for (key, value) in &self.env {
            env.push("-e".to_string());
            env.push(format!("{}={}", key, value));
        }
        env
    }
}