#!/usr/bin/env python3
"""Join complete independent suite groups from one frozen image, without partial runs."""
import argparse
from datetime import datetime, timezone
import json
from pathlib import Path
import shutil

from validate import ALL_SUITES, reference_root, sha256, validate_group, values

p = argparse.ArgumentParser(description=__doc__)
p.add_argument('output', type=Path)
p.add_argument('collections', type=Path, nargs='+')
p.add_argument('--reference-root', type=Path, help='C oracle root; defaults to the recorded root or harness/reference')
a = p.parse_args()
assert not a.output.exists(), 'refusing existing output'
build = (a.collections[0] / 'BUILD_ID').read_bytes()
bid = values(a.collections[0] / 'BUILD_ID')
assert sha256(Path(bid['binary'])) == bid['binary_sha256'], 'coverage image changed'
for source in a.collections:
    assert (source / 'BUILD_ID').read_bytes() == build, 'different coverage images/builds'
references = reference_root(a.collections[0], a.reference_root)
groups, profiles, oracles = {}, [], {}
for suite in ALL_SUITES:
    sources = [source for source in a.collections
               if (source / suite / 'DONE').exists() and (source / suite / 'profiles.json').exists()]
    assert len(sources) == 1, (suite, 'require exactly one accepted complete group', sources)
    source = sources[0].resolve()
    profiles.extend(validate_group(source, suite, bid, references, oracles))
    groups[suite] = source
assert len(set(profiles)) == len(profiles), 'profile reused across groups'
a.output.mkdir(parents=True)
shutil.copyfile(a.collections[0] / 'BUILD_ID', a.output / 'BUILD_ID')
for suite, source in groups.items():
    (a.output / suite).symlink_to(source / suite, target_is_directory=True)
    shutil.copyfile(source / (suite + '.log'), a.output / (suite + '.log'))
    (a.output / (suite + '.diff')).write_text(oracles[suite]['stdout'])
(a.output / 'REFERENCE_ROOT').write_text(str(references) + '\n')
(a.output / 'ORACLES.json').write_text(json.dumps(oracles, indent=2, sort_keys=True) + '\n')
(a.output / 'COLLECTIONS.json').write_text(json.dumps({k: str(v) for k, v in groups.items()}, indent=2)+'\n')
(a.output / 'DONE').write_text(datetime.now(timezone.utc).isoformat()+'\n')
print(f'Joined {len(groups)} complete groups, {len(profiles)} distinct successful profiles from one image')
