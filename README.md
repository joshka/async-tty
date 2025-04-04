# async-tty crate

A prototype fully asynchronous TTY library for Rust.

Other libraries that do async IO on the TTY have some compromises:

- tokio's stdio wraps blocking IO with a thread and shouldn't be used for interactive apps
- termion is unix only
- crossterm is fairly complex due to having to support windows APIs and having a bunch of
  abstractions over the TTY events

This library aims to keep things simple. It will not support Windows 7, so does not need to
support the Windows console APIs.

Status: experimental, not ready for production use.
