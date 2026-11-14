#!/usr/bin/env python3
"""Tests for class selection, profile safety and native-zoo export semantics."""
import copy
import json
from pathlib import Path
import tempfile
import unittest

import class_profiles as cp
from calibrate import plan


class Profiles(unittest.TestCase):
    def setUp(self):
        self.profiles = cp.load_profiles()

    def load(self, value):
        with tempfile.TemporaryDirectory() as temp:
            p = Path(temp) / 'profiles.json'
            p.write_text(json.dumps(value))
            return cp.load_profiles(p)

    def test_reference_has_shared_defaults_for_all_nine_classes(self):
        self.assertEqual(len(cp.CLASSES), 9)
        self.assertEqual(self.profiles['class_defaults'], {})
        values = [cp.resolve(self.profiles, 'reference', cls)[0] for cls in cp.CLASSES]
        self.assertTrue(all(v == self.profiles['base'] for v in values))
        self.assertEqual(values[0]['borg_town_target_depth'], 0)
        self.assertEqual(values[0]['borg_town_trip_turns'], 0)

    def test_probe_changes_only_the_named_class_and_parameter(self):
        expected = {'Mage': {'borg_town_w_gold': 50}, 'Ranger': {'borg_town_home_discount': 750},
                    'Blackguard': {'borg_town_risk': 1500}}
        before = copy.deepcopy(self.profiles)
        for cls in cp.CLASSES:
            values, sources = cp.resolve(self.profiles, 'class-selection-probe', cls)
            differences = {k: v for k, v in values.items() if v != self.profiles['base'][k]}
            self.assertEqual(differences, expected.get(cls, {}))
            for name in differences:
                self.assertTrue(sources[name].endswith(f'class_overrides.{cls}'))
        self.assertEqual(self.profiles, before)

    def test_precedence_is_explicit(self):
        j = copy.deepcopy(self.profiles)
        j['class_defaults'] = {'Mage': {'borg_town_w_gold': 30}}
        j['arms']['cash-cautious']['settings']['borg_town_w_gold'] = 60
        self.assertEqual(cp.resolve(j, 'reference', 'Mage')[0]['borg_town_w_gold'], 30)
        self.assertEqual(cp.resolve(j, 'cash-cautious', 'Mage')[0]['borg_town_w_gold'], 60)
        j['arms']['cash-cautious']['class_overrides'] = {'Mage': {'borg_town_w_gold': 80}}
        values, provenance = cp.resolve(j, 'cash-cautious', 'Mage')
        self.assertEqual(values['borg_town_w_gold'], 80)
        self.assertEqual(provenance['borg_town_w_gold'], 'arms.cash-cautious.class_overrides.Mage')
        self.assertEqual(cp.resolve(j, 'reference', 'Warrior')[0]['borg_town_w_gold'], 25)

    def test_native_export_resolves_every_class_to_the_same_effective_vector(self):
        exported = cp.export_zoo(self.profiles)['profiles']
        for arm in self.profiles['arms']:
            selector = exported[f'rs-town-{arm}-by-class-normal']
            for cls in cp.CLASSES:
                selected = selector['by_class'].get(cls, selector['otherwise'])
                self.assertEqual(exported[selected]['settings'], cp.resolve(self.profiles, arm, cls)[0])
                self.assertEqual(selected, cp.selected_profile(self.profiles, arm, cls))
        reference = exported['rs-town-reference-by-class-normal']
        self.assertEqual(reference['by_class'], {})
        self.assertEqual(reference['otherwise'], 'rs-town-reference-normal')

    def test_plan_pairs_classes_and_seeds_and_uses_real_menu_indices(self):
        frozen = plan(self.profiles, ['reference', 'class-selection-probe'], list(cp.CLASSES),
                      [901, 1901], 2000, Path('/tmp/profiles-test-only'))
        self.assertEqual(len(frozen['cells']), 36)
        self.assertEqual(len({c['out'] for c in frozen['cells']}), 36)
        for cell in frozen['cells']:
            self.assertIn('menu,', cell['command'][-1])
            self.assertEqual(cell['class_idx'], cp.CLASSES.index(cell['class']))
            self.assertEqual(cell['command'][5], '2000')
        self.assertEqual(cp.CLASSES[7:], ('Ranger', 'Blackguard'))

    def test_legacy_profile_file_is_accepted(self):
        old = {'schema': 1, 'base': self.profiles['base'], 'arms': {'reference': {}, 'shallow': {'borg_town_target_depth': 2, 'borg_town_trip_turns': 1000}}}
        normalized = self.load(old)
        self.assertEqual(cp.resolve(normalized, 'reference', 'Mage')[0], old['base'])
        self.assertTrue(normalized['arms']['shallow']['calibration_only'])

    def test_invalid_class_or_setting_cannot_silently_fall_back(self):
        for override in ({'Magge': {'borg_town_w_gold': 50}}, {'Mage': {'borg_town_w_goldd': 50}},
                         {'Mage': {'borg_town_risk': True}}, {'Mage': {'borg_town_risk': -1}},
                         {'Mage': {'borg_town_home_discount': 1001}}, {'Mage': {'borg_town_w_gold': '50'}}):
            with self.subTest(override=override):
                j = copy.deepcopy(self.profiles)
                j['class_defaults'] = override
                with self.assertRaises(ValueError):
                    self.load(j)

    def test_class_overrides_cannot_change_modes_or_enable_execution(self):
        for setting, value in [('borg_town_plan', 1), ('borg_town_no_stores', True),
                               ('borg_munchkin_start', True), ('borg_cheat_death', True)]:
            j = copy.deepcopy(self.profiles)
            j['class_defaults'] = {'Mage': {setting: value}}
            with self.assertRaises(ValueError):
                self.load(j)

    def test_missing_explicit_mode_or_non_shadow_base_is_rejected(self):
        for setting, value in [('borg_town_plan', 1), ('borg_town_no_stores', True),
                               ('borg_munchkin_start', True)]:
            j = copy.deepcopy(self.profiles)
            j['base'][setting] = value
            with self.assertRaises(ValueError):
                self.load(j)
        j = copy.deepcopy(self.profiles)
        del j['base']['borg_lunal_mode']
        with self.assertRaises(ValueError):
            self.load(j)

    def test_misspelled_profile_field_is_not_ignored(self):
        j = copy.deepcopy(self.profiles)
        j['class_default'] = {'Mage': {'borg_town_w_gold': 50}}
        with self.assertRaises(ValueError):
            self.load(j)

    def test_duplicate_json_keys_are_rejected(self):
        with tempfile.TemporaryDirectory() as temp:
            p = Path(temp) / 'profiles.json'
            p.write_text('{"schema": 2, "schema": 1}')
            with self.assertRaisesRegex(ValueError, 'duplicate'):
                cp.load_profiles(p)

    def test_report_retains_all_nine_classes_and_incomplete_cells(self):
        from class_report import report
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            frozen = plan(self.profiles, ['reference', 'class-selection-probe'], list(cp.CLASSES),
                          [901, 1901], 2000, root)
            (root / 'experiment.json').write_text(json.dumps(frozen))
            result = report(root)
            self.assertEqual(result['planned_cells'], 36)
            self.assertEqual(result['retained_cells'], 36)
            self.assertEqual(result['outcomes'], {'not_run': 36})
            self.assertEqual({row['class'] for row in result['per_class']}, set(cp.CLASSES))
            self.assertEqual(len(result['per_class']), 18)
            self.assertFalse(any(row['comparable'] for row in result['paired_comparisons']))

    def test_fixed_depth_cannot_accidentally_become_a_progression_class_default(self):
        j = copy.deepcopy(self.profiles)
        j['class_defaults'] = {'Mage': {'borg_town_target_depth': 2}}
        with self.assertRaisesRegex(ValueError, 'calibration_only'):
            self.load(j)
        for arm in j['arms'].values():
            arm['calibration_only'] = True
        allowed = self.load(j)
        self.assertEqual(cp.resolve(allowed, 'reference', 'Mage')[0]['borg_town_target_depth'], 2)

    def test_unknown_arm_and_class_are_errors(self):
        with self.assertRaises(ValueError):
            cp.resolve(self.profiles, 'typo', 'Mage')
        with self.assertRaises(ValueError):
            cp.resolve(self.profiles, 'reference', 'Wizard')


if __name__ == '__main__':
    unittest.main()
