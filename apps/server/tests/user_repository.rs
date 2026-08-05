use chrono::Utc;
use uuid::Uuid;

use server::{
    models::User,
    repository::{UserRepository, connect, sqlite::SqliteUserRepository},
};

#[tokio::test]
async fn create_and_find_user() -> anyhow::Result<()> {
    let db = connect("sqlite:file::memory:?cache=shared").await?;

    let repo = SqliteUserRepository::new(db);

    let user = User {
        id: Uuid::new_v4().to_string(),
        username: "alice".into(),
        password_hash: "hash".into(),
        created_at: Utc::now().to_rfc3339(),
    };

    repo.create(&user).await?;

    let fetched = repo
        .find_by_username("alice")
        .await?
        .expect("user should exist");

    assert_eq!(fetched.username, "alice");
    assert_eq!(fetched.id, user.id);

    Ok(())
}
