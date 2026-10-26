use bon::builder;

pub fn configure(args: &crate::Args) {
    // polkit gives the name of the user, not its id
    let user_name = crate::util::user_name(args.user_id);
    let polkit_config = generate_config()
        .nb_task(args.nb_task)
        .user_name(&user_name)
        .call();
    crate::util::write_config_to_file(&args.run0_file, &polkit_config)
        .expect("Failed to write the polkit configuration to a file");
}

/// Generate the polkit rules of `run0`: one rule per task, only the last one grants.
///
/// For `run0 <command>`, systemd asks polkit for `org.freedesktop.systemd1.manage-units`, verb
/// `start`, on a transient unit `run-p<pid>-i<id>.service` (measured with polkit 127 and systemd
/// 262): polkit never sees the command, so a rule cannot allow `/usr/bin/true` alone, it allows
/// any command to the user. The rules that do not grant name users that do not exist, where the
/// other tools name commands that do not exist. polkit stops at the first rule that answers, so
/// the one that grants comes last, and every rule is evaluated.
#[builder]
pub fn generate_config(
    nb_role: Option<u32>,
    nb_user: Option<u32>,
    nb_task: u32,
    user_name: &str,
) -> String {
    let nb_role = nb_role.unwrap_or(1);
    let nb_user = nb_user.unwrap_or(1);
    (1..=nb_role)
        .flat_map(|role_index| {
            (1..=nb_task).map(move |task_index| {
                let last = role_index == nb_role && task_index == nb_task;
                let users = (1..=nb_user)
                    .map(|user| {
                        if last && user == nb_user {
                            format!("\"{user_name}\"")
                        } else {
                            format!("\"perf{user}{role_index}{task_index}\"")
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "// role {role_index}, task {task_index}
polkit.addRule(function(action, subject) {{
    if (action.id == \"org.freedesktop.systemd1.manage-units\"
        && action.lookup(\"verb\") == \"start\"
        && action.lookup(\"unit\").indexOf(\"run-p\") == 0
        && [{users}].indexOf(subject.user) >= 0) {{
        return polkit.Result.YES;
    }}
    return undefined;
}});
"
                )
            })
        })
        .collect::<Vec<_>>()
        .join("\n")
}
