use std::{fs::File, io::Write};

use capctl::{Cap, CapState};

#[allow(dead_code)]
pub fn with_cap<T>(
    cap: Cap,
    function: impl FnOnce() -> Result<T, anyhow::Error>,
) -> Result<T, anyhow::Error> {
    let mut current = CapState::get_current()?;
    current.effective.clear();
    current.effective.set_state(cap, true);
    current.set_current()?;
    let result = function();
    current.effective.clear();
    current.set_current()?;
    result
}

/// The name of the user `uid`: doas, please and polkit match names, not ids.
pub fn user_name(uid: u32) -> String {
    nix::unistd::User::from_uid(nix::unistd::Uid::from_raw(uid))
        .ok()
        .flatten()
        .map(|user| user.name)
        .expect("the user id of the benchmark has a name")
}

pub fn cap_clear() -> Result<(), anyhow::Error> {
    let mut state = CapState::get_current()?;
    state.effective.clear();
    state.set_current()?;
    Ok(())
}

/// Write `config` to `filename`, unless the file already holds it: hyperfine runs confgen before
/// each tool, and polkitd reloads its rules each time the file changes, which would take the CPU
/// while another tool is measured.
pub fn write_config_to_file(filename: &str, config: &str) -> anyhow::Result<()> {
    let current = std::fs::read(filename)
        .or_else(|_| with_cap(Cap::DAC_OVERRIDE, || Ok(std::fs::read(filename)?)));
    if current.is_ok_and(|current| current == config.as_bytes()) {
        return Ok(());
    }
    let mut file = File::create(filename).or_else(|_| {
        with_cap(Cap::DAC_OVERRIDE, || {
            File::create(filename).map_err(|e| {
                anyhow::anyhow!(
                    "Failed to create the configuration file with privileges {}: {}",
                    filename,
                    e
                )
            })
        })
    })?;
    file.write_all(config.as_bytes())?;
    Ok(())
}
