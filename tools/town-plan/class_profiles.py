#!/usr/bin/env python3
"""Shared defaults and optional class calibration, exported to native zoo profiles.

No class-specific policy is embedded in the Borg. Selection is done before
birth by the runner; state/HP/spell adaptation stays in the shared evaluator.
"""
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[2]
PROFILES = Path(__file__).with_name('zoo-profiles.json')
CLASSES = tuple(line[5:] for line in (ROOT / 'angband/lib/gamedata/class.txt').read_text().splitlines()
                if line.startswith('name:'))
MODE_KEYS = ('borg_munchkin_start', 'borg_munchkin_by_class', 'borg_lunal_mode', 'borg_self_lunal')
REGISTRY = {name: (kind, value == 'true' if kind == 'b' else int(value))
            for kind, name, value in re.findall(r'\b([ib])\(c"(borg_\w+)", (-?\d+|true|false)\)',
                                               (ROOT / 'borg-rs/src/borg_init.rs').read_text())}
TOWN_KEYS = tuple(name for name in REGISTRY if name.startswith('borg_town_') and
                  name not in ('borg_town_no_stores', 'borg_town_plan', 'borg_town_food_trip', 'borg_town_light_depth'))
RANGES = {
    'target_depth': (0, 100), 'trip_turns': (0, 1000000), 'depth_gain': (1, 100),
    'explore_turns': (1, 1000000), 'stair_turns': (0, 1000000), 'food_turns': (1, 1000000),
    'light_turns': (1, 1000000), 'risk': (0, 1000000), 'home_discount': (0, 1000),
    'power_scale': (1, 1000000000),
}


def fingerprint(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True).encode()).hexdigest()


def _settings(settings, where, class_only=False):
    if not isinstance(settings, dict):
        raise ValueError(f'{where}: expected a settings object')
    for name, value in settings.items():
        if name not in REGISTRY or (class_only and name not in TOWN_KEYS):
            raise ValueError(f'{where}: unsupported setting {name}')
        kind = REGISTRY[name][0]
        if type(value) is not (bool if kind == 'b' else int):
            raise ValueError(f'{where}: wrong type for {name}')
        if kind == 'i':
            low, high = RANGES.get(name.removeprefix('borg_town_'),
                                   (0, 2147483647) if name in TOWN_KEYS else (-2147483648, 2147483647))
            if not low <= value <= high:
                raise ValueError(f'{where}: {name} outside {low}..{high}')


def _classes(mapping, where):
    if not isinstance(mapping, dict):
        raise ValueError(f'{where}: expected a class-to-settings object')
    for name, settings in mapping.items():
        if name not in CLASSES:
            raise ValueError(f'{where}: unknown class {name!r}')
        _settings(settings, f'{where}.{name}', class_only=True)


def _unique_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f'duplicate profile key {key!r}')
        result[key] = value
    return result


def load_profiles(path=PROFILES):
    raw = json.loads(Path(path).read_text(), object_pairs_hook=_unique_object)
    if not isinstance(raw, dict) or set(raw) - {'schema', 'purpose', 'base', 'class_defaults', 'arms'}:
        raise ValueError('unknown profile-file fields')
    if raw.get('schema') not in (1, 2):
        raise ValueError('unsupported town profiles schema')
    if raw['schema'] == 1:
        raw = {**raw, 'schema': 2, 'class_defaults': {},
               'arms': {name: {'settings': settings, 'calibration_only': bool(settings.get('borg_town_target_depth', 0) or settings.get('borg_town_trip_turns', 0))}
                        for name, settings in raw['arms'].items()}}
    _settings(raw['base'], 'base')
    _classes(raw.get('class_defaults', {}), 'class_defaults')
    if not isinstance(raw.get('arms'), dict) or not raw['arms']:
        raise ValueError('arms must be a nonempty object')
    for name, arm in raw['arms'].items():
        if not re.fullmatch(r'[a-z0-9]+(?:-[a-z0-9]+)*', name):
            raise ValueError(f'unsafe arm name {name!r}')
        if not isinstance(arm, dict) or set(arm) - {'settings', 'class_overrides', 'description', 'probe_only', 'calibration_only'}:
            raise ValueError(f'{name}: unknown arm fields')
        if any(type(arm[k]) is not bool for k in ('probe_only', 'calibration_only') if k in arm):
            raise ValueError(f'{name}: probe/calibration flags must be booleans')
        if 'description' in arm and not isinstance(arm['description'], str):
            raise ValueError(f'{name}: description must be text')
        _settings(arm.get('settings', {}), f'arms.{name}', class_only=True)
        _classes(arm.get('class_overrides', {}), f'arms.{name}.class_overrides')
        for cls in CLASSES:
            settings, _ = resolve(raw, name, cls)
            if any(settings.get(key) is not False for key in MODE_KEYS):
                raise ValueError(f'{name}: normal mode must be explicit for {cls}')
            if (settings.get('borg_town_target_depth') != 0 or settings.get('borg_town_trip_turns') != 0) and not arm.get('calibration_only', False):
                raise ValueError(f'{name}: fixed destinations/durations require calibration_only; progression profiles use automatic ambition')
            if settings.get('borg_town_plan') != 0 or settings.get('borg_town_no_stores') is not False:
                raise ValueError(f'{name}: this runner accepts shadow calibration only')
    return raw


def resolve(profiles, arm_name, class_name):
    if class_name not in CLASSES:
        raise ValueError(f'unknown class {class_name!r}')
    if arm_name not in profiles['arms']:
        raise ValueError(f'unknown arm {arm_name!r}')
    arm = profiles['arms'][arm_name]
    settings, sources = {}, {}
    layers = [
        ('base', profiles['base']),
        (f'class_defaults.{class_name}', profiles.get('class_defaults', {}).get(class_name, {})),
        (f'arms.{arm_name}.settings', arm.get('settings', {})),
        (f'arms.{arm_name}.class_overrides.{class_name}', arm.get('class_overrides', {}).get(class_name, {})),
    ]
    for source, values in layers:
        settings.update(values)
        sources.update({key: source for key in values})
    return settings, sources


def export_zoo(profiles, arm_names=None):
    """Use existing borg-zoo ordinary + by_class/otherwise profiles.

    An unchanged class falls back to the shared ordinary profile. A class
    override becomes a complete ordinary profile, not an in-game class rule.
    """
    exported = {}
    for name in arm_names or profiles['arms']:
        arm = profiles['arms'][name]
        common = profiles['base'] | arm.get('settings', {})
        ordinary = f'rs-town-{name}-normal'
        description = ('Bounded fixed-journey calibration only: ' if arm.get('calibration_only') else '') + arm.get('description', name) + ' (shadow configuration; no live-policy outcome claim)'
        exported[ordinary] = {'description': description, 'settings': common}
        selected = {}
        for cls in CLASSES:
            values, _ = resolve(profiles, name, cls)
            if values != common:
                class_profile = f'rs-town-{name}-{cls.lower()}-normal'
                exported[class_profile] = {'description': description + f'; {cls}', 'settings': values}
                selected[cls] = class_profile
        exported[f'rs-town-{name}-by-class-normal'] = {
            'description': description + '; class selection with shared fallback',
            'by_class': selected, 'otherwise': ordinary}
    return {'schema': 'borg-zoo/profiles@1', 'profiles': exported}


def selected_profile(profiles, arm, cls):
    exported = export_zoo(profiles, [arm])['profiles']
    selector = exported[f'rs-town-{arm}-by-class-normal']
    return selector['by_class'].get(cls, selector['otherwise'])
