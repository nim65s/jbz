# Just Bacon Zellij

A [Zellij](https://zellij.dev/) plugin to display your [just](https://github.com/casey/just) commands wrapped in
[bacon](https://github.com/Canop/bacon).

Try it in a `zellij` session after a `cargo build`:
```
zellij action start-or-reload-plugin file:$PWD/target/wasm32-wasi/debug/jbz.wasm
```

![demo](./jbz.gif)

## Option

Optionnaly, with a `all=true` configuration option, jbz will look for `all:<whitespace-separated-actions>` and run those.

## Pre-built wasm

Github actions build the project mode and publish jbz.wasm on each release, so you can also use eg.:

```
# specific version:
zellij action start-or-reload-plugin https://github.com/nim65s/jbz/releases/download/v0.1.1/jbz.wasm
# latest:
zellij action start-or-reload-plugin https://github.com/nim65s/jbz/releases/latest/download/jbz.wasm
```
