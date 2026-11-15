import unittest

from optimise_joint_policy import aggregate, score


def suite(deaths, level=3, depth=2):
    return [{"name": str(i), "valid": True,
             "summary": {"dead": i < deaths, "clevel": level, "max_depth": depth}}
            for i in range(45)]


class ObjectiveTests(unittest.TestCase):
    def test_survival_has_priority_over_progress(self):
        cautious = aggregate(suite(4, level=2, depth=1))
        rapid = aggregate(suite(5, level=20, depth=30))
        self.assertLess(score(cautious), score(rapid))

    def test_incomplete_candidate_cannot_win_by_not_dying(self):
        failed = suite(0, level=50, depth=99)
        failed[0]["valid"] = False
        self.assertGreater(score(aggregate(failed)), score(aggregate(suite(11))))
        self.assertFalse(aggregate(failed[:-1])["valid"])

    def test_means_include_dead_characters_and_dl3_uses_deepest(self):
        rows = suite(9, level=5, depth=4)
        for row in rows[:9]:
            row["summary"].update(clevel=1, max_depth=1)
        result = aggregate(rows)
        self.assertEqual(result["deaths"], 9)
        self.assertEqual(result["mean_character_level"], 4.2)
        self.assertEqual(result["mean_deepest_level"], 3.4)
        self.assertEqual(result["reached_dl3"], 36)


if __name__ == "__main__":
    unittest.main()
