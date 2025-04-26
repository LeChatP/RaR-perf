use std::fs::File;
use std::io::Write;


/// Generate the full sudo configuration based on the provided parameters.
pub fn generate_config(
    nb_role: u32,
    nb_user: u32,
    nb_task: u32,
    nb_commands: u32,
    user_id: u32,
) -> String {
    (1..=nb_role).map(|role_index| {
        (1..=nb_task).map(|task_index| {
            format!(
                "Cmnd_Alias R{r}T{t} = {commands}",
                r=role_index,
                t=task_index,
                commands=(1..=nb_commands)
                    .map(|i| {
                        let (r,t,c) = if role_index==nb_role && task_index == nb_task && i == nb_commands { 
                            ("".to_string(),"".to_string() ,"".to_string() ) 
                        } else { 
                            (i.to_string(), role_index.to_string(), task_index.to_string())
                        };
                        format!("/usr/bin/true{}{}{}",r,t,c)  
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }).chain(std::iter::once(format!(
            "{users} ALL = {tasks}\n",
            users=(1..=nb_user)
                .map(|user| format!("#{}", if user == nb_user { user_id } else { user + nb_role } ))
                .collect::<Vec<_>>()
                .join(", "),
            tasks=(1..=nb_task)
                .map(|task_index| {
                    format!("(ALL) NOPASSWD: R{r}T{t}",
                        r=role_index,
                        t=task_index
                    )
                })
                .collect::<Vec<_>>()
                .join(", "),
        ))).collect::<Vec<_>>().join("\n")
    }).collect::<Vec<_>>().join("\n")
}

/// Write the generated sudo configuration to a file.
pub fn write_sudo_config_to_file(filename: &str, config: &str) -> anyhow::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(config.as_bytes())?;
    Ok(())
}