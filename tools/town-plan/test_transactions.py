import tempfile
from pathlib import Path
import unittest
from transactions import parse_run, quantity


class ConfirmedTransactions(unittest.TestCase):
    def fixture(self, messages, gold):
        temp=tempfile.TemporaryDirectory();self.addCleanup(temp.cleanup)
        p=Path(temp.name)
        (p/'messages.log').write_text(messages)
        (p/'state.log').write_text(''.join(f'1 0,0 d0/0 hp20/20 au{g} lv1\n' for g in gold))
        return p

    def test_confirmed_buy_uses_removed_quantity_not_resulting_pack_total(self):
        p=self.fixture("0 1 0 # Currently in preferred store '5'.\n0 1 0 # Buying Potions of Cure Light Wounds {unseen}.\n1 1 0 You bought a Green Potion for 20 gold.\n1 1 0 You have 2 Potions of Cure Light Wounds (b).\n",[100,80])
        rows,missing=parse_run(p,'current','test','Warrior')
        self.assertEqual(len(rows),1);self.assertEqual(missing,[])
        self.assertEqual((rows[0]['item'],rows[0]['quantity'],rows[0]['gold_spent']),('Potion of Cure Light Wounds',1,20))
        self.assertEqual((rows[0]['gold_before'],rows[0]['gold_after'],rows[0]['wallet_verified']),(100,80,True))
        self.assertEqual(rows[0]['inventory_after'],'2 Potions of Cure Light Wounds')

    def test_executor_message_without_receipt_is_not_a_purchase(self):
        p=self.fixture("0 1 0 # Currently in preferred store '5'.\n0 1 0 # Buying Potions of Cure Light Wounds.\n",[100])
        rows,missing=parse_run(p,'current','test','Warrior')
        self.assertEqual(rows,[]);self.assertEqual(len(missing),1)

    def test_home_drop_and_zero_proceeds_market_sale_are_distinct(self):
        p=self.fixture("0 1 0 # Currently in preferred store '8'.\n0 1 0 # Selling Scroll of Word of Recall\n1 1 0 You drop a Scroll of Word of Recall (a).\n2 1 0 # Currently in preferred store '5'.\n2 1 0 # Selling Potion of Boldness\n3 1 0 You had a Potion of Boldness (a).\n4 1 0 You drop a Torch (a).\n",[100]*5)
        rows,missing=parse_run(p,'current','test','Warrior')
        self.assertEqual([r['kind'] for r in rows],['home_store','sell'])
        self.assertEqual([r['gold_received'] for r in rows],[0,0]);self.assertEqual(missing,[])

    def test_receipt_cost_must_match_observed_wallet(self):
        p=self.fixture("0 1 0 # Buying Potions of Healing.\n1 1 0 You bought a Potion of Healing for 20 gold.\n",[100,99])
        with self.assertRaisesRegex(ValueError,'wallet mismatch'):parse_run(p,'current','test','Warrior')

    def test_engine_prefix_rule_includes_unprefixed_armour(self):
        self.assertEqual(quantity('Soft Leather Armour [8,+0]'),1)
        self.assertEqual(quantity('3 Arrows (1d4) (+0,+0)'),3)
        self.assertEqual(quantity('a Pair of Leather Boots [2,+0]'),1)


if __name__=='__main__':unittest.main()
