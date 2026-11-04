#!/usr/bin/env python3
"""Post-process raw c2rust output so it builds on stable Rust.

* `extern_types`: `pub type foo;` items inside `extern "C" { }` blocks
  (opaque C structs) become `#[repr(C)] pub struct foo { _opaque: [u8; 0] }`.
* drops the `#![feature(...)]` lines (raw_ref_op is stable).

Usage: postprocess.py SRC_DIR   (rewrites *.rs in place)
"""
import os
import re
import sys

OPAQUE = re.compile(r"^\s*pub type ([A-Za-z_][A-Za-z0-9_]*);\s*$")


def process(text):
    out = []
    opaque = []
    in_extern = False
    for line in text.split("\n"):
        if line.startswith('extern "C" {'):
            in_extern = True
        elif in_extern and line.startswith("}"):
            in_extern = False
        m = OPAQUE.match(line) if in_extern else None
        if m:
            opaque.append(m.group(1))
            continue
        if line.startswith("#![feature("):
            continue
        out.append(line)
    text = "\n".join(out)
    # an extern block may now be empty
    text = re.sub(r'extern "C" \{\s*\}\n', "", text)
    if opaque:
        decls = "".join(
            "#[repr(C)]\npub struct %s {\n    _opaque: [u8; 0],\n}\n" % n
            for n in opaque)
        # insert after the leading attributes/use lines
        lines = text.split("\n")
        i = 0
        while i < len(lines) and (lines[i].startswith("#!") or
                                  lines[i].startswith("use ") or
                                  lines[i].strip() == ""):
            i += 1
        lines[i:i] = decls.rstrip("\n").split("\n") + [""]
        text = "\n".join(lines)
    return text


def main():
    d = sys.argv[1]
    for f in sorted(os.listdir(d)):
        if f.endswith(".rs"):
            p = os.path.join(d, f)
            with open(p) as fh:
                t = fh.read()
            t2 = process(t)
            if t2 != t:
                with open(p, "w") as fh:
                    fh.write(t2)


if __name__ == "__main__":
    main()
