# Error handling

- Rust does not use exceptions in the same way as many other languages.
- Rust errors are commonly grouped into:
	- Recoverable errors: represented by `Result<T, E>` and handled by the caller.
	- Unrecoverable errors: handled by `panic!`, which stops normal execution of the current thread.
- A backtrace is similar to a stack trace in Java.
- `unwrap()` is a convenience method:
	- It returns the inner value for `Ok`/`Some`.
	- It panics for `Err`/`None`.
- `expect("message")` is similar to `unwrap()`, but lets you provide a custom panic message.
	- Use it when a failure would indicate a bug and you want clearer context in the panic output.
- Error propagation means returning an error to the caller to decide how to handle it.
- The `?` operator on `Result` (or `Option`) does two things:
	- It extracts the success value and continues.
	- It returns early on error/`None`.
- Prefer `Result` for expected failures. Use `panic!` for unrecoverable states or violated assumptions.

## Advanced notes

- Choose panic vs `Result` deliberately:
	- Use `Result` for expected runtime failures (I/O, parsing, network, user input).
	- Use `panic!` for bugs, broken invariants, or impossible states.
- At function boundaries, return `Result` so callers can decide recovery behavior.
- `?` also converts error types when possible (`From`), not just early-returning errors.
- In libraries, prefer typed error enums so callers can match and handle specific cases.
- In applications, add clear context to errors so logs explain what operation failed.
- Avoid `unwrap()` in production paths unless crashing is the intended behavior.
- Test failure paths as well as success paths.