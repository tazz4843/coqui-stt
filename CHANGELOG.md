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
