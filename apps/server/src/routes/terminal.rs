use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::{app::AppState, terminal::TerminalSession};

#[derive(Deserialize)]
pub struct TerminalRequest {
    pub command: String,
    pub cwd: String,
}

#[derive(Serialize)]
pub struct TerminalResponse {
    pub stdout: String,
    pub cwd: String,
}

pub async fn execute(
    State(state): State<AppState>,
    Json(request): Json<TerminalRequest>,
) -> Result<Json<TerminalResponse>, StatusCode> {
    let mut session = TerminalSession {
        user_id: "demo".into(),
        cwd: request.cwd.as_str().into(),
    };

    let mut parts = request.command.split_whitespace();

    let command = parts.next().unwrap_or("");

    let args: Vec<&str> = parts.collect();

    let stdout = match command {
        "pwd" => state.filesystem.pwd(&session),

        "ls" => {
            let path = args.first().copied().unwrap_or(".");

            let nodes = state.filesystem.ls(&session, path).await.map_err(|e| {
                println!("{e:?}");
                StatusCode::BAD_REQUEST
            })?;

            nodes
                .into_iter()
                .map(|n| n.name)
                .collect::<Vec<_>>()
                .join("\n")
        }

        "cd" => {
            let path = args.first().copied().unwrap_or(".");

            state
                .filesystem
                .cd(&mut session, path)
                .await
                .map_err(|_| StatusCode::BAD_REQUEST)?;

            String::new()
        }

        "mkdir" => {
            let path = args.first().copied().ok_or(StatusCode::BAD_REQUEST)?;

            state
                .filesystem
                .mkdir(&session, path)
                .await
                .map_err(|_| StatusCode::BAD_REQUEST)?;

            String::new()
        }

        "touch" => {
            let path = args.first().copied().ok_or(StatusCode::BAD_REQUEST)?;

            state
                .filesystem
                .touch(&session, path)
                .await
                .map_err(|_| StatusCode::BAD_REQUEST)?;

            String::new()
        }

        "cat" => {
            let path = args.first().copied().ok_or(StatusCode::BAD_REQUEST)?;

            state
                .filesystem
                .cat(&session, path)
                .await
                .map_err(|_| StatusCode::BAD_REQUEST)?
        }

        "rm" => {
            let path = args.first().copied().ok_or(StatusCode::BAD_REQUEST)?;

            state
                .filesystem
                .rm(&session, path)
                .await
                .map_err(|_| StatusCode::BAD_REQUEST)?;

            String::new()
        }

        "rmdir" => {
            let path = args.first().copied().ok_or(StatusCode::BAD_REQUEST)?;

            state
                .filesystem
                .rmdir(&session, path)
                .await
                .map_err(|_| StatusCode::BAD_REQUEST)?;

            String::new()
        }

        "echo" => args.join(" "),

        "help" => String::from(
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

        "clear" => String::new(),

        _ => format!("{command}: command not found"),
    };

    Ok(Json(TerminalResponse {
        stdout,
        cwd: session.cwd.to_string(),
    }))
}
