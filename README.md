# RustFSM

A full static Rust finite state machine macro library.

Compatible with `no_std` and embedded environments.

---

- Build

```bash
cargo build
```

- Generate doc

```bash
cargo doc --open
```

## Add Dependency

Add crate to your project

```bash
cargo add rustfsm --git ssh://git@bitbucket01.somfytech.com:7999/sne/rustfsm.git
```

Other arguments are also available:

- `--branch`: branch to use when adding from git
- `--tag`: tag to use when adding from git
- `--rev`: specific commit to use when adding from git

## Example

An example is available in `examples/mario.rs` with a well known state machine.

```bash
cargo run --example mario
```
