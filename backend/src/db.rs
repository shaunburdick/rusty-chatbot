use std::str::FromStr;

use chrono::Utc;
use models::{Voice, Conversation, Message, Author};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, Error, Row, QueryBuilder};
use uuid::Uuid;

#[derive(Clone)]
pub struct DB {
    pool: SqlitePool,
}

impl DB {
    /// Creates a new instance of DB
    ///
    /// Arguments:
    /// - url: an SQLite connection string
    pub async fn new(url: &str) -> Result<Self, Error> {
        let pool = SqlitePool::connect(url).await?;

        Ok(DB { pool })
    }

    /// Asserts the database schema,
    /// creating tables and indexes as needed
    ///
    /// See DATABASE.md for schema reference
    pub async fn assert_schema(&self) -> Result<(), Error> {
        let schema = r#"
            CREATE TABLE IF NOT EXISTS "voice" (
                "id"            TEXT NOT NULL UNIQUE,
                "name"          TEXT NOT NULL,
                "description"   TEXT NOT NULL,
                "prefix"        TEXT NOT NULL,
                "created_at"    INTEGER NOT NULL,
                "deleted_at"    INTEGER,
                PRIMARY KEY("id")
            );

            CREATE INDEX IF NOT EXISTS "enabled_voices" ON "voice" (
                "deleted_at" ASC,
                "id" ASC
            );

            CREATE TABLE IF NOT EXISTS "conversation" (
                "id"            TEXT NOT NULL UNIQUE,
                "user_id"       TEXT NOT NULL,
                "name"          TEXT NOT NULL,
                "voice_id"      TEXT NOT NULL,
                "created_at"    INTEGER NOT NULL,
                "deleted_at"    INTEGER,
                FOREIGN KEY("voice_id") REFERENCES "voice"("id"),
                PRIMARY KEY("id")
            );

            CREATE INDEX IF NOT EXISTS "enabled_user_conversations" ON "conversation" (
                "deleted_at" ASC,
                "user_id" ASC
            );

            CREATE TABLE IF NOT EXISTS "message" (
                "id"              TEXT NOT NULL UNIQUE,
                "conversation_id" TEXT NOT NULL,
                "author"          TEXT NOT NULL,
                "content"         TEXT NOT NULL,
                "created_at"      INTEGER NOT NULL,
                "deleted_at"      INTEGER,
                FOREIGN KEY("conversation_id") REFERENCES "conversation"("id"),
                PRIMARY KEY("id")
            );

            CREATE INDEX IF NOT EXISTS "enabled_messages_by_conversations" ON "message" (
                "deleted_at" ASC,
                "conversation_id" ASC
            );
        "#;

        let mut connection = self.pool.acquire().await?;

        sqlx::query(schema).execute(&mut *connection).await?;

        Ok(())
    }

    /// Initializes the database with the following:
    /// - Inserts initial voices if the table is empty
    pub async fn init(&self) -> Result<(), Error> {
        let mut connection = self.pool.acquire().await?;

        let voice_count_query = r#"
            SELECT COUNT(*) as count
            FROM `voice`
        "#;

        let voice_count = sqlx::query(voice_count_query)
            .fetch_one(&mut *connection)
            .await?
            .get::<i64, &str>("count");

        if voice_count == 0 {
            let initial_voices = vec![
                Voice {
                    id: Uuid::new_v4().to_string(),
                    name: "Shaun Burdick".to_string(),
                    description: "The developer of this tool".to_string(),
                    prefix: "A software developer; Learning Rust; Too busy to focus on you;".to_string(),
                    created_at: Utc::now().timestamp_micros(),
                    deleted_at: None
                },
                Voice {
                    id: Uuid::new_v4().to_string(),
                    name: "Gwen Burdick".to_string(),
                    description: "My dog".to_string(),
                    prefix: "A dog; Just discovered the English language; Learned how to type; Just happy to be here;".to_string(),
                    created_at: Utc::now().timestamp_micros(),
                    deleted_at: None
                },
            ];

            let mut bulk_voice_query = QueryBuilder::new(r#"
                INSERT INTO voice (id, name, description, prefix, created_at)
            "#);

            bulk_voice_query.push_values(initial_voices.iter(), |mut b, voice| {
                b.push_bind(&voice.id);
                b.push_bind(&voice.name);
                b.push_bind(&voice.description);
                b.push_bind(&voice.prefix);
                b.push_bind(voice.created_at);
            });

            let query = bulk_voice_query.build();

            query.execute(&mut *connection).await?;
        }

        Ok(())
    }

    /// Fetches voices from database
    ///
    /// Arguments:
    /// - deleted: include deleted voices
    pub async fn get_voices(&self, deleted: bool) -> Result<Vec<Voice>, Error> {
        let sql = format!(r#"
            SELECT `id`, `name`, `description`, `prefix`, `created_at`, `deleted_at`
            FROM `voice`
            WHERE `deleted_at` IS {}
        "#, if deleted { "NOT NULL"} else { "NULL" });

        let mut connection = self.pool.acquire().await?;
        let rows = sqlx::query(&sql)
            .map(|row| DB::row_to_voice(&row))
            .fetch_all(&mut *connection)
            .await?;

        Ok(rows)
    }

    /// Fetches a voice by ID
    ///
    /// Arguments:
    /// - id: the id of the voice
    pub async fn get_voice(&self, id: &String) -> Result<Voice, Error> {
        let sql = String::from(r#"
            SELECT `id`, `name`, `description`, `prefix`, `created_at`, `deleted_at`
            FROM `voice`
            WHERE `id` = ?
        "#);

        let mut connection = self.pool.acquire().await?;

        sqlx::query(&sql)
            .bind(id)
            .map(|row| DB::row_to_voice(&row))
            .fetch_one(&mut *connection)
            .await
    }

    /// Saves a voice to the database, will upsert
    ///
    /// Arguments:
    /// - voice: The voice struct to be saved
    pub async fn save_voice(&self, voice: &Voice) -> Result<bool, Error> {
        let mut connection = self.pool.acquire().await?;

        let rows_affected = sqlx::query(r#"
            INSERT INTO `voice` (id, name, description, prefix, created_at, deleted_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ON CONFLICT (id)
            DO UPDATE SET
                name = excluded.name,
                description = excluded.description,
                prefix = excluded.prefix,
                deleted_at = excluded.deleted_at
        "#)
        .bind(&voice.id)
        .bind(&voice.name)
        .bind(&voice.description)
        .bind(&voice.prefix)
        .bind(voice.created_at)
        .bind(voice.deleted_at)
        .execute(&mut *connection)
        .await?
        .rows_affected();

        Ok(rows_affected == 1)
    }

    /// Set the deleted_at timestamp for a voice
    ///
    /// Arguments:
    /// - voice_id: The id of the voice to "delete"
    // pub async fn delete_voice(&self, voice_id: &String) -> Result<bool, Error> {
    //     let mut connection = self.pool.acquire().await?;

    //     let rows_affected = sqlx::query(r#"
    //         UPDATE `voice`
    //         SET `deleted_at` = ?1
    //         WHERE `id` = ?2
    //     "#)
    //     .bind(Utc::now().timestamp())
    //     .bind(voice_id)
    //     .execute(&mut *connection)
    //     .await?
    //     .rows_affected();

    //     Ok(rows_affected == 1)
    // }

    /// Fetches conversations from database
    ///
    /// Arguments:
    /// - user_id: the id of the user in the conversation
    /// - deleted: include deleted voices
    pub async fn get_conversations(&self, user_id: &String, deleted: bool) -> Result<Vec<Conversation>, Error> {
        let sql = format!(r#"
            SELECT `id`, `user_id`, `name`, `voice_id`, `created_at`, `deleted_at`
            FROM `conversation`
            WHERE `deleted_at` IS {}
                AND `user_id` = ?
        "#, if deleted { "NOT NULL"} else { "NULL" });

        let mut connection = self.pool.acquire().await?;
        let rows = sqlx::query(&sql)
            .bind(user_id)
            .map(|row| DB::row_to_conversation(&row))
            .fetch_all(&mut *connection)
            .await?;

        Ok(rows)
    }

    /// Fetches a conversation by ID
    ///
    /// Arguments:
    /// - id: the id of the voice
    pub async fn get_conversation(&self, id: &String) -> Result<Conversation, Error> {
        let sql = String::from(r#"
            SELECT `id`, `user_id`, `name`, `voice_id`, `created_at`, `deleted_at`
            FROM `conversation`
            WHERE `id` = ?
        "#);

        let mut connection = self.pool.acquire().await?;

        sqlx::query(&sql)
            .bind(id)
            .map(|row| DB::row_to_conversation(&row))
            .fetch_one(&mut *connection)
            .await
    }

    /// Saves a conversation to the database, will upsert
    ///
    /// Arguments:
    /// - conversation: The conversation struct to be saved
    pub async fn save_conversation(&self, conversation: &Conversation) -> Result<bool, Error> {
        let mut connection = self.pool.acquire().await?;

        let rows_affected = sqlx::query(r#"
            INSERT INTO `conversation` (id, user_id, name, voice_id, created_at, deleted_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ON CONFLICT (id)
            DO UPDATE SET
                user_id = excluded.user_id,
                name = excluded.name,
                voice_id = excluded.voice_id,
                deleted_at = excluded.deleted_at
        "#)
        .bind(&conversation.id)
        .bind(&conversation.user_id)
        .bind(&conversation.name)
        .bind(&conversation.voice_id)
        .bind(conversation.created_at)
        .bind(conversation.deleted_at)
        .execute(&mut *connection)
        .await?
        .rows_affected();

        Ok(rows_affected == 1)
    }

    /// Set the deleted_at timestamp for a conversation
    ///
    /// Arguments:
    /// - conversation_id: The id of the conversation to "delete"
    pub async fn delete_conversation(&self, conversation_id: &String) -> Result<bool, Error> {
        let mut connection = self.pool.acquire().await?;

        let rows_affected = sqlx::query(r#"
            UPDATE `conversation`
            SET `deleted_at` = ?1
            WHERE `id` = ?2
        "#)
        .bind(Utc::now().timestamp())
        .bind(conversation_id)
        .execute(&mut *connection)
        .await?
        .rows_affected();

        Ok(rows_affected == 1)
    }

    /// Fetches messages from database
    ///
    /// Arguments:
    /// - conversation_id: the id of the user in the conversation
    /// - deleted: include deleted voices
    pub async fn get_messages(&self, conversation_id: &String, deleted: bool) -> Result<Vec<Message>, Error> {
        let sql = format!(r#"
            SELECT `id`, `conversation_id`, `author`, `content`, `created_at`, `deleted_at`
            FROM `message`
            WHERE `deleted_at` IS {}
                AND `conversation_id` = ?
        "#, if deleted { "NOT NULL"} else { "NULL" });

        let mut connection = self.pool.acquire().await?;
        let rows = sqlx::query(&sql)
            .bind(conversation_id)
            .map(|row| DB::row_to_message(&row))
            .fetch_all(&mut *connection)
            .await?;

        Ok(rows)
    }

    /// Fetches a message by ID
    ///
    /// Arguments:
    /// - id: the id of the voice
    pub async fn get_message(&self, id: &String) -> Result<Message, Error> {
        let sql = String::from(r#"
            SELECT `id`, `conversation_id`, `author`, `content`, `created_at`, `deleted_at`
            FROM `message`
            WHERE `id` = ?
        "#);

        let mut connection = self.pool.acquire().await?;

        sqlx::query(&sql)
            .bind(id)
            .map(|row| DB::row_to_message(&row))
            .fetch_one(&mut *connection)
            .await
    }

    /// Saves a message to the database, will upsert
    ///
    /// Arguments:
    /// - message: The message struct to be saved
    pub async fn save_message(&self, message: &Message) -> Result<bool, Error> {
        let mut connection = self.pool.acquire().await?;

        let rows_affected = sqlx::query(r#"
            INSERT INTO `message` (id, conversation_id, author, content, created_at, deleted_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ON CONFLICT (id)
            DO UPDATE SET
                conversation_id = excluded.conversation_id,
                author = excluded.author,
                content = excluded.content,
                deleted_at = excluded.deleted_at
        "#)
        .bind(&message.id)
        .bind(&message.conversation_id)
        .bind(message.author.to_string())
        .bind(&message.content)
        .bind(message.created_at)
        .bind(message.deleted_at)
        .execute(&mut *connection)
        .await?
        .rows_affected();

        Ok(rows_affected == 1)
    }

    /// Set the deleted_at timestamp for a message
    ///
    /// Arguments:
    /// - message_id: The id of the message to "delete"
    // pub async fn delete_message(&self, message_id: &String) -> Result<bool, Error> {
    //     let mut connection = self.pool.acquire().await?;

    //     let rows_affected = sqlx::query(r#"
    //         UPDATE `message`
    //         SET `deleted_at` = ?1
    //         WHERE `id` = ?2
    //     "#)
    //     .bind(Utc::now().timestamp())
    //     .bind(message_id)
    //     .execute(&mut *connection)
    //     .await?
    //     .rows_affected();

    //     Ok(rows_affected == 1)
    // }

    /// Set the deleted_at timestamp for all messages in a conversation
    ///
    /// Arguments:
    /// - conversation_id: The id of the conversation to "delete" messages for
    pub async fn delete_messages_by_conversation(&self, conversation_id: &String) -> Result<bool, Error> {
        let mut connection = self.pool.acquire().await?;

        let rows_affected = sqlx::query(r#"
            UPDATE `message`
            SET `deleted_at` = ?1
            WHERE `conversation_id` = ?2
        "#)
        .bind(Utc::now().timestamp())
        .bind(conversation_id)
        .execute(&mut *connection)
        .await?
        .rows_affected();

        Ok(rows_affected == 1)
    }


    /// Converts an SQLite Row to a Voice
    ///
    /// Arguments:
    /// - row: The row in the DB
    fn row_to_voice(row: &SqliteRow) -> Voice {
        Voice {
            id: row.get::<String, &str>("id"),
            name: row.get::<String, &str>("name"),
            description: row.get::<String, &str>("description"),
            prefix: row.get::<String, &str>("prefix"),
            created_at: row.get::<i64, &str>("created_at"),
            deleted_at: row.get::<Option<i64>, &str>("deleted_at")
        }
    }

    /// Converts an SQLite Row to a Conversation
    ///
    /// Arguments:
    /// - row: The row in the DB
    fn row_to_conversation(row: &SqliteRow) -> Conversation {
        Conversation {
            id: row.get::<String, &str>("id"),
            user_id: row.get::<String, &str>("user_id"),
            name: row.get::<String, &str>("name"),
            voice_id: row.get::<String, &str>("voice_id"),
            created_at: row.get::<i64, &str>("created_at"),
            deleted_at: row.get::<Option<i64>, &str>("deleted_at")
        }
    }

    /// Converts an SQLite Row to a Message
    ///
    /// Arguments:
    /// - row: The row in the DB
    fn row_to_message(row: &SqliteRow) -> Message {
        Message {
            id: row.get::<String, &str>("id"),
            conversation_id: row.get::<String, &str>("conversation_id"),
            author: Author::from_str(&row.get::<String, &str>("author").to_string()).unwrap(),
            content: row.get::<String, &str>("content"),
            created_at: row.get::<i64, &str>("created_at"),
            deleted_at: row.get::<Option<i64>, &str>("deleted_at")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::Utc;

    #[sqlx::test]
    async fn test_db_assert_schema() {
        // Build test DB and run the assert_schema method
        let db = DB::new("sqlite::memory:").await.unwrap();
        db.assert_schema().await.unwrap();

        // Grab a connection from the DB object, and get a list of tables
        let table_query = r#"
            SELECT `name`
            FROM `sqlite_master`
            WHERE type='table'
            ORDER BY `name`
        "#;
        let mut connection = db.pool.acquire().await.unwrap();
        let tables = sqlx::query(table_query)
            .fetch_all(&mut *connection)
            .await
            .unwrap();

        // Compare tables to the expected list
        assert_eq!(tables.len(), 3);
        // assert_eq!(tables, vec![("voice",), ("conversation",), ("messages",)]);
    }

    #[sqlx::test]
    async fn test_db_assert_schema_can_run_twice() {
        // Build test DB and run the assert_schema method
        let db = DB::new("sqlite::memory:").await.unwrap();
        db.assert_schema().await.unwrap();
        db.assert_schema().await.unwrap();

        // Grab a connection from the DB object, and get a list of tables
        let table_query = r#"
            SELECT `name`
            FROM `sqlite_master`
            WHERE type='table'
            ORDER BY `name`
        "#;
        let mut connection = db.pool.acquire().await.unwrap();
        let tables = sqlx::query(table_query)
            .fetch_all(&mut *connection)
            .await
            .unwrap();

        // Compare tables to the expected list
        assert_eq!(tables.len(), 3);
        // assert_eq!(tables, vec![("voice",), ("conversation",), ("messages",)]);
    }

    #[sqlx::test]
    async fn test_db_init() {
        // create instance and assert schema
        let db = DB::new("sqlite::memory:").await.unwrap();
        db.assert_schema().await.unwrap();

        let mut connection = db.pool.acquire().await.unwrap();

        let voice_count_query = r#"
            SELECT COUNT(*) as count
            FROM `voice`
        "#;

        let voice_count = sqlx::query(voice_count_query)
            .fetch_one(&mut *connection)
            .await.unwrap()
            .get::<i64, &str>("count");

        // verify table is empty
        assert_eq!(voice_count, 0);

        // initialize db
        db.init().await.unwrap();

        let new_voice_count = sqlx::query(voice_count_query)
            .fetch_one(&mut *connection)
            .await.unwrap()
            .get::<i64, &str>("count");

        // verify table is populated now
        assert!(new_voice_count > 0);
    }

    #[sqlx::test]
    async fn test_db_crud_voice() {
        // Build test DB and run the assert_schema method
        let db = DB::new("sqlite::memory:").await.unwrap();
        db.assert_schema().await.unwrap();

        // create a voice and insert it into the database
        let mut voice = Voice::new(
            "Shaun".to_string(),
            "It's me".to_string(),
            "I'm boring".to_string()
        );

        let res = db.save_voice(&voice).await;
        assert!(res.unwrap());

        // Grab the record from the database
        let fetched_voice = db.get_voice(&voice.id).await;

        // And it should match
        assert_eq!(fetched_voice.unwrap(), voice);

        // Grab all the records from the database
        let voices = db.get_voices(false).await.unwrap();

        // Should be one record
        assert_eq!(voices.len(), 1);
        // And it should match
        assert_eq!(voices[0], voice);

        // "Delete" a voice
        voice.deleted_at = Some(Utc::now().timestamp());
        assert!(db.save_voice(&voice).await.unwrap());

        // It should no longer show up in list of voices
        let new_voices = db.get_voices(false).await.unwrap();
        assert_eq!(new_voices.len(), 0);

        // It should show up if you ask for deleted
        let deleted_voices = db.get_voices(true).await.unwrap();
        assert_eq!(deleted_voices.len(), 1);
    }

    #[sqlx::test]
    async fn test_db_crud_conversation() {
        // Build test DB and run the assert_schema method
        let db = DB::new("sqlite::memory:").await.unwrap();
        db.assert_schema().await.unwrap();

        // create a voice and insert it into the database
        let voice = Voice::new(
            "Shaun".to_string(),
            "It's me".to_string(),
            "I'm boring".to_string()
        );

        let voice_res = db.save_voice(&voice).await;
        assert!(voice_res.unwrap());

        // create a conversation and insert it into the database
        let mut conversation = Conversation::new(
            Uuid::new_v4().to_string(),
            "Test Conversation".to_string(),
            voice.id.clone()
        );
        let conversation_res = db.save_conversation(&conversation).await;
        assert!(conversation_res.unwrap());

        // Grab the record from the database
        let fetched_conversation = db.get_conversation(&conversation.id).await;

        // And it should match
        assert_eq!(fetched_conversation.unwrap(), conversation);

        // Grab all the records from the database
        let conversations = db.get_conversations(&conversation.user_id, false).await.unwrap();

        // Should be one record
        assert_eq!(conversations.len(), 1);
        // And it should match
        assert_eq!(conversations[0], conversation);

        // "Delete" a conversation
        conversation.deleted_at = Some(Utc::now().timestamp());
        assert!(db.save_conversation(&conversation).await.unwrap());

        // It should no longer show up in list of conversations
        let new_conversations = db.get_conversations(&conversation.user_id, false).await.unwrap();
        assert_eq!(new_conversations.len(), 0);

        // It should show up if you ask for deleted
        let deleted_conversations = db.get_conversations(&conversation.user_id, true).await.unwrap();
        assert_eq!(deleted_conversations.len(), 1);
    }

    #[sqlx::test]
    async fn test_db_crud_message() {
        // Build test DB and run the assert_schema method
        let db = DB::new("sqlite::memory:").await.unwrap();
        db.assert_schema().await.unwrap();

        // create a voice and insert it into the database
        let voice = Voice::new(
            "Shaun".to_string(),
            "It's me".to_string(),
            "I'm boring".to_string()
        );

        let voice_res = db.save_voice(&voice).await;
        assert!(voice_res.unwrap());

        // create a conversation and insert it into the database
        let conversation = Conversation::new(
            Uuid::new_v4().to_string(),
            "Test Conversation".to_string(),
            voice.id.clone()
        );
        let conversation_res = db.save_conversation(&conversation).await;
        assert!(conversation_res.unwrap());

        // create a message and insert it into the database
        let mut message = Message::new(
            conversation.id.clone(),
            Author::User,
            "This is a test message".to_string()
        );
        let message_res = db.save_message(&message).await;
        assert!(message_res.unwrap());

        // Grab the record from the database
        let fetched_message = db.get_message(&message.id).await;

        // And it should match
        assert_eq!(fetched_message.unwrap(), message);

        // Grab all the records from the database
        let messages = db.get_messages(&conversation.id, false).await.unwrap();

        // Should be one record
        assert_eq!(messages.len(), 1);
        // And it should match
        assert_eq!(messages[0], message);

        // "Delete" a message
        message.deleted_at = Some(Utc::now().timestamp());
        assert!(db.save_message(&message).await.unwrap());

        // It should no longer show up in list of messages
        let new_message = db.get_messages(&conversation.id, false).await.unwrap();
        assert_eq!(new_message.len(), 0);

        // It should show up if you ask for deleted
        let deleted_message = db.get_messages(&conversation.id, true).await.unwrap();
        assert_eq!(deleted_message.len(), 1);
    }
}
