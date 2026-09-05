#!/usr/bin/env python3
"""Static SwiftyULP string/resource extraction (no exe execution).

- Extract embedded .NET resources via dnfile
- Resolve nmKikxo27T offsets from decompiled C# + module constants
- Simulate sequential byte-array assignments for AES key/IV blocks
- Custom decrypt (nJSi6AXhs2) for string table blob
"""
from __future__ import annotations

import re
import struct
import sys
from pathlib import Path

try:
    import dnfile
except ImportError:
    dnfile = None  # type: ignore

ROOT = Path(r"C:\Users\beakd\Desktop\uplKit\SwiftyULP")
EXE = Path(r"G:\[CRACKED.ST] SwiftyULP\[CRACKED.ST] SwiftyULP\SwiftyULP.exe")
MODULE = ROOT / "-Module--023069ab-5e12-4f3e-b56a-119ef93dc26a-.cs"
MYNA = ROOT / "pX9yS0ilP15qj4Ak4KS" / "MyNA1Iid6j5htnA8XyE.cs"
UBEN = ROOT / "inru20iKnAcBB4kRU2Y" / "uBENWXiQQcdQecsoo9k.cs"
RESOURCE_NAME = "IIcl2uv1D8DVp94l7q.HaonIHdScDIdOeJG68"


def eval_expr(expr: str) -> int:
    expr = expr.strip().rstrip(";")
    # normalize C# unary --
    while "--" in expr and not expr.startswith("--"):
        expr = expr.replace("--", "+")
    if expr.startswith("--"):
        expr = expr[2:]
    expr = re.sub(r"~\\(", "-(", expr)
    allowed = set("0123456789abcdefxABCDEF+-^() ~")
    if not set(expr) <= allowed:
        raise ValueError(f"bad expr: {expr!r}")
    return eval(expr, {"__builtins__": {}}, {})  # noqa: S307


def parse_module_constants() -> dict[str, int]:
    text = MODULE.read_text(encoding="utf-8", errors="ignore")
    consts: dict[str, int] = {}
    pat = re.compile(
        r"m_d13888ddcdfb4bf0af71d570d817ec50\.(m_[0-9a-f]+)\s*=\s*([^;]+);"
    )
    for m in pat.finditer(text):
        name, expr = m.group(1), m.group(2)
        try:
            consts[name] = eval_expr(expr) & 0xFFFFFFFF
        except Exception:
            pass
    return consts


def parse_nm_offsets() -> list[tuple[str, int]]:
    text = MYNA.read_text(encoding="utf-8", errors="ignore")
    consts = parse_module_constants()
    out: list[tuple[str, int]] = []
    pat = re.compile(
        r"nmKikxo27T\((0x[0-9A-Fa-f]+|\d+)\s*\^\s*"
        r"_003CModule_003E_\{[^}]+\}\.m_d13888ddcdfb4bf0af71d570d817ec50\.(m_[0-9a-f]+)\)"
    )
    for m in pat.finditer(text):
        a = int(m.group(1), 0)
        key = m.group(2)
        if key in consts:
            out.append((m.group(0), (a ^ consts[key]) & 0xFFFFFFFF))
    return out


def simulate_byte_array(block: str, name: str, size: int) -> list[int]:
    """Last-write-wins simulation of array[i] = (byte)... assignments."""
    arr = [0] * size
    num_vars: dict[str, int] = {}
    for line in block.splitlines():
        line = line.strip()
        m = re.match(rf"{name}\[(\d+)\]\s*=\s*(\d+)\s*;", line)
        if m:
            arr[int(m.group(1))] = int(m.group(2)) & 0xFF
            continue
        m = re.match(rf"num\d+\s*=\s*([^;]+);", line)
        if m:
            try:
                num_vars["last"] = eval_expr(m.group(1)) & 0xFF
            except Exception:
                pass
            continue
        m = re.match(rf"{name}\[(\d+)\]\s*=\s*\(byte\)num\d+\s*;", line)
        if m and "last" in num_vars:
            arr[int(m.group(1))] = num_vars["last"]
    return arr


def extract_linear_key_iv() -> tuple[bytes, bytes] | None:
    text = UBEN.read_text(encoding="utf-8", errors="ignore")
    # linear AES setup block (static ctor path)
    start = text.find("byte[] array2 = new byte[32];")
    if start < 0:
        return None
    end = text.find("byte[] array5 = array4;", start)
    if end < 0:
        return None
    block = text[start:end]
    key = bytes(simulate_byte_array(block, "array2", 32))
    iv = bytes(simulate_byte_array(block, "array4", 16))
    return key, iv


def njsi_decrypt(key: bytes, _iv: bytes, data: bytes) -> bytes:
    """Port of nJSi6AXhs2 custom XOR stream decrypt."""
    num = len(data) % 4
    num2 = len(data) // 4
    out = bytearray(len(data))
    num3 = len(key) // 4
    num4 = 0
    num5 = 0
    num6 = 0
    if num > 0:
        num2 += 1
    for i in range(num2):
        num8 = i % num3
        num9 = i * 4
        num7 = num8 * 4
        num5 = key[num7] | (key[num7 + 1] << 8) | (key[num7 + 2] << 16) | (key[num7 + 3] << 24)
        num5 &= 0xFFFFFFFF
        if i == num2 - 1 and num > 0:
            num6 = 0
            num4 = (num4 + num5) & 0xFFFFFFFF
            for j in range(num):
                if j > 0:
                    num6 <<= 8
                num6 |= data[-(1 + j)]
            num6 &= 0xFFFFFFFF
        else:
            num4 = (num4 + num5) & 0xFFFFFFFF
            num6 = data[num9] | (data[num9 + 1] << 8) | (data[num9 + 2] << 16) | (data[num9 + 3] << 24)
            num6 &= 0xFFFFFFFF
        num12 = num4
        num4 = 0
        num13, num14, num15, num16 = 400425286, 2144675933, 646645818, 1970456059
        num17 = num12
        num18 = num14 & 0xFF00FF
        num19 = num14 & 0xFF00FF00
        num18 = ((num18 >> 8) | (num19 << 8)) ^ num15
        num14 = ((num14 >> 8) | (num14 << 24)) & 0xFFFFFFFF
        if num16 == 0:
            num16 = 0xFFFFFFFF
        num20 = (num13 // num16 + num16) & 0xFFFFFFFF
        num16 = ((num13 - num15) ^ num20) + num13
        num16 = (598554144 * (num16 & 7) + (num16 >> 3)) & 0xFFFFFFFF
        num13 = (356717981 * (num13 & 7) - (num13 >> 3)) & 0xFFFFFFFF
        num14 = (3624 * num14 - num15) & 0xFFFFFFFF
        num15 = (num15 - num14) & 0xFFFFFFFF
        if num17 == 0:
            num17 = 0xFFFFFFFF
        num20 = (num14 // num17 + num17) & 0xFFFFFFFF
        num17 = (num14 + num14 + num20 + num14) & 0xFFFFFFFF
        num17 ^= num17 >> 5
        num17 = (num17 + num13) & 0xFFFFFFFF
        num17 ^= (num17 << 3) & 0xFFFFFFFF
        num17 = (num17 + num15) & 0xFFFFFFFF
        num17 ^= num17 >> 21
        num17 = (num17 + num17) & 0xFFFFFFFF
        num17 = (((num16 << 8) + num15) ^ num13 - num17) & 0xFFFFFFFF
        num4 = (num12 + num17) & 0xFFFFFFFF
        if i == num2 - 1 and num > 0:
            num21 = (num4 ^ num6) & 0xFFFFFFFF
            num10 = 255
            num11 = 0
            for k in range(num):
                if k > 0:
                    num10 = (num10 << 8) & 0xFFFFFFFF
                    num11 += 8
                out[num9 + k] = (num21 & num10) >> num11
        else:
            num22 = (num4 ^ num6) & 0xFFFFFFFF
            out[num9] = num22 & 0xFF
            out[num9 + 1] = (num22 >> 8) & 0xFF
            out[num9 + 2] = (num22 >> 16) & 0xFF
            out[num9 + 3] = (num22 >> 24) & 0xFF
    return bytes(out)


def aes_cbc_decrypt(key: bytes, iv: bytes, data: bytes) -> bytes | None:
    try:
        from Crypto.Cipher import AES  # type: ignore
    except ImportError:
        try:
            from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
            from cryptography.hazmat.backends import default_backend

            cipher = Cipher(algorithms.AES(key), modes.CBC(iv), backend=default_backend())
            dec = cipher.decryptor()
            plain = dec.update(data) + dec.finalize()
            pad = plain[-1]
            if 1 <= pad <= 16:
                plain = plain[:-pad]
            return plain
        except ImportError:
            return None
    plain = AES.new(key, AES.MODE_CBC, iv).decrypt(data)
    pad = plain[-1]
    if 1 <= pad <= 16:
        plain = plain[:-pad]
    return plain


def read_string_at(blob: bytes, offset: int) -> str | None:
    if offset + 4 > len(blob):
        return None
    (n,) = struct.unpack_from("<i", blob, offset)
    if n <= 0 or n > 8000 or offset + 4 + n > len(blob):
        return None
    try:
        return blob[offset + 4 : offset + 4 + n].decode("utf-16-le")
    except UnicodeDecodeError:
        return None


def load_resource() -> bytes:
    if dnfile is None:
        p = ROOT / RESOURCE_NAME.replace(".", ".")  # fallback file in uplKit
        if p.exists():
            return p.read_bytes()
        raise RuntimeError("dnfile not installed and resource file missing")
    pe = dnfile.dnPE(str(EXE))
    for r in pe.net.resources:
        if r.name == RESOURCE_NAME:
            return r.data if hasattr(r, "data") else r.raw
    raise RuntimeError(f"resource {RESOURCE_NAME} not found")


def decode_ktqk_blob(data: bytes) -> list[str]:
    import base64

    out: list[str] = []
    for m in re.finditer(rb"[A-Za-z0-9+/=]{6,200}", data):
        try:
            raw = base64.b64decode(m.group(), validate=True)
            dec = bytes((b ^ 0xBA) - 146 & 0xFF for b in raw)
            s = dec.decode("utf-8")
            if s.isprintable() and 2 <= len(s) <= 200:
                out.append(s)
        except Exception:
            pass
    return sorted(set(out))


def main() -> None:
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    consts = parse_module_constants()
    print(f"module constants: {len(consts)}")
    offsets = parse_nm_offsets()
    print(f"resolved nmKikxo27T offsets: {len(offsets)}")

    enc = load_resource()
    print(f"encrypted resource size: {len(enc)}")

    key_iv = extract_linear_key_iv()
    decrypted: bytes | None = None
    if key_iv:
        key, iv = key_iv
        print(f"linear AES key: {key.hex()}")
        print(f"linear AES iv:  {iv.hex()}")
        decrypted = aes_cbc_decrypt(key, iv, enc)
        if decrypted:
            print(f"AES decrypted size: {len(decrypted)}")
        else:
            print("AES decrypt skipped (no crypto lib)")

    if not decrypted:
        # try custom decrypt with key from DQkiMoUUgf array (32 bytes) — use linear key as guess
        if key_iv:
            decrypted = njsi_decrypt(key_iv[0], key_iv[1], enc)
            print(f"custom decrypt size: {len(decrypted)}")

    groups = {"ynyiO7kmBG": [], "CTEiqW5reR": [], "QnaiPwMkkB": [], "other": []}
    blob = decrypted or enc

    for i, (_, off) in enumerate(offsets):
        if off >= len(blob):
            continue
        s = read_string_at(blob, off)
        label = ["ynyiO7kmBG", "CTEiqW5reR", "QnaiPwMkkB"][min(i // 12, 2)]
        if s:
            groups[label].append(s)
            print(f"[{label}] @{off:#x}: {s!r}")

    # scan decrypted blob for utf16 string table entries
    if decrypted:
        found = 0
        for off in range(0, len(decrypted) - 8, 4):
            s = read_string_at(decrypted, off)
            if s and len(s) >= 3 and s.isprintable():
                groups["other"].append(s)
                found += 1
        print(f"scanned utf16 strings in blob: {found}")

    ktqk = decode_ktqk_blob(enc)
    print(f"ktqk strings in resource: {len(ktqk)}")
    for s in ktqk[:30]:
        print(f"  ktqk: {s!r}")

    out = Path(__file__).with_name("swifty_extracted.json")
    import json

    out.write_text(
        json.dumps({k: sorted(set(v)) for k, v in groups.items()}, ensure_ascii=False, indent=2),
        encoding="utf-8",
    )
    print(f"wrote {out}")


if __name__ == "__main__":
    main()
