use bon::builder;

pub fn configure(args: &crate::Args) {
    let doas_config = generate_config()
        .nb_task(args.nb_task)
        .user_name(&crate::util::user_name(args.user_id))
        .call();
    crate::util::write_config_to_file(&args.doas_file, &doas_config)
        .expect("Failed to write the doas configuration to a file");
}

/// Generate a `doas.conf` policy: one rule per command of each task, the same commands as the
/// other generators. doas reads every rule and the last one that matches wins, so only the very
/// last one allows `/usr/bin/true`. doas only reads user names (`getpwnam`), not ids.
#[builder]
pub fn generate_config(
    nb_role: Option<u32>,
    nb_user: Option<u32>,
    nb_task: u32,
    nb_commands: Option<u32>,
    user_name: &str,
) -> String {
    let nb_role = nb_role.unwrap_or(1);
    let nb_user = nb_user.unwrap_or(1);
    let nb_commands = nb_commands.unwrap_or(1);
    (1..=nb_role)
        .flat_map(|role_index| {
            // the other users of the role do not exist, like the uids of the other generators
            let users = (1..=nb_user)
                .map(|user| {
                    if user == nb_user {
                        user_name.to_string()
                    } else {
                        format!("perf{}", user + 1000 * nb_role)
                    }
                })
                .collect::<Vec<_>>();
            users.into_iter().flat_map(move |user| {
                (1..=nb_task).flat_map(move |task_index| {
                    let user = user.clone();
                    (1..=nb_commands).map(move |i| {
                        let command =
                            if role_index == nb_role && task_index == nb_task && i == nb_commands {
                                "/usr/bin/true".to_string()
                            } else {
                                format!("/usr/bin/true{i}{role_index}{task_index}")
                            };
                        format!("permit nopass {user} as root cmd {command}")
                    })
                })
            })
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}
