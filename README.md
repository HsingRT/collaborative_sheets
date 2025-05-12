# Collaborative Sheets

## Build Instructions

To build this Rust project, make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

Then, in the project root directory, run:

```bash
cargo build
```

To run the project:

```bash
cargo run
```

To run tests:

```bash
cargo test
```

To check code formatting (optional):

```bash
cargo fmt -- --check
```

---

## Features

- User management: create and manage users
- Sheet creation: users can create new sheets
- Sheet editing: change and update content in sheets
- Sheet printing: output the current sheet
- Access control: per-user read-only or editable rights
- Sheet sharing: share sheets with other users
- Rational arithmetic support: perform `+`, `-`, `*`, `/` operations on rational numbers like `123.456`, `123`

Note: Features like access control and sheet sharing can be optionally disabled by modifying or removing related code (e.g., avoiding inheritance or object generation).