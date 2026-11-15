import json
import tempfile
from pathlib import Path
import unittest
from transactions import parse_run, quantity, components, label


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

    def test_schema_nine_confirmation_log_follows_the_native_receipt(self):
        p=self.fixture("0 1 0 # Town plan executing action 1 at shop 4 slot 0\n1 1 0 You bought a Green Potion for 20 gold.\n1 1 0 You have 2 Potions of Cure Light Wounds (b).\n2 1 0 # Buying Potion of Cure Light Wounds {unseen}.\n",[100,80,80])
        rec={'v':9,'seq':1,'gturn':1,'paction':1,'act':[4,0,1,20],'for':2,'wares':[], 'tryon':[], 'capability_supplies':[]}
        (p/'plan.jsonl').write_text(json.dumps(rec)+'\n')
        rows,missing=parse_run(p,'live','test','Warrior')
        self.assertEqual(missing,[]);self.assertEqual(len(rows),1)
        self.assertTrue(rows[0]['wallet_verified']);self.assertEqual(rows[0]['plan_seq'],1)
        self.assertTrue(rows[0]['unseen_or_unknown_at_purchase'])
        self.assertEqual(rows[0]['item'],'Potion of Cure Light Wounds')

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

    def test_inventory_stack_names_do_not_split_identical_shot_and_wine_purchases(self):
        self.assertEqual(label('22 Iron Shots (1d4) (+0,+0)'), 'Iron Shot (1d4) (+0,+0)')
        self.assertEqual(label('3 Pints of Fine Wine'), 'Pint of Fine Wine')

    def test_engine_prefix_rule_includes_unprefixed_armour(self):
        self.assertEqual(quantity('Soft Leather Armour [8,+0]'),1)
        self.assertEqual(quantity('3 Arrows (1d4) (+0,+0)'),3)
        self.assertEqual(quantity('a Pair of Leather Boots [2,+0]'),1)

    def test_capability_records_explain_supplies_instead_of_calling_oil_unmapped(self):
        record={'v':7,'act':[0,2,1,3],'wares':[[0,2,8,0,1,20,-1,0,0]],'tryon':[],
                'capability_supplies':[[0,2,6,2500],[0,2,0,7500000]]}
        role,learning=components(record)
        self.assertEqual(role,'Light turns + Ranged damage')
        self.assertFalse(learning)


if __name__=='__main__':unittest.main()
