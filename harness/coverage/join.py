#!/usr/bin/env python3
"""Join complete independent suite groups from one frozen image, without partial runs."""
import argparse
from datetime import datetime, timezone
import json
from pathlib import Path
import shutil

from validate import ALL_SUITES, sha256, validate_group, values

p = argparse.ArgumentParser(description=__doc__)
p.add_argument('output', type=Path)
p.add_argument('collections', type=Path, nargs='+')
a = p.parse_args()
assert not a.output.exists(), 'refusing existing output'
build = (a.collections[0] / 'BUILD_ID').read_bytes()
bid = values(a.collections[0] / 'BUILD_ID')
assert sha256(Path(bid['binary'])) == bid['binary_sha256'], 'coverage image changed'
for source in a.collections:
    assert (source / 'BUILD_ID').read_bytes() == build, 'different coverage images/builds'
groups, profiles = {}, []
for suite in ALL_SUITES:
    sources = [source for source in a.collections
               if (source / suite / 'DONE').exists() and (source / suite / 'profiles.json').exists()]
    assert len(sources) == 1, (suite, 'require exactly one accepted complete group', sources)
    source = sources[0].resolve()
    profiles.extend(validate_group(source, suite, bid))
    groups[suite] = source
assert len(set(profiles)) == len(profiles), 'profile reused across groups'
a.output.mkdir(parents=True)
shutil.copyfile(a.collections[0] / 'BUILD_ID', a.output / 'BUILD_ID')
for suite, source in groups.items():
    (a.output / suite).symlink_to(source / suite, target_is_directory=True)
    for suffix in ('.log', '.diff'):
        shutil.copyfile(source / (suite + suffix), a.output / (suite + suffix))
(a.output / 'COLLECTIONS.json').write_text(json.dumps({k: str(v) for k, v in groups.items()}, indent=2)+'\n')
(a.output / 'DONE').write_text(datetime.now(timezone.utc).isoformat()+'\n')
print(f'Joined {len(groups)} complete groups, {len(profiles)} distinct successful profiles from one image')
