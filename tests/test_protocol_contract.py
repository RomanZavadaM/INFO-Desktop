import unittest

from info_desktop.protocol import PROTOCOL_ID


class ProtocolContractTest(unittest.TestCase):
    def test_protocol_identifier_is_v1(self):
        self.assertEqual(PROTOCOL_ID, "info-desktop-protocol/1")


if __name__ == "__main__":
    unittest.main()
