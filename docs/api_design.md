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
|-- SC2API
    |-- Request/Response
        |-- (protocol types)
```

## Bot

The agent used to infer **GameState** and take an **Action**. The **Bot** will
be notified of **GameState** via a `tick()`.

### Action

In-game actions that can be performed by a **Bot**.

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

Will send and receive **SC2API** request and response via a **Connection**.

## SC2API

The generated protocols from the [s2client-proto](https://github.com/Blizzard/s2client-proto)
Protocol Buffers. These protocols will primarily be utilized by the
**Remote**.
