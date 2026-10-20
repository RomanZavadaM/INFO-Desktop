# INFO Desktop queue pagination

The INFO server intentionally caps one queue page to 100 items.

INFO Desktop must therefore never assume one response is the whole newsroom
queue.

`queueAll()`:
- requests at most 100 items per page;
- follows `next_cursor` until the server reports the end;
- rejects a repeated cursor;
- has a hard page-count safety limit;
- returns one combined list to the UI.

This is required for real INFO production data. A queue with 121 pending items
must render all 121, not only the first 100.
