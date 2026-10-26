use bon::builder;

pub fn configure(args: &crate::Args) {
    // Generate the sudo configuration
    let sudo_config = generate_config()
        .nb_task(args.nb_task)
        .user_id(args.user_id)
        .call();
    // Write the sudo configuration to a file
    crate::util::write_config_to_file(&args.sudo_file, &sudo_config)
        .expect("Failed to write the sudo configuration to a file");
}

/// Generate the full sudo configuration based on the provided parameters.
#[builder]
pub fn generate_config(
    nb_role: Option<u32>,
    nb_user: Option<u32>,
    nb_task: u32,
    nb_commands: Option<u32>,
    user_id: u32,
) -> String {
    (1..=nb_role.unwrap_or(1))
        .map(|role_index| {
            (1..=nb_task)
                .map(|task_index| {
                    format!(
                        "Cmnd_Alias R{r}T{t} = {commands}",
                        r = role_index,
                        t = task_index,
                        commands = (1..=nb_commands.unwrap_or(1))
                            .map(|i| {
                                let (r, t, c) = if role_index == nb_role.unwrap_or(1)
                                    && task_index == nb_task
                                    && i == nb_commands.unwrap_or(1)
                                {
                                    ("".to_string(), "".to_string(), "".to_string())
                                } else {
                                    (
                                        i.to_string(),
                                        role_index.to_string(),
                                        task_index.to_string(),
                                    )
                                };
                                format!("/usr/bin/true{}{}{}", r, t, c)
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                })
                .chain(std::iter::once(format!(
                    "{users} ALL = {tasks}\n",
                    users = (1..=nb_user.unwrap_or(1))
                        .map(|user| format!(
                            "#{}",
                            if user == nb_user.unwrap_or(1) {
                                user_id
                            } else {
                                user + 1000 * nb_role.unwrap_or(1)
                            }
                        ))
                        .collect::<Vec<_>>()
                        .join(", "),
                    tasks = (1..=nb_task)
                        .map(|task_index| {
                            format!("(ALL) NOPASSWD: R{r}T{t}", r = role_index, t = task_index)
                        })
                        .collect::<Vec<_>>()
                        .join(", "),
                )))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
