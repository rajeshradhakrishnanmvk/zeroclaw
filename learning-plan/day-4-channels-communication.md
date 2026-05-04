# Day 4: Channels & Communication

## Goals
- Understand the channel system architecture
- Learn how messaging platforms are integrated
- Explore the channel orchestrator
- Study message routing and media handling

## Morning Session (2-3 hours): Channel Architecture

### 1. Channel System Overview

```
┌─────────────────────────────────────────┐
│         Channel Trait (API)             │
└────────────────┬────────────────────────┘
                 │
    ┌────────────┴────────────────────────────┐
    │                                         │
┌───▼──────────┐                    ┌────────▼─────────┐
│   Channel    │                    │     Channel      │
│ Orchestrator │◄───────────────────│  Implementations │
│  (routing,   │                    │  (Telegram,      │
│   lifecycle, │                    │   Discord,       │
│   media)     │                    │   Slack, etc.)   │
└──────────────┘                    └──────────────────┘
```

### 2. Channel Trait Deep Dive

**File**: `crates/zeroclaw-api/src/channel.rs`

Study:
- `Channel` trait definition
- Message structure
- Send/receive patterns
- Media handling
- Error handling

### 3. Channel Implementations

**Directory**: `crates/zeroclaw-channels/src/` or `src/channels/`

Explore major channel implementations:

#### Telegram Channel
**File**: `src/channels/telegram.rs`

- Bot API integration
- Message types (text, photo, document, voice)
- Inline keyboards
- Webhook vs polling
- Media download/upload

#### Discord Channel
**File**: `src/channels/discord.rs`

- Discord bot integration
- Guild/channel management
- Embeds and rich messages
- Voice channel support
- Slash commands

#### Slack Channel
**File**: `src/channels/slack.rs`

- Slack API integration
- Workspace/channel management
- Interactive messages
- File sharing
- Event subscriptions

#### Matrix Channel
**File**: `src/channels/matrix.rs`

- Matrix protocol
- End-to-end encryption
- Federation support
- Room management

### 4. Reading Exercise: Channel Implementation Pattern

Pick one channel (e.g., Telegram) and trace:

1. How it implements the Channel trait
2. Authentication/initialization
3. Message sending logic
4. Message receiving logic
5. Media handling
6. Error handling

Document the pattern in your notes.

## Afternoon Session (2-3 hours): Channel Orchestrator

### 1. Orchestrator Architecture

**Directory**: `crates/zeroclaw-channels/src/orchestrator/` or similar

The orchestrator manages:
- Channel lifecycle (start/stop)
- Message routing
- Media pipeline
- Session management
- Debouncing

### 2. Key Orchestrator Components

#### Channel Lifecycle
Study how channels are:
- Initialized from config
- Started and stopped
- Health-checked
- Restarted on failure

#### Message Routing
**File**: Look for routing logic in orchestrator

Understand:
- How messages are routed to the agent
- How responses are routed back to channels
- Multi-channel scenarios
- Priority handling

#### Media Pipeline
Study media handling:
- Download from channels
- Format conversion
- Storage/caching
- Upload to channels
- Transcription (audio/video)
- OCR (images)

### 3. Session Management

**Files**: 
- `src/channels/session_store.rs`
- `src/channels/session_backend.rs`
- `src/channels/session_sqlite.rs`

Study:
- Session storage
- User context tracking
- Conversation continuity
- Session expiration

### 4. Debouncing & Rate Limiting

**File**: `src/channels/debounce.rs`

Understand:
- Why debouncing is needed
- How it prevents duplicate processing
- Rate limiting strategies
- Stall detection

**File**: `src/channels/stall_watchdog.rs`

Study:
- Stall detection
- Timeout handling
- Recovery mechanisms

## Evening Session (1-2 hours): Advanced Channel Features

### 1. Link Enrichment

**File**: `src/channels/link_enricher.rs`

Study:
- URL detection
- Metadata extraction
- Preview generation
- Content summarization

### 2. Transcription & TTS

**Files**:
- `src/channels/transcription.rs` - Speech-to-text
- `src/channels/tts.rs` - Text-to-speech

Understand:
- Audio format handling
- Transcription providers
- Voice synthesis
- Language support

### 3. Voice Features

**Files**:
- `src/channels/voice_call.rs` - Voice call handling
- `src/channels/voice_wake.rs` - Wake word detection

Study:
- Real-time audio processing
- Wake word detection
- Call management

### 4. Channel-Specific Features

Explore unique features:

#### Email Channel
**File**: `src/channels/email_channel.rs`

- IMAP/SMTP integration
- Attachment handling
- Threading

#### WhatsApp Channels
**Files**:
- `src/channels/whatsapp.rs`
- `src/channels/whatsapp_web.rs`
- `src/channels/whatsapp_storage.rs`

- Multiple WhatsApp integrations
- Media storage
- Status updates

#### IRC Channel
**File**: `src/channels/irc.rs`

- IRC protocol
- Channel management
- Nick handling

## Exercises

### Exercise 1: Channel Comparison Matrix

Create: `learning-plan/exercises/day-4/channel-comparison.md`

Compare 5 channels across:
- Authentication method
- Message types supported
- Media capabilities
- Real-time vs polling
- Rate limits
- Special features

### Exercise 2: Message Flow Diagram

Create: `learning-plan/exercises/day-4/message-flow.md`

Draw the complete flow:
1. User sends message on platform
2. Channel receives it
3. Orchestrator processes it
4. Agent handles it
5. Response generated
6. Routed back through orchestrator
7. Channel sends response
8. User receives it

### Exercise 3: Media Pipeline Analysis

Create: `learning-plan/exercises/day-4/media-pipeline.md`

Document:
- Supported media types
- Download process
- Conversion/processing
- Storage strategy
- Upload process

### Exercise 4: Channel Configuration

Create: `learning-plan/exercises/day-4/channel-configs.toml`

Write example configurations for:
- Telegram bot
- Discord bot
- Slack app
- Email channel
- Matrix homeserver

### Exercise 5: Code Reading - Orchestrator

Study the orchestrator and document:
1. How channels are registered
2. Message routing algorithm
3. Error handling strategy
4. Concurrent channel management
5. Graceful shutdown process

## Deep Dive: Multi-Channel Scenarios

### Scenario 1: Same User, Multiple Channels

How does ZeroClaw handle:
- User identity across channels
- Conversation continuity
- Preference management
- Context sharing

### Scenario 2: Channel Failover

What happens when:
- A channel goes down
- API rate limits hit
- Network issues occur
- How does recovery work

### Reading Exercise

Find and document:
1. Channel health checking
2. Automatic reconnection
3. Message queuing during downtime
4. User notification of issues

## Deep Dive: Webhook vs Polling

### Webhook Mode
Study webhook implementations:
- Endpoint registration
- Security (signature verification)
- Event handling
- Scalability

### Polling Mode
Study polling implementations:
- Poll interval
- Long polling
- Resource usage
- Reliability

### Comparison
Document pros/cons of each approach.

## Checkpoint Questions

Before moving to Day 5, ensure you can answer:

- [ ] What is the Channel trait's main purpose?
- [ ] Name 5 channel implementations
- [ ] What does the channel orchestrator do?
- [ ] How are messages routed to the agent?
- [ ] What is debouncing and why is it needed?
- [ ] How does session management work?
- [ ] What media types are supported?
- [ ] How does transcription work?
- [ ] What is the difference between webhook and polling?
- [ ] How are multiple channels managed concurrently?

## Exercises Summary

Create in `learning-plan/exercises/day-4/`:

1. **`channel-comparison.md`** - Compare channel implementations
2. **`message-flow.md`** - Complete message flow diagram
3. **`media-pipeline.md`** - Media handling documentation
4. **`channel-configs.toml`** - Example channel configurations
5. **`orchestrator-notes.md`** - Orchestrator deep dive notes
6. **`webhook-vs-polling.md`** - Comparison analysis

## Resources

- **Channel API**: `crates/zeroclaw-api/src/channel.rs`
- **Channel Implementations**: `src/channels/*.rs`
- **Orchestrator**: `crates/zeroclaw-channels/src/orchestrator/`
- **Session Management**: `src/channels/session_*.rs`
- **Tests**: `tests/integration/channel_*.rs`
- **Documentation**: `docs/book/src/` (channel setup guides)

## Key Concepts

1. **Channel Abstraction**: Unified interface for messaging platforms
2. **Orchestrator**: Central coordinator for all channels
3. **Media Pipeline**: Handling various media types
4. **Session Management**: Tracking user conversations
5. **Debouncing**: Preventing duplicate message processing
6. **Webhook vs Polling**: Two modes of receiving messages
7. **Multi-Channel**: Managing multiple platforms simultaneously

## Testing Channels

```bash
# Run channel tests
cargo test --package zeroclaw-channels

# Run integration tests
cargo test --test test_integration channel

# Specific channel tests
cargo test telegram
cargo test discord
```

## Next Steps

Once you've completed Day 4, proceed to:
**[Day 5: Tools & Runtime](./day-5-tools-runtime.md)**

---

**Time Estimate**: 5-7 hours total
**Difficulty**: ⭐⭐⭐⭐☆ (Advanced)