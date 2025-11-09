<h1 align="center">Angi</h1>

<p align="center">
  <img src="https://img.shields.io/github/last-commit/nhat-tien/angi"/>
  <img src="https://github.com/nhat-tien/angi/actions/workflows/test.yml/badge.svg" />
</p>

## Example
```
{
    port = 3030;
    routes = [
       {
           path = "/";
           handler = () => "Hello world";
       },
       {
           path = "/<id>";
           handler = (id: int) => "Hello {id}";
       }
    ]
}
```

## Build from source

```bash
cargo build -p angi --release
```

## Docs

[Documents](https://nhat-tien.github.io/angi/)

## Motivation

- Nix-for-be
- A scripting language lua-like, simple, fast, static-typed.
- A generate tool that generate backend-service code (transpile, meta programming?).
- Declarative backend language (or framework???)
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

- I learn how to write a lexer from [gleam-lang/gleam](https://github.com/gleam-lang/gleam)
- VM implement: [Stumblinbear/register-vm.rs](https://github.com/Stumblinbear/register-vm.rs) 
- [lambdavm/lvm](https://gitlab.com/lambdavm/lvm)


