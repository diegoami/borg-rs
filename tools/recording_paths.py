"""Persistent recording storage; linked worktrees share the sibling cemetery."""
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def cemetery_root(root=ROOT):
    root = Path(root).resolve()
    result = subprocess.run(['git', '-C', str(root), 'rev-parse', '--path-format=absolute', '--git-common-dir'],
                            capture_output=True, text=True)
    if result.returncode == 0:
        common = Path(result.stdout.strip())
        if common.name == '.git':
            return common.parent.parent / 'player-cemetery'
    return root.parent / 'player-cemetery'


def data_root_for(root=ROOT):
    return Path(os.environ['BORG_PLAY_DATA_DIR']).expanduser().resolve() if os.environ.get('BORG_PLAY_DATA_DIR') else cemetery_root(root)


if __name__ == '__main__':
    print(data_root_for())
