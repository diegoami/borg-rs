"""Boundary/clock controls for the trip outcome report."""
import unittest

from trip_policy_report import summarize


def state(turn, depth):
    return f"{turn} 1,1 d{depth}/5 hp10/10\n"


def town(turn, pack):
    return {"v": 3, "turn": turn, "depth": 0, "pack": pack,
            "traits": [[38, 2], [39, 4]], "gold": 100}


class TripReportTests(unittest.TestCase):
    def test_real_stays_not_repeated_policy_votes_and_native_clock(self):
        states = [state(1, 0), state(10, 1), state(20, 0), state(21, 0),
                  state(30, 1), state(40, 0)]
        trip = [{"v": 2, "turn": 999999, "game_turn": tick,
                 "decision": {"action": "return", "reasons": ["pack(free 2 <= 2)"]}}
                for tick in [15, 16, 17]]
        gear = [0, 21, 1, 1, 0, 2]
        rows = [town(1, []), town(20, [gear]), town(21, [gear]), town(29, []), town(40, [gear])]
        report = summarize(states, trip, rows)
        self.assertEqual(report["returns_observed"], 2)
        visits = report["town_stays"]
        self.assertTrue(visits[0]["initial"])
        self.assertEqual(visits[1]["inbound_game_turns"], 10)
        self.assertEqual(visits[1]["policy_return_reasons_observed"], ["pack(free 2 <= 2)"])
        self.assertEqual(visits[1]["observed_delta"]["unknown_gear_entries"], -1)
        self.assertFalse(visits[1]["unknown_gear_unchanged_between_snapshots"])
        self.assertIsNone(visits[2]["departure_game_turn"])
        self.assertIsNone(visits[2]["unknown_gear_unchanged_between_snapshots"])

    def test_missing_snapshots_are_not_zero_inventory(self):
        result = summarize([state(1, 1), state(2, 0), state(3, 1)], [], [])
        visit = result["town_stays"][0]
        self.assertFalse(visit["initial"])
        self.assertIsNone(visit["first_observed"])
        self.assertIsNone(visit["observed_delta"])

    def test_instant_depth_excursions_do_not_split_one_town_stay(self):
        states = [state(1, 0), state(2, 1), state(2, 0), state(3, 0),
                  state(4, 1), state(5, 0), state(5, 1), state(6, 1), state(7, 0)]
        report = summarize(states, [], [])
        self.assertEqual(report["returns_observed"], 1)
        self.assertEqual(report["same_turn_depth_changes"], 2)
        self.assertEqual(report["town_stays"][0]["departure_game_turn"], 4)
        self.assertEqual(report["town_stays"][1]["arrival_game_turn"], 7)

    def test_unchanged_unknown_gear_is_observed_without_classifying_visit(self):
        gear = [0, 21, 1, 1, 0, 2]
        result = summarize([state(1, 1), state(2, 0), state(5, 1)], [],
                           [town(2, [gear]), town(4, [gear])])
        visit = result["town_stays"][0]
        self.assertTrue(visit["unknown_gear_unchanged_between_snapshots"])
        self.assertNotIn("useful", visit)

    def test_wrong_clock_and_appended_runs_are_refused(self):
        with self.assertRaisesRegex(ValueError, "Borg turns"):
            summarize([], [{"v": 2, "turn": 99}], [])
        with self.assertRaisesRegex(ValueError, "backwards"):
            summarize([state(2, 1), state(1, 0)], [], [])
        with self.assertRaisesRegex(ValueError, "fresh run"):
            summarize([], [], [town(2, []), town(1, [])])


if __name__ == "__main__":
    unittest.main()
