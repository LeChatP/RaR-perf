use std::{cell::RefCell, rc::Rc};

use bon::builder;
use rootasrole_core::{
    RemoteStorageSettings, SettingsContent,
    database::versionning::Versioning,
    database::{
        actor::SActor,
        options::SAuthentication,
        structs::{
            SCommand, SCommands, SCredentials, SGroupsEither, SPolicy, SRole, SSetgidSet,
            SSetuidSet, STask, SUserEither, SetBehavior,
        },
    },
    file::{LockedSettingsFile, RootSettings},
    util::StorageMethod,
};

pub fn configure(args: &crate::Args) {
    // the policy is in the root file itself, as dosr reads it from RAR_CFG_PATH
    let config = LockedSettingsFile::open_write(&args.rar_file, |_, _| {
        // the version written in the file, without which dosr refuses it
        Ok(Versioning::new(
            RootSettings::builder()
                .storage(
                    SettingsContent::builder()
                        .method(StorageMethod::default())
                        .settings(
                            RemoteStorageSettings::builder()
                                .path(args.rar_file.as_str())
                                .not_immutable()
                                .build(),
                        )
                        .build(),
                )
                .config(
                    generate_config()
                        .nb_task(args.nb_task)
                        .user_id(args.user_id)
                        .call(),
                )
                .build(),
        ))
    });

    config
        .unwrap()
        .save(StorageMethod::default(), false)
        .unwrap();
}

#[builder]
pub fn generate_config(
    nb_role: Option<u32>,
    nb_user: Option<u32>,
    nb_task: u32,
    nb_commands: Option<u32>,
    user_id: u32,
) -> Rc<RefCell<SPolicy>> {
    SPolicy::builder()
        .options(|opt| opt.authentication(SAuthentication::Skip).build())
        .roles((1..=nb_role.unwrap_or(1)).map(|i| {
            SRole::builder("")
                .actors(
                    (1..=nb_user.unwrap_or(1))
                        .map(|j| {
                            if j == nb_user.unwrap_or(1) && i == nb_role.unwrap_or(1) {
                                SActor::user(user_id).build()
                            } else {
                                SActor::user(j as u32 + 5000).build()
                            }
                        })
                        .collect::<Vec<_>>(),
                )
                .tasks(
                    (1..=nb_task)
                        .map(|j| {
                            STask::builder(0)
                                .cred(
                                    SCredentials::builder()
                                        .setuid(SUserEither::UserSelector(
                                            SSetuidSet::builder()
                                                .default(SetBehavior::All)
                                                .fallback(0)
                                                .build(),
                                        ))
                                        .setgid(SGroupsEither::GroupSelector(
                                            SSetgidSet::builder(SetBehavior::All)
                                                .fallback(0)
                                                .build(),
                                        ))
                                        .build(),
                                )
                                .commands(
                                    SCommands::builder(SetBehavior::None)
                                        .add(
                                            (1..=nb_commands.unwrap_or(1))
                                                .map(|k| {
                                                    if k == nb_commands.unwrap_or(1)
                                                        && j == nb_task
                                                        && i == nb_role.unwrap_or(1)
                                                    {
                                                        SCommand::Simple(
                                                            "/usr/bin/true".to_string(),
                                                        )
                                                    } else {
                                                        // the names of the other generators:
                                                        // command, role, task
                                                        SCommand::Simple(format!(
                                                            "/usr/bin/true{k}{i}{j}"
                                                        ))
                                                    }
                                                })
                                                .collect::<Vec<_>>(),
                                        )
                                        .build(),
                                )
                                .build()
                        })
                        .collect::<Vec<_>>(),
                )
                .build()
        }))
        .build()
}
