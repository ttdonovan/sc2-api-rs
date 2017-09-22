# API Design

> Warning: This is work in progress and APIs are very likely to change.

This is a working draft of the API design for **s2-api**.

## Anatomy

```
s2_api
|-- Bot
    |-- Action
    |-- `tick()`
|-- Connection
|-- Engine
    |-- GameState
    |-- `run()`
|-- Remote
    |-- Controller
        |-- `act()`, `observe()`, `ping()`, ...
    |-- Protocol
        |-- `send()` and `receive()`
|-- RawProtobufAPI
    |-- SC2API Request/Response
        |-- (protocol types)
```

## Bot

The agent used to inspect **GameState** and take an **Action**. At every game
frame, the bot's `tick()` method will be called to let the bot inspect the
current game state and emit actions (if desired).

### Action

A dumb `enum` of all the actions a `Bot` can emit.

## Connection

Responsible for establishing the WebSocket connection to a StarCraft II running
process for sending and receiving messages. Used by the **Remote::Protocol**.

## Engine

Coordinates the connection and change of **GameState** to any **Bot** and
accepts an **Action** to be sent to the **Remote::Controller** via `run()`.

### GameState

A representation of the current game state of a running StarCraft II process.

## Remote

Handles the request and response cycle for a requested **Bot::Action**
between the **Engine** and **Connection**.

### Controller

Primarily used to `act()` and `observe()`. Also will be able to make
`ping()` and `game_info()` types of one-off requests.

### Protocol

Will send and receive raw protobuf **SC2API** request and response via a **Connection**.

## RawProtobufAPI

The generated protocols from the [s2client-proto](https://github.com/Blizzard/s2client-proto)
Protocol Buffers. These protocols will primarily be utilized by the
**Remote**.
