#!/usr/bin/env python3
"""Rewrite call sites in c2rust code when a function moves to a converted
module (M3 helper).

Library use (from a one-off script):

    import m3_calls as m
    m.rewrite_calls(files, "borg_cave_floor_grid",
                    lambda a: "crate::borg_cave_util::borg_cave_floor_grid(&*(%s))" % a[0])
    m.drop_imports(files, {"borg_cave_floor_grid", ...})

`rewrite_calls` finds `name(` (as a whole word, not preceded by `fn ` or
`::`), splits the arguments at top-level commas and replaces the call with
the template's result. `drop_imports` removes names from `use ...::{...};`
lists (and whole `use` lines that end up empty).
"""
import re


def _match(s, i, o, c):
    d = 0
    n = len(s)
    while i < n:
        ch = s[i]
        if ch == '"':
            i += 1
            while s[i] != '"':
                if s[i] == "\\":
                    i += 1
                i += 1
        elif ch == o:
            d += 1
        elif ch == c:
            d -= 1
            if d == 0:
                return i
        i += 1
    raise ValueError("unbalanced")


def split_args(s):
    args = []
    depth = 0
    cur = []
    i = 0
    while i < len(s):
        ch = s[i]
        if ch == '"':
            j = i + 1
            while s[j] != '"':
                if s[j] == "\\":
                    j += 1
                j += 1
            cur.append(s[i:j + 1])
            i = j + 1
            continue
        if ch in "([{":
            depth += 1
        elif ch in ")]}":
            depth -= 1
        if ch == "," and depth == 0:
            args.append("".join(cur).strip())
            cur = []
        else:
            cur.append(ch)
        i += 1
    last = "".join(cur).strip()
    if last:
        args.append(last)
    return [re.sub(r"\s+", " ", a) for a in args]


def rewrite_text(s, name, template):
    pat = re.compile(r"(?<![A-Za-z0-9_:])%s\s*\(" % re.escape(name))
    out = []
    i = 0
    count = 0
    while True:
        m = pat.search(s, i)
        if not m:
            out.append(s[i:])
            break
        # skip definitions `fn name(`
        before = s[max(0, m.start() - 3):m.start()]
        if before.endswith("fn "):
            out.append(s[i:m.end()])
            i = m.end()
            continue
        a = m.end() - 1
        b = _match(s, a, "(", ")")
        args = split_args(s[a + 1:b])
        # rewrite nested calls inside the arguments first
        args = [rewrite_text(x, name, template)[0] for x in args]
        out.append(s[i:m.start()])
        out.append(template(args))
        i = b + 1
        count += 1
    return "".join(out), count


def rewrite_calls(files, name, template):
    total = 0
    for f in files:
        with open(f) as fh:
            s = fh.read()
        s2, n = rewrite_text(s, name, template)
        if n:
            with open(f, "w") as fh:
                fh.write(s2)
        total += n
    return total


def drop_imports(files, names):
    def fix(m):
        pub, path, body = m.group(1) or "", m.group(2), m.group(3)
        if body.startswith("{"):
            lst = [x.strip() for x in body[1:-1].split(",") if x.strip()]
        else:
            lst = [body]
        keep = [x for x in lst if x not in names]
        if not keep:
            return ""
        if len(keep) == len(lst):
            return m.group(0)
        return "%suse %s::{%s};" % (pub, path, ", ".join(keep))

    for f in files:
        with open(f) as fh:
            s = fh.read()
        s2 = re.sub(r"(pub )?use ([A-Za-z0-9_:]+)::(\{[^}]*\}|[A-Za-z0-9_]+);", fix, s)
        if s2 != s:
            with open(f, "w") as fh:
                fh.write(s2)


def drop_header_reexports(header_file, module):
    """Remove `pub use crate::c2rust::<module>::x;` lines from a header."""
    with open(header_file) as fh:
        s = fh.read()
    s = re.sub(r"pub use crate::c2rust::%s::\w+;\n" % re.escape(module), "", s)
    with open(header_file, "w") as fh:
        fh.write(s)
