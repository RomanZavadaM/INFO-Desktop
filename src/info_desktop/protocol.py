from __future__ import annotations

import json
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass
from typing import Any


PROTOCOL_ID = "info-desktop-protocol/1"


class ProtocolError(RuntimeError):
    pass


@dataclass(frozen=True)
class ServerProfile:
    base_url: str
    token: str = ""

    def normalized_base(self) -> str:
        return self.base_url.rstrip("/")


class InfoServerClient:
    """Read-only client for INFO Desktop Protocol v1."""

    def __init__(self, profile: ServerProfile, timeout: float = 10.0) -> None:
        self.profile = profile
        self.timeout = timeout

    def _get(self, path: str, query: dict[str, Any] | None = None) -> Any:
        base = self.profile.normalized_base()
        url = f"{base}{path}"
        if query:
            url += "?" + urllib.parse.urlencode(query)
        headers = {
            "Accept": "application/json",
            "User-Agent": "INFO-Desktop/0.1.0-dev",
        }
        if self.profile.token:
            headers["Authorization"] = f"Bearer {self.profile.token}"

        request = urllib.request.Request(url=url, headers=headers, method="GET")
        try:
            with urllib.request.urlopen(request, timeout=self.timeout) as response:
                raw = response.read()
        except urllib.error.HTTPError as exc:
            raise ProtocolError(f"HTTP {exc.code}: {exc.reason}") from exc
        except urllib.error.URLError as exc:
            raise ProtocolError(f"Connection failed: {exc.reason}") from exc

        try:
            return json.loads(raw.decode("utf-8"))
        except (UnicodeDecodeError, json.JSONDecodeError) as exc:
            raise ProtocolError("Server returned invalid JSON") from exc

    def handshake(self) -> dict[str, Any]:
        data = self._get("/api/info-desktop/v1/handshake")
        if not isinstance(data, dict):
            raise ProtocolError("Handshake response must be an object")
        if data.get("protocol") != PROTOCOL_ID:
            raise ProtocolError(
                f"Unsupported protocol: {data.get('protocol')!r}; expected {PROTOCOL_ID!r}"
            )
        return data

    def status(self) -> dict[str, Any]:
        data = self._get("/api/info-desktop/v1/status")
        if not isinstance(data, dict):
            raise ProtocolError("Status response must be an object")
        return data

    def channels(self) -> list[dict[str, Any]]:
        data = self._get("/api/info-desktop/v1/channels")
        if not isinstance(data, list):
            raise ProtocolError("Channels response must be a list")
        return data

    def queue(self, *, limit: int = 200, cursor: str | None = None) -> dict[str, Any]:
        query: dict[str, Any] = {"limit": limit}
        if cursor:
            query["cursor"] = cursor
        data = self._get("/api/info-desktop/v1/queue", query=query)
        if not isinstance(data, dict) or not isinstance(data.get("items"), list):
            raise ProtocolError("Queue response must contain an items list")
        return data

    def item(self, item_id: str | int) -> dict[str, Any]:
        encoded = urllib.parse.quote(str(item_id), safe="")
        data = self._get(f"/api/info-desktop/v1/queue/{encoded}")
        if not isinstance(data, dict):
            raise ProtocolError("Item response must be an object")
        return data
