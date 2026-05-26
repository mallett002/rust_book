# Cargo

## Common commands
- `cargo new my_project` -> Create a new rust project with cargo (creates dir)
- `cargo build` -> Build the project (creates executable in debug dir for development)
    - creates an exectuable to run `./my_project`
- `cargo build --release` -> Build the project for a release (optimized)
- `cargo run` -> Build and run the project in one go
- `cargo check` -> Compile the project without building an executable

## Often
- `cargo check` -> often while developing
- `cargo run` -> often to actually run the code

## cargo lock
- `cargo build` creates cargo.lock file (uses these pinned versions on susequent builds)
- `cargo update` -> update cargo.lock file with new versions

## Docs on current dependencies
- `cargo doc --open` -> create docs on all your deps and open them in browser
