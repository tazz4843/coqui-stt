# Version 1.0.0 (4.7.2022)
* Fix Undefined Behavior in `Stream`.
  This causes `Stream` to now require a mutable reference to a `Model` to be created.
  We now recommend using [deadpool](https://crates.io/crates/deadpool) to manage `Model`s.
* Implement `deadpool::managed::Manager` for `Model`.
  This allows `Model` to be used with [deadpool](https://crates.io/crates/deadpool).

# Version 0.3.3
* Fix double free in `Stream`. This should fix some segfaults users may have.
* Fix the wrong function being called in `Stream::finish_stream_with_metadata`.

# Version 0.3.2
* Add `threads` example
* Fix a small number of Clippy lints behind the scenes

# Version 0.3.1 (7.5.2022)
* Remove outdated documentation lines ([#10](https://github.com/tazz4843/coqui-stt/pull/10))
* Remove some unsafe code ([#11](https://github.com/tazz4843/coqui-stt/pull/11))

# Version 0.3.0 (29.4.2022)
* Finally fix Windows build ([#8](https://github.com/tazz4843/coqui-stt/pull/8))
* Add `Send` and `Sync` to `Stream` type
* Change all functions on `Stream` that access the C API to take `&mut self`
* Remove `ThreadSafeStream`
* Add docs for compiling and running with `libstt`

# Version 0.2.3 (25.3.2022)
* Update Coqui-STT to version 1.3.0
* Add new functions `Model::new_from_buffer`
  and `Model::enable_external_scorer_from_buffer`

# Version 0.2.2 (5.3.2022)
* Attempt fixing a corruption issue when stream ends.
* Add build.rs to allow this to build on Windows platforms
 (see [issue 3](https://github.com/tazz4843/coqui-stt/issues/3))

# Version 0.2.1 (8.2.2022)
* Adds a new ``ThreadSafeStream`` type,
  locked behind the ``threadsafe-streams`` feature flag.
  This is a workaround for ``Stream``s no longer implementing ``Send + Sync``.
* Expose new functions introduced in ``coqui-stt`` v1.2.0.

# Version 0.2.0 (8.2.2022)

* Removes accidentally added `Send + Sync` implementations for Stream.
  These were mistakenly added when they were wildly unsafe to add.
* Very basic logging for some things added.
