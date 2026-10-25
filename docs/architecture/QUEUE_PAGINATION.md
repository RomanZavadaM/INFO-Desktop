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


## Client refresh policy (source 0.1.5)

The complete queue still uses cursor pagination; no page is intentionally dropped or capped at the first 100 items.

To keep the daily editor responsive:
- initial connect and explicit **Refresh queue** reconcile the complete queue;
- each fetched page can be rendered progressively while later cursor pages are still loading;
- opening an item fetches only that item's detail;
- **Save** writes the item and refreshes only that item's detail;
- **Save & next** saves and opens the next already-known queue item;
- **Publish / Skip / Defer** remove the completed row from the local pending list and open the next item instead of repeating handshake + channels + the entire queue;
- explicit refresh remains the reconciliation point for changes made by another editor or server-side background work.

This optimization changes only client behavior. Server pagination and the INFO Desktop protocol remain unchanged.
