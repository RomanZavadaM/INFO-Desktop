import json
import unittest
from pathlib import Path


class LocaleCompletenessTest(unittest.TestCase):
    def test_all_locales_match_ukrainian_keys(self):
        root = Path(__file__).resolve().parents[1] / "src" / "info_desktop" / "locales"
        baseline = json.loads((root / "uk.json").read_text(encoding="utf-8"))
        self.assertTrue(baseline)
        for path in sorted(root.glob("*.json")):
            data = json.loads(path.read_text(encoding="utf-8"))
            self.assertEqual(set(data), set(baseline), f"Locale key mismatch: {path.name}")
            for key, value in data.items():
                self.assertIsInstance(value, str)
                self.assertTrue(value.strip(), f"Empty translation {path.name}:{key}")


if __name__ == "__main__":
    unittest.main()
