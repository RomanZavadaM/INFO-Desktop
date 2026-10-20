from __future__ import annotations

import json
from pathlib import Path


SUPPORTED_LANGUAGES = ("uk", "en", "de", "es", "fr", "ko", "ja")
DEFAULT_LANGUAGE = "uk"


class Translator:
    def __init__(self, language: str = DEFAULT_LANGUAGE) -> None:
        self.language = language if language in SUPPORTED_LANGUAGES else DEFAULT_LANGUAGE
        self._base = self._load(DEFAULT_LANGUAGE)
        self._active = self._load(self.language)

    def _load(self, language: str) -> dict[str, str]:
        path = Path(__file__).with_name("locales") / f"{language}.json"
        with path.open("r", encoding="utf-8") as handle:
            return json.load(handle)

    def t(self, key: str) -> str:
        return self._active.get(key, self._base.get(key, key))
