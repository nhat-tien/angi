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
 subgraph s1["Compiler"]
    direction LR
        n4["Parser/Code gen"]
        n8[".bytecode"]
  end
 subgraph s2["Angi"]
        s1
        n9["AVM (Angi Virtual Machine)"]
        n10["Server"]
        n11["Pre-built binary"]
        n12["Inject .bytecode to binary"]
  end
    A[".ag file"] --> s1
    n4 --> n8
    n9 --> n11
    n10 --> n11
    n11 --> n12
    s1 --> n12
    n12 --> n1["binary_output (production ready)"]
```

## Acknowledgement

- I learn how to write a lexer from [Gleam Project](https://github.com/gleam-lang/gleam)
