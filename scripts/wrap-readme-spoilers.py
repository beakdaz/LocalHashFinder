import re
from pathlib import Path

path = Path(__file__).resolve().parents[1] / "README.md"
lines = path.read_text(encoding="utf-8").splitlines()

def clean_title(line: str) -> str:
    t = line.lstrip("#").strip()
    return re.sub(r"\*+", "", t).strip()

def is_wrap_header(line: str) -> bool:
    if line.startswith("### "):
        return False
    if line.startswith("## "):
        return True
    if line.startswith("# ") and "Содержание" not in line and not line.startswith("# HashFinder"):
        if "✳️" in line or "Возможности" in line:
            return True
    return False

out: list[str] = []
i = 0
while i < len(lines) and not (lines[i].startswith("#") and "Возможности" in lines[i]):
    out.append(lines[i])
    i += 1

while i < len(lines):
    line = lines[i]
    if line.strip() == "<details>" and i + 1 < len(lines) and "Спонсор" in lines[i + 1]:
        out.extend(lines[i:])
        break
    if line.strip() == "# English":
        out.append(line)
        i += 1
        continue
    if not is_wrap_header(line):
        out.append(line)
        i += 1
        continue

    title = clean_title(line)
    out.extend(["<details>", f"<summary>{title}</summary>", ""])
    i += 1
    body: list[str] = []
    while i < len(lines):
        if lines[i].strip() == "---":
            j = i + 1
            while j < len(lines) and lines[j].strip() == "":
                j += 1
            if j < len(lines) and (is_wrap_header(lines[j]) or lines[j].strip() == "# English"):
                break
            if j < len(lines) and lines[j].strip() == "<details>" and j + 1 < len(lines) and "Спонсор" in lines[j + 1]:
                break
        if lines[i].strip() == "<details>" and i + 1 < len(lines) and "Спонсор" in lines[i + 1]:
            break
        body.append(lines[i])
        i += 1

    while body and body[-1].strip() == "":
        body.pop()
    if body and body[-1].strip() == "---":
        body.pop()
    while body and body[-1].strip() == "":
        body.pop()

    out.extend(body)
    out.extend(["", "</details>", ""])
    if i < len(lines) and lines[i].strip() == "---":
        out.extend(["---", ""])
        i += 1

text = "\n".join(out) + "\n"
text = text.replace(
    "![Local Hash Finder — вкладка «Расшифровка»](docs/screenshot-main.png)| Описание | Путь |",
    "![Local Hash Finder — вкладка «Расшифровка»](docs/screenshot-main.png)\n\n| Описание | Путь |",
)
text = text.replace(
    "![Local Hash Finder — Hash Lookup tab](docs/screenshot-main.png)| Description | Path |",
    "![Local Hash Finder — Hash Lookup tab](docs/screenshot-main.png)\n\n| Description | Path |",
)
path.write_text(text, encoding="utf-8", newline="\n")
print("OK")
