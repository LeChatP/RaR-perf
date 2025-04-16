use std::{cell::RefCell, rc::Rc};

use rootasrole_core::database::{
    actor::SActor,
    options::SAuthentication,
    structs::{
        SCommand, SCommands, SConfig, SCredentials, SGroupschooser, SRole, SSetgidSet, SSetuidSet,
        STask, SUserChooser, SetBehavior,
    },
};

pub fn generate_config(
    nb_role: u32,
    nb_user: u32,
    nb_task: u32,
    nb_commands: u32,
    user_id: u32,
) -> Rc<RefCell<SConfig>> {
    SConfig::builder()
        .options(|opt| opt.authentication(SAuthentication::Skip).build())
        .roles((1..=nb_role).map(|i| {
            SRole::builder("")
                .actors(
                    (1..=nb_user)
                        .map(|j| {
                            if j == nb_user && i == nb_role {
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
                                        .setuid(SUserChooser::ChooserStruct(
                                            SSetuidSet::builder()
                                                .default(SetBehavior::All)
                                                .fallback(0)
                                                .build(),
                                        ))
                                        .setgid(SGroupschooser::StructChooser(
                                            SSetgidSet::builder(0, SetBehavior::All).build(),
                                        ))
                                        .build(),
                                )
                                .commands(
                                    SCommands::builder(SetBehavior::None)
                                        .add(
                                            (1..=nb_commands)
                                                .map(|k| {
                                                    if k == nb_commands
                                                        && j == nb_task
                                                        && i == nb_role
                                                    {
                                                        SCommand::Simple(
                                                            "/usr/bin/true".to_string(),
                                                        )
                                                    } else {
                                                        SCommand::Simple(format!(
                                                            "/usr/bin/true{}",
                                                            k
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
