### Error Handling
- 2 types of errors: Recoverable and Unrecoverable
- Recoverable: Result<T, E> type is used to handle recoverable errors. It allows the program to continue running even if an error occurs.
- Unrecoverable: Panic! macro is used to handle unrecoverable errors. It stops the program execution and prints an error message.

### Aborting
- Normally, rust panics, and then cleans things up from memory
- Can also abort, which means it will not clean up memory and will exit immediately
- To abort:
    - `Cargo.toml`:
    ```.toml
    [profile.release]
    panic = 'abort'
    ```

### Backtracing
- `RUST_BACKTRACE=1 cargo run`
- Run program allowing backtracing (error stack) if the program panics
- Reading: start from top, read down until you see the files you wrote
- Debug symbols must be enabled (they are by default with `cargo build` or `cargo run` without the --release flag)

### Recoverable Errors: Result<T, E>
