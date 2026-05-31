# Concurrency

- Rust uses a **1:1 thread model** — each language-level thread maps directly to one OS thread. There is no green-thread runtime built into the standard library.
- The 1:1 model gives predictable performance and easy interop with C, but spawning many threads is more expensive than in languages with M:N green threads (e.g. Go).
- If M:N concurrency is needed, third-party async runtimes such as Tokio provide lightweight tasks on top of OS threads.
- Rust's ownership and type system guarantee thread safety at compile time, preventing data races without a runtime cost.
- Use `std::thread::spawn` to create a new OS thread; the closure passed to `spawn` takes ownership of the values it captures.
- `JoinHandle::join` waits for a thread to finish and returns its result.
- Message passing via channels (`std::sync::mpsc`) is the preferred way to share data between threads — send values, not references.
- When shared state is unavoidable, use `Arc<Mutex<T>>` — `Arc` for shared ownership across threads, `Mutex` for exclusive mutable access.
- The `Send` trait marks a type as safe to transfer to another thread; `Sync` marks it as safe to share via reference across threads.
- Most standard types implement `Send` and `Sync` automatically; types like `Rc<T>` and raw pointers do not.
- Use `move` closures with `thread::spawn` when the closure references data from the parent scope — `move` transfers ownership into the thread so the compiler can guarantee the data lives long enough.
- `RwLock<T>` allows multiple concurrent readers or one exclusive writer; prefer it over `Mutex` when reads heavily outnumber writes.
- `Mutex` does not prevent deadlocks — acquiring two locks in inconsistent order across threads can still deadlock.
- `mpsc::channel()` creates an unbounded channel (sender never blocks); `mpsc::sync_channel(n)` creates a bounded channel that back-pressures the sender when the buffer is full.
- `thread::Builder` lets you set a thread name and stack size before spawning, which helps with debugging and profiling.
- A panic inside a spawned thread does not crash the main thread; it surfaces as an `Err` only when `JoinHandle::join()` is called.
- Atomic types in `std::sync::atomic` (e.g. `AtomicUsize`, `AtomicBool`) allow lock-free shared state for simple counters and flags.
