# INFO Desktop credential rotation

The server owns credential lifetime.

Desktop stores the non-secret expiry timestamp locally. On connect, if the
credential expires within seven days, Desktop requests rotation before loading
newsroom data.

Rotation:
- authenticates with current device credential;
- receives replacement secret;
- replaces the secret in OS secure storage;
- updates only the non-secret expiry timestamp in local settings.

The server keeps the old secret valid for a short grace period, currently 60
minutes, so a transient client-side failure does not lock the editor out.

No user interaction is expected during normal rotation.
