#!/usr/bin/env python3
"""Decode SwiftyULP nmKikxo27T offsets using static module constants."""
import ast
import re
import struct
from pathlib import Path

ROOT = Path(r"C:\Users\beakd\Desktop\uplKit\SwiftyULP")
MODULE = ROOT / "-Module--023069ab-5e12-4f3e-b56a-119ef93dc26a-.cs"
MYNA = ROOT / "pX9yS0ilP15qj4Ak4KS" / "MyNA1Iid6j5htnA8XyE.cs"
RESOURCE = ROOT / "IIcl2uv1D8DVp94l7q.HaonIHdScDIdOeJG68"


def eval_expr(expr: str) -> int:
    expr = expr.strip()
    while "--" in expr and not expr.startswith("--"):
        expr = expr.replace("--", "+")
    if expr.startswith("--"):
        expr = expr[2:]
    expr = re.sub(r"~\\(", "-(", expr)
    expr = re.sub(r"~\\(-", "(", expr)
    # safe subset: digits, xor, parens, + - ~
    allowed = set("0123456789abcdefxABCDEF+-^() ~")
    if not set(expr) <= allowed:
        raise ValueError(expr)
    return eval(expr, {"__builtins__": {}}, {})  # noqa: S307


def parse_module_constants() -> dict[str, int]:
    text = MODULE.read_text(encoding="utf-8", errors="ignore")
    consts: dict[str, int] = {}
    for m in re.finditer(
        r"m_d13888ddcdfb4bf0af71d570d817ec50\.(m_[0-9a-f]+)\s*=\s*([^;]+);",
        text,
    ):
        name, expr = m.group(1), m.group(2)
        try:
            consts[name] = eval_expr(expr)
        except Exception:
            pass
    return consts


def parse_offset_exprs() -> list[tuple[str, int]]:
    text = MYNA.read_text(encoding="utf-8", errors="ignore")
    consts = parse_module_constants()
    out: list[tuple[str, int]] = []
    for m in re.finditer(
        r"nmKikxo27T\((0x[0-9A-Fa-f]+|\d+)\s*\^\s*_003CModule_003E_\{[^}]+\}\.m_d13888ddcdfb4bf0af71d570d817ec50\.(m_[0-9a-f]+)\)",
        text,
    ):
        a = int(m.group(1), 0)
        key = m.group(2)
        if key in consts:
            out.append((m.group(0), a ^ consts[key]))
    return out


def decode_ktqk(raw: bytes) -> str:
    data = bytes((b ^ 0xBA) - 146 & 0xFF for b in raw)
    return data.decode("utf-8", errors="replace")


def try_read_string_table(blob: bytes, offset: int) -> str | None:
    if offset + 4 > len(blob):
        return None
    (n,) = struct.unpack_from("<i", blob, offset)
    if n <= 0 or n > 5000 or offset + 4 + n > len(blob):
        return None
    try:
        return blob[offset + 4 : offset + 4 + n].decode("utf-16-le")
    except UnicodeDecodeError:
        return None


def main() -> None:
    consts = parse_module_constants()
    print(f"module constants: {len(consts)}")
    offsets = parse_offset_exprs()
    print(f"resolved offsets: {len(offsets)}")
    blob = RESOURCE.read_bytes()
    print(f"resource size: {len(blob)}")

    groups = {"ynyiO7kmBG": [], "CTEiqW5reR": [], "QnaiPwMkkB": []}
    # offsets appear in 3 blocks in static ctor
    for i, (_, off) in enumerate(offsets):
        s = try_read_string_table(blob, off)
        label = ["ynyiO7kmBG", "CTEiqW5reR", "QnaiPwMkkB"][min(i // 12, 2)]
        if s:
            groups[label].append(s)
            print(f"[{label}] @{off:#x}: {s!r}")

    print("\n=== Summary (plaintext resource read failed — AES encrypted at rest) ===")
    print("Offset resolution succeeded; runtime decryption (DQkiMoUUgf) required for strings.")
    print("Ktqk3BZv2l1hQLabdRS7 algorithm: Base64 -> XOR 0xBA -> subtract 146")


if __name__ == "__main__":
    main()
