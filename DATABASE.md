# Database Spec

This is the design/spec for the database.

## Voice

The options for voice in a conversation. A voice is a description of the responder in the conversation

-   id: UUID, The id of the voice
-   name: String, A name for the voice
-   description: String, A description of the voice
-   prefix: String, The LLM prefix description of the voice, used in the prompt
-   created_at: Datetime, When the voice was created
-   deleted_at: Datetime|null, When the voice was deleted

### Indexes

-   Primary Key: `id`
-   Enabled Voices: `id`, `deleted_at`

### Create Statement

```sql
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
```

### Typical queries

```sql
SELECT `id`, `name`, `description`, `prefix`, `created_at`
FROM `voice`
WHERE `deleted_at` = NULL
```

```sql
SELECT `id`, `name`, `description`, `prefix`, `created_at`
FROM `voice`
WHERE `deleted_at` = NULL AND `id` = ?
```

## Conversation

A conversation represents all the information about a conversation

-   id: UUID, The id of the conversation
-   user_id: UUID, The id of the user. Generated by the frontend and saved in browser.
-   name: String, A name for the conversation
-   voice_id: UUID, The id of the voice used. Reference to `voice`.`id`
-   created_at: Datetime, When the conversation was created
-   deleted_at: Datetime|null, When the conversation was deleted

### Indexes

-   Primary Key: `id`
-   Enabled User Conversations: `user_id`, `deleted_at`

### Create Statement

```sql
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
```

### Typical queries

```sql
SELECT `id`, `user_id`, `name`, `voice_id`, `created_at`
FROM `conversation`
WHERE `deleted_at` = NULL AND `user_id` = ?
```

## Message

A message is a bit of text as part of the conversation.

-   id: UUID, the id of the message
-   conversation_id: UUID, The id of the conversation this message is associated with. Reference to `conversation`.`id`
-   author: String, The author of the message. Typically `user` or `voice`
-   content: String, The content of the message
-   created_at: Datetime, When the message was created
-   deleted_at: Datetime|null, When the message was deleted

### Indexes

-   Primary Key: `id`
-   Enabled Messages by Conversation: `conversation_id`, `deleted_at`

### Create Statement

```sql
CREATE TABLE IF NOT EXISTS "messages" (
    "id"              TEXT NOT NULL UNIQUE,
    "conversation_id" TEXT NOT NULL,
    "author"          TEXT NOT NULL,
    "content"         TEXT NOT NULL,
    "created_at"      INTEGER NOT NULL,
    "deleted_at"      INTEGER,
    FOREIGN KEY("conversation_id") REFERENCES "conversation"("id"),
    PRIMARY KEY("id")
);

CREATE INDEX IF NOT EXISTS "enabled_messages_by_conversations" ON "messages" (
    "deleted_at" ASC,
    "conversation_id" ASC
);
```

### Typical queries

```sql
SELECT `id`, `conversation_id`, `author`, `content`, `created_at`
FROM `message`
WHERE `deleted_at` = NULL AND `conversation_id` = ?
ORDER BY `created_at` ASC
```