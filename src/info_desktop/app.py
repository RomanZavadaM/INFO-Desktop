from __future__ import annotations

import os
import sys
import tkinter as tk
from tkinter import messagebox, ttk

from .i18n import Translator
from .protocol import InfoServerClient, ProtocolError, ServerProfile


class InfoDesktopApp(tk.Tk):
    def __init__(self) -> None:
        super().__init__()
        self.tr = Translator(os.environ.get("INFO_DESKTOP_LANG", "uk"))
        self.title(self.tr.t("app.title"))
        self.geometry("1280x800")
        self.minsize(960, 640)

        self.client: InfoServerClient | None = None
        self.queue_by_row: dict[str, dict] = {}

        self.server_var = tk.StringVar(value=os.environ.get("INFO_DESKTOP_SERVER", ""))
        self.token_var = tk.StringVar(value=os.environ.get("INFO_DESKTOP_TOKEN", ""))
        self.status_var = tk.StringVar(value=self.tr.t("status.disconnected"))

        self._build_ui()

    def _build_ui(self) -> None:
        top = ttk.Frame(self, padding=8)
        top.pack(fill="x")

        ttk.Label(top, text=self.tr.t("server.address")).pack(side="left")
        ttk.Entry(top, textvariable=self.server_var, width=42).pack(side="left", padx=(6, 8))
        ttk.Label(top, text=self.tr.t("server.token")).pack(side="left")
        ttk.Entry(top, textvariable=self.token_var, show="•", width=24).pack(side="left", padx=(6, 8))
        ttk.Button(top, text=self.tr.t("server.connect"), command=self.connect_server).pack(side="left")
        ttk.Label(top, textvariable=self.status_var).pack(side="right")

        body = ttk.Panedwindow(self, orient="horizontal")
        body.pack(fill="both", expand=True, padx=8, pady=(0, 8))

        left = ttk.Frame(body, padding=4)
        center = ttk.Frame(body, padding=4)
        right = ttk.Frame(body, padding=4)
        body.add(left, weight=1)
        body.add(center, weight=3)
        body.add(right, weight=4)

        ttk.Label(left, text=self.tr.t("channels.title")).pack(anchor="w")
        self.channels = tk.Listbox(left, exportselection=False)
        self.channels.pack(fill="both", expand=True, pady=(6, 0))

        ttk.Label(center, text=self.tr.t("queue.title")).pack(anchor="w")
        self.queue = ttk.Treeview(
            center,
            columns=("time", "channel", "title"),
            show="headings",
            selectmode="browse",
        )
        self.queue.heading("time", text=self.tr.t("queue.time"))
        self.queue.heading("channel", text=self.tr.t("queue.channel"))
        self.queue.heading("title", text=self.tr.t("queue.headline"))
        self.queue.column("time", width=90, anchor="w")
        self.queue.column("channel", width=130, anchor="w")
        self.queue.column("title", width=420, anchor="w")
        self.queue.pack(fill="both", expand=True, pady=(6, 0))
        self.queue.bind("<<TreeviewSelect>>", self.open_selected_item)

        ttk.Label(right, text=self.tr.t("item.title")).pack(anchor="w")
        self.item_text = tk.Text(right, wrap="word", state="disabled")
        self.item_text.pack(fill="both", expand=True, pady=(6, 0))

    def connect_server(self) -> None:
        base_url = self.server_var.get().strip()
        if not base_url:
            messagebox.showwarning(self.tr.t("error.title"), self.tr.t("error.no_server"))
            return

        self.status_var.set(self.tr.t("status.connecting"))
        self.update_idletasks()
        client = InfoServerClient(ServerProfile(base_url=base_url, token=self.token_var.get().strip()))

        try:
            handshake = client.handshake()
            channels = client.channels()
            queue = client.queue(limit=200)
        except ProtocolError as exc:
            self.status_var.set(self.tr.t("status.disconnected"))
            messagebox.showerror(self.tr.t("error.title"), str(exc))
            return

        self.client = client
        self.status_var.set(
            f"{handshake.get('instance_name') or handshake.get('server_name') or base_url} · "
            f"{handshake.get('server_version', '?')}"
        )
        self._fill_channels(channels)
        self._fill_queue(queue["items"])

    def _fill_channels(self, channels: list[dict]) -> None:
        self.channels.delete(0, tk.END)
        self.channels.insert(tk.END, self.tr.t("channels.all"))
        for channel in channels:
            label = channel.get("name") or channel.get("code") or str(channel.get("id", ""))
            self.channels.insert(tk.END, label)

    def _fill_queue(self, items: list[dict]) -> None:
        for row in self.queue.get_children():
            self.queue.delete(row)
        self.queue_by_row.clear()

        for item in items:
            row = self.queue.insert(
                "",
                "end",
                values=(
                    item.get("published_at") or item.get("created_at") or "",
                    item.get("channel_name") or item.get("channel") or "",
                    item.get("title") or "",
                ),
            )
            self.queue_by_row[row] = item

    def open_selected_item(self, _event=None) -> None:
        selected = self.queue.selection()
        if not selected or self.client is None:
            return

        preview = self.queue_by_row.get(selected[0], {})
        item_id = preview.get("id")
        if item_id is None:
            return

        try:
            item = self.client.item(item_id)
        except ProtocolError as exc:
            messagebox.showerror(self.tr.t("error.title"), str(exc))
            return

        lines = [
            item.get("title") or "",
            "",
            item.get("text") or item.get("prepared_text") or item.get("original_text") or "",
            "",
            f"{self.tr.t('item.source')}: {item.get('source_name') or item.get('source') or ''}",
            f"{self.tr.t('item.channel')}: {item.get('channel_name') or item.get('channel') or ''}",
            f"{self.tr.t('item.language')}: {item.get('language') or ''}",
            f"{self.tr.t('item.media')}: {item.get('media_count', len(item.get('media') or []))}",
        ]
        self.item_text.configure(state="normal")
        self.item_text.delete("1.0", tk.END)
        self.item_text.insert("1.0", "\n".join(lines))
        self.item_text.configure(state="disabled")


def run() -> None:
    if sys.version_info < (3, 8):
        raise SystemExit("INFO Desktop requires Python 3.8 or newer.")
    app = InfoDesktopApp()
    app.mainloop()
