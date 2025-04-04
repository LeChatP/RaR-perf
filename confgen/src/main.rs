use std::rc::Rc;

use capctl::{Cap, CapState};
use clap::Parser;
use rootasrole_core::{save_settings, RemoteStorageSettings, Settings, SettingsFile};

mod rar;
mod sudo;

/// Configure the rootasrole and sudo roles for performance testing.
///
/// # Parameters
/// - `nb_role`: Number of roles to create.
/// - `nb_user`: Number of users per role.
/// - `nb_task`: Number of tasks per role.
/// - `nb_commands`: Number of commands per task.
///
/// # Notes
/// - `nb_role` multiplies the number of `User_Alias` and `nb_task` (`nb_role * nb_task`), and each new task has its own `User_Alias`.
/// - `nb_user` is the number of users in `User_Alias`. `User_Alias` are per role.
/// - `nb_task` is the number of sudo rules, which are per role.
/// - `nb_commands` is the number of commands in a `Cmnd_Alias`, which are per task.
///
/// In order to compare the performance of rootasrole and sudo, we need to create a configuration that is as similar as possible.
/// To be empirical, the valid rule must be at the end of the configuration file.
fn configure_rootasrole_and_sudo(args: &Args) {
    // Generate the rootasrole configuration
    let rootasrole_config = rar::generate_config(
        args.nb_role,
        args.nb_user,
        args.nb_task,
        args.nb_commands,
        args.user_id,
    );
    // Write the rootasrole configuration to a file
    with_cap(Cap::DAC_OVERRIDE, || {
        save_settings(
            &env!("RAR_CFG_PATH").to_string(),
            Rc::new(SettingsFile::builder()
                .storage(
                    Settings::builder()
                        .method(rootasrole_core::StorageMethod::CBOR)
                        .settings(
                            RemoteStorageSettings::builder()
                                .path(env!("RAR_CFG_DATA_PATH"))
                                .not_immutable()
                                .build(),
                        )
                        .build(),
                )
                .config(rootasrole_config)
                .build().into()), true
        ).map_err(|e| {
            anyhow::anyhow!("Failed to save rootasrole config: {}", e)
        })
    })
    .unwrap();

    // Generate the sudo configuration
    let sudo_config = sudo::generate_config(
        args.nb_role,
        args.nb_user,
        args.nb_task,
        args.nb_commands,
        args.user_id,
    );
    // Write the sudo configuration to a file
    with_cap(Cap::DAC_OVERRIDE, || {
        sudo::write_sudo_config_to_file(&args.sudo_file, &sudo_config)
    })
    .unwrap();
}

#[derive(Parser)]
pub struct Args {
    sudo_file: String,
    rar_file: String,
    nb_role: u32,
    nb_user: u32,
    nb_task: u32,
    nb_commands: u32,
    user_id: u32,
}

fn main() {
    let args = Args::parse();

    // Set the effective capabilities to none
    cap_clear().unwrap();

    // Configure the rootasrole and sudo roles for performance testing
    configure_rootasrole_and_sudo(&args);
}

fn with_cap(
    cap: Cap,
    function: impl FnOnce() -> Result<(), anyhow::Error>,
) -> Result<(), anyhow::Error> {
    let mut current = CapState::get_current()?;
    current.effective.clear();
    current.effective.set_state(cap, true);
    current.set_current()?;
    let result = function();
    current.effective.clear();
    current.set_current()?;
    result
}

fn cap_clear() -> Result<(), anyhow::Error> {
    let mut state = CapState::get_current()?;
    state.effective.clear();
    state.set_current()?;
    Ok(())
}
