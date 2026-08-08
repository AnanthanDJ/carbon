use anyhow::{Result, anyhow};

use crate::{
    app::AppState,
    terminal::{ParsedCommand, TerminalSession},
};

pub struct CommandResult {
    pub stdout: String,
}

pub async fn dispatch(
    state: &AppState,
    session: &mut TerminalSession,
    command: &ParsedCommand,
) -> Result<CommandResult> {
    match command.name.as_str() {
        "pwd" => cmd_pwd(state, session).await,
        "ls" => cmd_ls(state, session, &command.args).await,
        "cd" => cmd_cd(state, session, &command.args).await,
        "mkdir" => cmd_mkdir(state, session, &command.args).await,
        "touch" => cmd_touch(state, session, &command.args).await,
        "cat" => cmd_cat(state, session, &command.args).await,
        "rm" => cmd_rm(state, session, &command.args).await,
        "rmdir" => cmd_rmdir(state, session, &command.args).await,
        "echo" => cmd_echo(&command.args).await,
        "help" => cmd_help().await,
        "clear" => cmd_clear().await,
        _ => Err(anyhow!("{}: command not found", command.name)),
    }
}

async fn cmd_pwd(state: &AppState, session: &TerminalSession) -> anyhow::Result<CommandResult> {
    Ok(CommandResult {
        stdout: state.filesystem.pwd(session),
    })
}

async fn cmd_echo(args: &[String]) -> anyhow::Result<CommandResult> {
    Ok(CommandResult {
        stdout: args.join(" "),
    })
}

async fn cmd_clear() -> anyhow::Result<CommandResult> {
    Ok(CommandResult {
        stdout: String::new(),
    })
}

async fn cmd_ls(
    state: &AppState,
    session: &TerminalSession,
    args: &[String],
) -> anyhow::Result<CommandResult> {
    let path = args.first().map(String::as_str).unwrap_or(".");

    let nodes = state.filesystem.ls(session, path).await?;

    Ok(CommandResult {
        stdout: nodes
            .into_iter()
            .map(|n| n.name)
            .collect::<Vec<_>>()
            .join("\n"),
    })
}

async fn cmd_cd(
    state: &AppState,
    session: &mut TerminalSession,
    args: &[String],
) -> anyhow::Result<CommandResult> {
    let path = args.first().map(String::as_str).unwrap_or(".");

    state.filesystem.cd(session, path).await?;

    Ok(CommandResult {
        stdout: String::new(),
    })
}

async fn cmd_mkdir(
    state: &AppState,
    session: &TerminalSession,
    args: &[String],
) -> anyhow::Result<CommandResult> {
    let path = args.first().ok_or_else(|| anyhow!("missing operand"))?;

    state.filesystem.mkdir(session, path).await?;

    Ok(CommandResult {
        stdout: String::new(),
    })
}

async fn cmd_touch(
    state: &AppState,
    session: &TerminalSession,
    args: &[String],
) -> anyhow::Result<CommandResult> {
    let path = args.first().ok_or_else(|| anyhow!("missing operand"))?;

    state.filesystem.touch(session, path).await?;

    Ok(CommandResult {
        stdout: String::new(),
    })
}

async fn cmd_cat(
    state: &AppState,
    session: &TerminalSession,
    args: &[String],
) -> anyhow::Result<CommandResult> {
    let path = args.first().ok_or_else(|| anyhow!("missing operand"))?;

    Ok(CommandResult {
        stdout: state.filesystem.cat(session, path).await?,
    })
}

async fn cmd_rm(
    state: &AppState,
    session: &TerminalSession,
    args: &[String],
) -> anyhow::Result<CommandResult> {
    let path = args.first().ok_or_else(|| anyhow!("missing operand"))?;

    state.filesystem.rm(session, path).await?;

    Ok(CommandResult {
        stdout: String::new(),
    })
}

async fn cmd_rmdir(
    state: &AppState,
    session: &TerminalSession,
    args: &[String],
) -> anyhow::Result<CommandResult> {
    let path = args.first().ok_or_else(|| anyhow!("missing operand"))?;

    state.filesystem.rmdir(session, path).await?;

    Ok(CommandResult {
        stdout: String::new(),
    })
}

async fn cmd_help() -> anyhow::Result<CommandResult> {
    Ok(CommandResult {
        stdout: String::from(
            r#"Carbon Terminal

Commands

help
echo
clear

pwd
ls
cd

mkdir
touch
cat
rm
rmdir"#,
        ),
    })
}
