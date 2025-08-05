# Angi

## What?

- A scripting language lua-like, simple, fast, static-typed.
- A generate tool that generate backend-service code (transpile, meta programming?).
- Declarative backend language (or framework???)

## Why?

- I love the simplicity of lua, but I want static-type.
- I want write a server with minimal effort. Everything is built-in, hyper extendable.

## Architecture

### Flow

```mermaid
flowchart LR
 subgraph s1["Angi Compiler"]
    direction LR
        n4["Parser/Code gen"]
        n5["Rust .rs"]
        n6["Rustc/Cargo"]
  end
    A[".ag file"] --> s1
    n4 --> n5
    n5 --> n6
    s1 --> n1["binary_output (production ready)"]
```
