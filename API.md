# API Spec

This is a high-level design/spec for the API

## Response Format

The API responds using a standard format:

A successful response contains an array of data, and a message

```json
{
    "data": ["...some data..."],
    "message": "OK"
}
```

A failure response contains an array of errors, and a message

```json
{
    "message": "NOT OK",
    "errors": ["An error occurred"]
}
```

## Voice

### GET /voices

Get a list of voices configured for this instance

### GET /voices/{id}

Get a single voice, by id

### POST /voices

Create a new voice

### PUT /voices/{voice_id}

Save a voice

## Conversation

### GET /conversations?user_id={user_id}

Get a list of conversations, by user id

### GET /conversations/{conversation_id}

Get a single conversation, by id

### POST /conversations

Create a new conversation

### PUT /conversations/{conversation_id}

Save a conversation

### DELETE /conversations/{conversation_id}

Delete a conversation AND all associated messages

## Message

### GET /messages?conversation_id={conversation_id}

Get all messages associated with a conversation

### GET /messages/{message_id}

Get a single message, by id

### POST /messages

Create a new message

### PUT /messages/{message_id}

Save a message
