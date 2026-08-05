use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
pub enum NodeKind {
    File,
    Directory,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FilesystemNode {
    pub id: String,

    pub parent_id: Option<String>,

    pub user_id: String,

    pub name: String,

    pub kind: NodeKind,

    pub content: Option<String>,
}
