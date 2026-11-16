import tempfile
from pathlib import Path
import unittest
from summarize_live_suite import metrics, diagnostics


class OutcomeReport(unittest.TestCase):
    def test_failures_are_visible_and_not_zero_level_survivors(self):
        rows = [dict(outcome='death', clevel=2, max_depth=1, gold=4, diagnostics={'town_fraction':.5}),
                dict(outcome='turn_cap', clevel=6, max_depth=4, gold=100, diagnostics={'town_fraction':.1}),
                dict(outcome='timeout')]
        m=metrics(rows)
        self.assertEqual((m['attempts'],m['valid'],m['deaths']),(3,2,1))
        self.assertEqual(m['failures'],{'timeout':1})
        self.assertEqual((m['mean_character_level'],m['mean_deepest_level'],m['reached_dl3']),(4,2.5,1))

    def test_recall_wait_note_is_not_a_trade_and_home_commands_are_separate(self):
        with tempfile.TemporaryDirectory() as d:
            p=Path(d)
            (p/'messages.log').write_text("# Buying time waiting for Recall.(3)\n# Currently in preferred store '8'.\n# Buying Potion of Healing {unseen}.\n# Selling 'decision only'\n# Selling Torch\n")
            (p/'state.log').write_text('0 0,0 d0/0 hp10/10\n10 0,0 d1/1 hp10/10\n30 0,0 d1/1 hp10/10\n')
            m=diagnostics(p,100)
            self.assertEqual((m['buy_commands'],m['sell_commands'],m['learning_buy_commands']),(1,1,1))
            self.assertEqual((m['home_take_commands'],m['home_store_commands']),(1,1))
            self.assertEqual((m['town_world_turns'],m['observed_world_turns']),(10,30))


if __name__=='__main__':unittest.main()
