use bon::builder;

use crate::util::write_config_to_file;

pub fn configure(args: &crate::Args) {
    // please matches the name of the user, not its id
    let user_name = crate::util::user_name(args.user_id);
    write_config_to_file(
        &args.please_file,
        &generate_config()
            .nb_task(args.nb_task)
            .user_name(&user_name)
            .call(),
    )
    .expect("Failed to write the please configuration to a file");
}

/// Generate the `please.ini` policy: one section per task, as sudo and doas have one rule per
/// task. please reads every section and the last one that matches wins, so all of them are read,
/// and only the last one allows `/usr/bin/true`.
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
        .map(|role_index| {
            // the other users of the role do not exist, like the uids of the other generators
            let name = (1..=nb_user)
                .map(|user| {
                    if user == nb_user {
                        user_name.to_string()
                    } else {
                        format!("perf{}", user + 1000 * nb_role)
                    }
                })
                .collect::<Vec<_>>()
                .join("|");
            (1..=nb_task)
                .map(|task_index| {
                    let rule = (1..=nb_commands)
                        .map(|i| {
                            if role_index == nb_role && task_index == nb_task && i == nb_commands {
                                "/usr/bin/true".to_string()
                            } else {
                                format!("/usr/bin/true{i}{role_index}{task_index}")
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("|");
                    format!(
                        "[r{role_index}t{task_index}]\nname = ^({name})$\ntarget = .*\nrule = ^({rule})$\npermit = true\nrequire_pass = false\n"
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
