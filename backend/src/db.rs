use sqlx::{Pool, Sqlite};

use crate::Prompt;

pub struct DBPrompt {
    pub title: String,
    pub id: i64,
}

pub struct DBSegment {
    pub color: String,
    pub text: String,
}

pub async fn insert_prompt(prompt: Prompt, pool: &Pool<Sqlite>) -> Result<i64, sqlx::Error> {
    let id = sqlx::query!(r#"INSERT INTO prompts VALUES ( ?1, NULL )"#, prompt.title)
        .execute(pool)
        .await?
        .last_insert_rowid();

    for segment in prompt.segments.as_ref() {
        sqlx::query!(
            r#"INSERT INTO segments VALUES ( ?1, ?2, ?3 )"#,
            id,
            segment.0,
            segment.1
        )
        .execute(pool)
        .await?;
    }

    Ok(id)
}

pub async fn get_prompt(id: i64, pool: &Pool<Sqlite>) -> Result<Prompt, sqlx::Error> {
    let prompt: DBPrompt = sqlx::query_as!(DBPrompt, r#"SELECT * FROM prompts WHERE id=?1"#, id)
        .fetch_one(pool)
        .await?;

    let segments: Vec<DBSegment> = sqlx::query_as!(
        DBSegment,
        r#"SELECT color, text FROM segments WHERE prompt_id=?1"#,
        prompt.id
    )
    .fetch_all(pool)
    .await?;

    let segments: Box<[(String, String)]> = segments
        .iter()
        .map(|x| (x.color.clone(), x.text.clone()))
        .collect();

    Ok(Prompt {
        id: Some(prompt.id as u64),
        title: prompt.title,
        segments,
    })
}
