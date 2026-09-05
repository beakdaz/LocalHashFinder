#!/usr/bin/env python3
"""Post-process SwiftyULP nmKikxo27T blobs via SwiftyStringDumper + Ktqk decode."""
from __future__ import annotations

import base64
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(r"C:\Users\beakd\Desktop\uplKit\SwiftyULP")
MODULE = ROOT / "-Module--023069ab-5e12-4f3e-b56a-119ef93dc26a-.cs"
MYNA = ROOT / "pX9yS0ilP15qj4Ak4KS" / "MyNA1Iid6j5htnA8XyE.cs"
EXE = Path(r"G:\[CRACKED.ST] SwiftyULP\[CRACKED.ST] SwiftyULP\SwiftyULP.exe")
DUMPER = Path(__file__).with_name("SwiftyStringDumper.exe")
OUT_JSON = Path(__file__).with_name("swifty_extracted.json")


def eval_expr(expr: str) -> int:
    expr = expr.strip()
    while "--" in expr and not expr.startswith("--"):
        expr = expr.replace("--", "+")
    if expr.startswith("--"):
        expr = expr[2:]
    return eval(expr, {"__builtins__": {}}, {}) & 0xFFFFFFFF


def ktqk(s: str) -> str:
    raw = base64.b64decode(s)
    return bytes((b ^ 0xBA) - 146 & 0xFF for b in raw).decode("utf-8", errors="replace")


def parse_offsets() -> list[tuple[str, int]]:
    text = MODULE.read_text(encoding="utf-8", errors="ignore")
    consts: dict[str, int] = {}
    for m in re.finditer(
        r"m_d13888ddcdfb4bf0af71d570d817ec50\.(m_[0-9a-f]+)\s*=\s*([^;]+);", text
    ):
        consts[m.group(1)] = eval_expr(m.group(2))
    my = MYNA.read_text(encoding="utf-8", errors="ignore")
    pat = re.compile(
        r"nmKikxo27T\((0x[0-9A-Fa-f]+|\d+)\s*\^\s*"
        r"_003CModule_003E_007B[^.]+\.m_d13888ddcdfb4bf0af71d570d817ec50\.(m_[0-9a-f]+)\)"
    )
    out: list[tuple[str, int]] = []
    i = 0
    for m in pat.finditer(my):
        off = int(m.group(1), 0) ^ consts[m.group(2)]
        label = ["ynyiO7kmBG", "CTEiqW5reR", "QnaiPwMkkB"][min(i // 12, 2)]
        out.append((label, off))
        i += 1
    return out


def dump_at_offsets(offsets: list[int]) -> dict[int, str]:
    if not DUMPER.exists():
        raise SystemExit(f"build SwiftyStringDumper.exe first: {DUMPER}")
    off_file = Path(__file__).with_name("_offsets.txt")
    off_file.write_text("\n".join(str(o) for o in offsets), encoding="utf-8")
    proc = subprocess.run(
        [str(DUMPER), str(EXE), str(off_file)],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
    )
    if proc.returncode != 0 and not proc.stdout:
        print(proc.stderr, file=sys.stderr)
        raise SystemExit(proc.returncode)
    raw: dict[int, str] = {}
    for line in proc.stdout.splitlines():
        if line.startswith("@") or "\t" in line:
            parts = line.split("\t", 1)
            if len(parts) == 2:
                off = int(parts[0].lstrip("@"), 16)
                raw[off] = parts[1]
    return raw


def main() -> None:
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    groups: dict[str, list[str]] = {
        "ynyiO7kmBG": [],
        "CTEiqW5reR": [],
        "QnaiPwMkkB": [],
        "all_decoded": [],
    }

    labeled = parse_offsets()
    unique_offs = sorted(set(o for _, o in labeled))
    print(f"unique offsets: {len(unique_offs)}")

    raw = dump_at_offsets(unique_offs)
    print(f"raw strings: {len(raw)}")

    decoded_by_off: dict[int, str] = {}
    for off, blob in raw.items():
        try:
            plain = ktqk(blob)
            decoded_by_off[off] = plain
            groups["all_decoded"].append(plain)
        except Exception:
            decoded_by_off[off] = blob

    for label, off in labeled:
        if off in decoded_by_off:
            groups[label].append(decoded_by_off[off])

    # dedupe preserving order
    for k in groups:
        seen: set[str] = set()
        uniq = []
        for s in groups[k]:
            if s not in seen:
                seen.add(s)
                uniq.append(s)
        groups[k] = uniq

    # heuristics: weak passwords = short alphanumeric in QnaiPwMkkB
    weak = [s for s in groups["QnaiPwMkkB"] if s.isascii() and 3 <= len(s) <= 32 and " " not in s]
    blacklist = [s for s in groups["CTEiqW5reR"] if "." in s or "mail" in s.lower()]
    misc = [s for s in groups["ynyiO7kmBG"] if s]

    result = {
        "groups": groups,
        "weak_passwords": weak,
        "domain_blacklist": blacklist,
        "misc_module_names": misc,
        "stats": {
            "weak_count": len(weak),
            "blacklist_count": len(blacklist),
            "misc_count": len(misc),
            "total_decoded": len(groups["all_decoded"]),
        },
    }

    OUT_JSON.write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
    print(json.dumps(result["stats"], indent=2))
    print(f"wrote {OUT_JSON}")
    for label in ("misc_module_names", "domain_blacklist", "weak_passwords"):
        print(f"\n=== {label} ===")
        for s in result[label][:30]:
            print(f"  {s!r}")


if __name__ == "__main__":
    main()
