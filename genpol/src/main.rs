//! genpol <nb_tasks> <uid> <dir> <ref>:<version>:<layout>...
//!
//! The policy of perf/confgen for each version of RootAsRole that versions.yml builds, in the
//! format that this version reads, which confgen cannot write, since it writes the format of its
//! own version: one role of `uid`, where only the last of `nb_tasks` tasks allows /usr/bin/true.
//! The layout `single` writes the policy in the CBOR file `<dir>/<ref>.bin`; `split`, for 3.x,
//! writes there a JSON file that names the CBOR file `<dir>/<ref>-data.bin`.
use std::{error::Error, fs, os::unix::fs::PermissionsExt};

use serde_json::{Value, json};

const USAGE: &str = "usage: genpol <nb_tasks> <uid> <dir> <ref>:<version>:<single|split>...";

/// The policy of `version`, stored in `path`.
fn policy(version: &str, nb_tasks: usize, uid: u32, path: &str) -> Value {
    let tasks: Vec<_> = (1..=nb_tasks)
        .map(|i| {
            let command = if i == nb_tasks {
                "/usr/bin/true".to_string()
            } else {
                format!("/usr/bin/true11{i}")
            };
            json!({
                "name": format!("t{i}"),
                "cred": {
                    "setuid": {"default": "all", "fallback": 0},
                    "setgid": {"default": "all", "fallback": [0]}
                },
                "commands": {"default": "none", "add": [command]}
            })
        })
        .collect();
    json!({
        "version": version,
        "storage": {"method": "cbor", "settings": {"immutable": false, "path": path}},
        "options": {"authentication": "skip"},
        "roles": [{"name": "r", "actors": [{"type": "user", "id": uid}], "tasks": tasks}]
    })
}

/// Writes `bytes` in `path`, that only its owner reads.
fn write(path: &str, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    fs::write(path, bytes)?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [nb_tasks, uid, dir, versions @ ..] = args.as_slice() else {
        return Err(USAGE.into());
    };
    if versions.is_empty() {
        return Err(USAGE.into());
    }
    let (nb_tasks, uid): (usize, u32) = (nb_tasks.parse()?, uid.parse()?);
    for spec in versions {
        let [name, version, layout] = spec.split(':').collect::<Vec<_>>()[..] else {
            return Err(format!("{spec}: {USAGE}").into());
        };
        let root = format!("{dir}/{name}.bin");
        match layout {
            "single" => {
                let policy = policy(version, nb_tasks, uid, &root);
                write(&root, &cbor4ii::serde::to_vec(Vec::new(), &policy)?)?;
            }
            "split" => {
                let data = format!("{dir}/{name}-data.bin");
                let policy = policy(version, nb_tasks, uid, &data);
                write(&data, &cbor4ii::serde::to_vec(Vec::new(), &policy)?)?;
                let settings = json!({
                    "version": version,
                    "storage": {"method": "cbor", "settings": {"immutable": false, "path": data}}
                });
                write(&root, &serde_json::to_vec(&settings)?)?;
            }
            _ => return Err(format!("{spec}: {USAGE}").into()),
        }
    }
    Ok(())
}
