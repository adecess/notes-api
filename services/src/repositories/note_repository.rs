use super::traits::NoteRepositoryTrait;
use crate::models::Note;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct NoteRepository {
    db: PgPool,
}

impl NoteRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl NoteRepositoryTrait for NoteRepository {
    async fn create(&self, user_id: Uuid, title: &str, content: &str) -> Result<Note, sqlx::Error> {
        let note = sqlx::query_as::<_, Note>(
            r#"
            INSERT INTO notes (user_id, title, content)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, title, content, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(title)
        .bind(content)
        .fetch_one(&self.db)
        .await?;

        Ok(note)
    }

    async fn find_note_by_id(
        &self,
        note_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Note>, sqlx::Error> {
        let user = sqlx::query_as::<_, Note>(
            r#"
            SELECT id, user_id, title, content, created_at, updated_at
            FROM notes
            WHERE id = $1
            AND user_id = $2
            "#,
        )
        .bind(note_id)
        .bind(user_id)
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }

    async fn find_all_notes(&self, user_id: Uuid) -> Result<Vec<Note>, sqlx::Error> {
        let notes = sqlx::query_as::<_, Note>(
            r#"
            SELECT id, user_id, title, content, created_at, updated_at
            FROM users
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.db)
        .await?;

        Ok(notes)
    }

    async fn update(
        &self,
        note_id: Uuid,
        title: Option<&str>,
        content: Option<&str>,
    ) -> Result<Option<Note>, sqlx::Error> {
        let note = sqlx::query_as::<_, Note>(
            r#"
            UPDATE notes
            SET title = COALESCE($2, title),
                content = COALESCE($3, content),
            WHERE id = $1
            RETURNING id, user_id, title, content, created_at, updated_at
            "#,
        )
        .bind(note_id)
        .bind(title)
        .bind(content)
        .fetch_optional(&self.db)
        .await?;

        Ok(note)
    }

    async fn delete(&self, note_id: Uuid) -> Result<Option<Note>, sqlx::Error> {
        let note = sqlx::query_as::<_, Note>(
            r#"
            DELETE FROM notes
            WHERE
            id = $1
            RETURNING id, user_id, title, content, created_at, updated_at
            "#,
        )
        .bind(note_id)
        .fetch_optional(&self.db)
        .await?;

        Ok(note)
    }
}
