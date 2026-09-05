#!/usr/bin/env python3
"""Emulate DQkiMoUUgf control-flow to recover AES key/IV for string resource."""
from __future__ import annotations

import re
from pathlib import Path

UBEN = Path(r"C:\Users\beakd\Desktop\uplKit\SwiftyULP\inru20iKnAcBB4kRU2Y\uBENWXiQQcdQecsoo9k.cs")


def extract_function(name: str) -> str:
    text = UBEN.read_text(encoding="utf-8", errors="ignore")
    m = re.search(rf"private static void {name}\(object P_0, int P_1\)\s*\{{", text)
    if not m:
        raise SystemExit(f"{name} not found")
    start = m.end()
    depth = 1
    i = start
    while i < len(text) and depth:
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
        i += 1
    return text[start : i - 1]


def parse_cases(body: str) -> dict[int, list[str]]:
    cases: dict[int, list[str]] = {}
    current = None
    for line in body.splitlines():
        m = re.match(r"\s*case\s+(\d+)\s*:", line)
        if m:
            current = int(m.group(1))
            cases.setdefault(current, [])
            continue
        if current is not None:
            stripped = line.strip()
            if stripped.startswith("case ") or stripped == "default:":
                m2 = re.match(r"case\s+(\d+)\s*:", stripped)
                if m2:
                    current = int(m2.group(1))
                    cases.setdefault(current, [])
                    continue
                if stripped == "default:":
                    current = -1
                    cases.setdefault(-1, [])
                    continue
            if stripped.startswith("break;") or stripped.startswith("continue;"):
                cases[current].append(stripped)
                continue
            if stripped and not stripped.startswith("//"):
                cases[current].append(stripped)
    return cases


def eval_int(expr: str, env: dict[str, int]) -> int:
    expr = expr.strip().rstrip(";")
    expr = expr.replace("((Array)P_0)", "0").replace("(Stream)P_0", "0")
    # simplify casts and method calls to 0
    expr = re.sub(r"\([^)]*\)", "", expr)
    while "--" in expr and not expr.startswith("--"):
        expr = expr.replace("--", "+")
    if expr.startswith("--"):
        expr = expr[2:]
    if re.match(r"^num\d+$", expr):
        return env.get(expr, 0) & 0xFF
    if re.match(r"^\d+$", expr):
        return int(expr) & 0xFF
    allowed = set("0123456789+-*/^() ")
    if not set(expr) <= allowed:
        return 0
    try:
        return eval(expr, {"__builtins__": {}}, {}) & 0xFF
    except Exception:
        return 0


def emulate() -> tuple[bytes, bytes]:
    body = extract_function("DQkiMoUUgf")
    cases = parse_cases(body)

    env: dict[str, int] = {f"num{i}": 0 for i in range(1, 10)}
    env["num"] = 151
    arrays: dict[str, list[int]] = {
        "array": [0] * 32,
        "array2": [0] * 16,
        "array4": [0] * 16,
        "array5": [0] * 16,
    }
    steps = 0
    max_steps = 50000
    state = 151

    def pps() -> bool:
        return True

    def tk() -> bool:
        return True  # TkAOtDWxMANHUQfErxI() == null

    while steps < max_steps:
        steps += 1
        stmts = cases.get(state, [])
        next_state: int | None = None
        break_outer = False

        for st in stmts:
            if st == "break;":
                break_outer = True
                break
            if st == "continue;":
                break
            if st.startswith("if "):
                cond = st[3:].strip()
                if cond.endswith("continue;"):
                    cond = cond[:-9].strip()
                if cond.endswith("break;"):
                    cond = cond[:-6].strip()
                if cond.endswith("{"):
                    cond = cond[:-1].strip()
                take_then = False
                if "ppsUJ2WZ9g6vVnjTWDV()" in cond:
                    val = pps()
                    if cond.startswith("!"):
                        take_then = not val
                    else:
                        take_then = val
                elif "TkAOtDWxMANHUQfErxI() == null" in cond:
                    take_then = tk()
                elif "TkAOtDWxMANHUQfErxI() != null" in cond:
                    take_then = not tk()
                elif ".Length" in cond:
                    # array5 from md5 - assume non-empty after init
                    take_then = "!= 0" in cond or "> 0" in cond
                else:
                    take_then = False
                if take_then:
                    m = re.search(r"num2?\s*=\s*(\d+)", st)
                    if m:
                        next_state = int(m.group(1))
                continue
            m = re.match(r"num(\d+)\s*=\s*(.+)", st)
            if m:
                idx = m.group(1)
                env[f"num{idx}"] = eval_int(m.group(2), env)
                continue
            m = re.match(r"(array\d?)\[(\d+)\]\s*=\s*(\d+)\s*;", st)
            if m:
                arrays[m.group(1)][int(m.group(2))] = int(m.group(3)) & 0xFF
                continue
            m = re.match(r"(array\d?)\[(\d+)\]\s*=\s*\(byte\)num(\d+)\s*;", st)
            if m:
                arrays[m.group(1)][int(m.group(2))] = env.get(f"num{m.group(3)}", 0) & 0xFF
                continue
            m = re.match(r"(array\d?)\[(\d+)\]\s*=\s*\(byte\)num(\d+)\s*;", st)
            if m:
                arrays[m.group(1)][int(m.group(2))] = env.get(f"num{m.group(3)}", 0) & 0xFF
                continue
            m = re.match(r"array4\[(\d+)\]\s*=\s*array5\[(\d+)\]\s*;", st)
            if m:
                arrays["array4"][int(m.group(1))] = arrays["array5"][int(m.group(2))]
                continue
            m = re.match(r"(array\d?)\s*=\s*new byte\[(\d+)\]\s*;", st)
            if m:
                arrays[m.group(1)] = [0] * int(m.group(2))
                continue
            if "CryptoStream" in st or "B2xsehw7Lx" in st:
                return bytes(arrays["array"]), bytes(arrays["array2"])
            if st.startswith("return"):
                return bytes(arrays["array"]), bytes(arrays["array2"])

        if break_outer:
            state = env.get("num", state)
        elif next_state is not None:
            state = next_state
        else:
            # fallthrough: find num = X after break in outer while - not modeled
            break

    return bytes(arrays["array"]), bytes(arrays["array2"])


if __name__ == "__main__":
    key, iv = emulate()
    print("key", key.hex())
    print("iv ", iv.hex())
