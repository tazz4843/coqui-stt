# Version 0.2.1 (8.2.2022)
* Adds a new ``ThreadSafeStream`` type,
  locked behind the ``threadsafe-streams`` feature flag.
  This is a workaround for ``Stream``s no longer implementing ``Send + Sync``.
* Expose new functions introduced in ``coqui-stt`` v1.2.0.

# Version 0.2.0 (8.2.2022)

* Removes accidentally added `Send + Sync` implementations for Stream.
  These were mistakenly added when they were wildly unsafe to add.
* Very basic logging for some things added.
