# BAT-скрипты LocalHashFinder

Все `.bat`-файлы лежат **в корне репозитория** рядом с `README.md`. Запускайте их двойным щелчком или из `cmd` / PowerShell из любой папки — каждый скрипт сам переходит в каталог проекта (`cd /d "%~dp0"`).

> **English version:** [Batch scripts (EN)](#english)

---

## Быстрый обзор

| Скрипт | Назначение |
|--------|------------|
| [BUILD.bat](#buildbat) | Сборка `LocalHashFinder.exe` и `TextMerger.exe` |
| [START-LOCAL-HASH.bat](#start-local-hashbat) | Запуск GUI |
| [START-WEB.bat](#start-webbat) | Legacy web UI (Node.js) |
| [MERGE-CLEAN.bat](#merge-cleanbat) | Склейка wordlist → plain passwords |
| [WORDLIST-HASH.bat](#wordlist-hashbat) | Один файл → `hash:pass` |
| [WORDLIST-HASH-FOLDER.bat](#wordlist-hash-folderbat) | Папка `*.txt` → `hash:pass` |
| [IMPORT-DB.bat](#import-dbbat) | Первичный импорт в LMDB |
| [APPEND-DB.bat](#append-dbbat) | Дополнение LMDB |
| [MERGE.bat](#mergebat) | Склейка mail:hash + hash:pass |
| [EXTRACT-SQL.bat](#extract-sqlbat) | Извлечение email:hash из SQL |

---

## Типовые цепочки (workflow)

### A. Подготовка LMDB из wordlist-ов

> Готовый `hashdb.lmdb` в **GitHub Releases пока нет** — только локальная сборка через цепочку ниже. Релиз с базой будет позже.

**Где взять plaintext wordlist-ы:** [HashMob](https://hashmob.net/), [Weakpass](https://weakpass.com/wordlist), [g0tmi1k wordlists](https://download.g0tmi1k.com/wordlists), [SecLists Passwords](https://github.com/danielmiessler/SecLists/tree/master/Passwords).

**Крупные дампы (Mail.ru):** [260M](https://cloud.mail.ru/public/HsHb/JamxkSKRF) · [243M](https://cloud.mail.ru/public/hEUf/XJkHjc6Ny) · [358M](https://cloud.mail.ru/public/EnHr/Qx1hYDDrC) · [112M](https://cloud.mail.ru/public/K6Ho/HCcQFxHNH) · [950M](https://cloud.mail.ru/public/eo1N/SYYp5gELP) · [999M+](https://cloud.mail.ru/public/Coni/QgPayRbhv)

```
BUILD.bat
    ↓
MERGE-CLEAN.bat "D:\raw_wordlists" "D:\merged_clean.txt" recursive
    ↓
WORDLIST-HASH-FOLDER.bat "D:\merged_folder" md5 32
    ↓
IMPORT-DB.bat "D:\merged_folder\123456_clean_md5.txt" 4
    ↓
START-LOCAL-HASH.bat
```

### B. Расшифровка combo + склейка

```
BUILD.bat → START-LOCAL-HASH.bat   (Lookup в GUI)
EXTRACT-SQL.bat "dump.sql"         (email:hash из SQL)
MERGE.bat "mail_hash.txt" "dehash_good.txt"
```

### C. Дополнение базы

```
APPEND-DB.bat "D:\new_hashes.txt" 280
```

**Важно:** перед `IMPORT-DB.bat` / `APPEND-DB.bat` закройте GUI — LMDB не поддерживает одновременную запись из двух процессов.

---

## Общие требования

- **Windows 10 / 11**
- **Кодировка:** все скрипты вызывают `chcp 65001` (UTF-8)
- **Сборка:** почти все скрипты требуют предварительного `BUILD.bat`
- **Пути к exe:** `engine\target\release\LocalHashFinder.exe`, `engine\target\release\TextMerger.exe`
- **LMDB по умолчанию:** `engine\target\release\data\hashdb.lmdb`

---

## BUILD.bat

**Назначение:** собирает release-бинарники Rust-проекта в `engine/`.

**Использование:**

```bat
BUILD.bat
```

**Параметры:** нет.

**Требования:**
- Установлен Rust / `cargo` ([rustup.rs](https://rustup.rs))
- При отсутствии: подсказка `winget install Rustlang.Rustup`

**Что создаёт:**
- `engine\target\release\LocalHashFinder.exe` — GUI + CLI
- `engine\target\release\TextMerger.exe` — склейка wordlist
- `engine\target\release\data\` — каталог для LMDB (при первом запуске)

**Пример вывода:**

```
Building LocalHashFinder + TextMerger release...
OK: C:\...\engine\target\release\LocalHashFinder.exe
Built: 05.09.2026 10:30  size=... bytes
Run: START-LOCAL-HASH.bat
```

**Связанные скрипты:** все остальные (запускайте первым).

---

## START-LOCAL-HASH.bat

**Назначение:** запускает графический интерфейс LocalHashFinder.

**Использование:**

```bat
START-LOCAL-HASH.bat
```

**Параметры:** нет.

**Требования:** `BUILD.bat` выполнен, `LocalHashFinder.exe` существует.

**Что создаёт при первом запуске:**
- `engine\target\release\LocalHashFinder.cfg` — конфиг с путём к LMDB
- `engine\target\release\data\` — если каталога ещё нет

**Пример вывода:**

```
LocalHashFinder — GUI (Lookup, Merge, SQL, Regex, Combo, ULP)
Launching:
  C:\...\engine\target\release\LocalHashFinder.exe
```

**Связанные скрипты:** `BUILD.bat` → этот скрипт; для import/append закройте GUI перед `IMPORT-DB.bat` / `APPEND-DB.bat`.

---

## START-WEB.bat

**Назначение:** legacy web UI на Node.js (если в проекте есть `server.js`).

**Использование:**

```bat
START-WEB.bat
```

**Параметры:** нет.

**Требования:**
- Node.js в PATH
- Файл `server.js` в корне проекта

**Что делает:** запускает `node server.js`, открывает `http://127.0.0.1:8787`.

**Связанные скрипты:** для LMDB используйте `START-LOCAL-HASH.bat`.

---

## MERGE-CLEAN.bat

**Назначение:** объединяет `.txt` wordlist-ы в один файл **только plain-паролей** (без combo, hash, мусора), с dedupe.

**Использование:**

```bat
MERGE-CLEAN.bat "D:\wordlists" "D:\merged_clean.txt"
MERGE-CLEAN.bat "D:\wordlists" "merged.txt" recursive
```

**Параметры:**

| # | Имя | Обязательный | Описание |
|---|-----|--------------|----------|
| 1 | `input_folder` | да | Папка с `.txt` |
| 2 | `output.txt` | да | Выходной файл |
| 3 | `recursive` | нет | Обход подпапок (`recursive` или `--recursive`) |

**Требования:** `BUILD.bat` (нужен `TextMerger.exe`).

**Что создаёт:** один `.txt` — по одному паролю на строку, без дубликатов.

**Удаляется:** `:NULL`, строки < 3 символов, hash/combo-строки, комментарии.

**Связанные скрипты:** → `WORDLIST-HASH.bat` / `WORDLIST-HASH-FOLDER.bat` → `IMPORT-DB.bat`.

---

## WORDLIST-HASH.bat

**Назначение:** хеширует plaintext wordlist в формат `hash:password` для импорта в LMDB.

**Использование:**

```bat
WORDLIST-HASH.bat "passwords.txt"
WORDLIST-HASH.bat "passwords.txt" sha1
WORDLIST-HASH.bat "passwords.txt" both 32
```

**Параметры:**

| # | Имя | По умолчанию | Описание |
|---|-----|--------------|----------|
| 1 | `wordlist.txt` | — | Входной файл (plain passwords) |
| 2 | `algo` | `md5` | `md5`, `sha1` или `both` |
| 3 | `threads` | `0` | Число потоков (`0` = auto) |

**Требования:** `BUILD.bat`.

**Что создаёт:** `{random}_{stem}_md5.txt` рядом с исходником (например `847291_passwords_md5.txt`).

**Связанные скрипты:** `MERGE-CLEAN.bat` → этот скрипт → `IMPORT-DB.bat`.

---

## WORDLIST-HASH-FOLDER.bat

**Назначение:** то же, что `WORDLIST-HASH.bat`, но для всех `*.txt` в папке (пакетно).

**Использование:**

```bat
WORDLIST-HASH-FOLDER.bat "D:\wordlists"
WORDLIST-HASH-FOLDER.bat "D:\wordlists" sha1
WORDLIST-HASH-FOLDER.bat "D:\wordlists" both 32
WORDLIST-HASH-FOLDER.bat
```

**Параметры:**

| # | Имя | По умолчанию | Описание |
|---|-----|--------------|----------|
| 1 | `folder` | `wordlists\` | Папка с `.txt` |
| 2 | `algo` | `md5` | `md5`, `sha1`, `both` |
| 3 | `threads` | `0` | Потоки |

**Пропускает:** `*_md5.txt`, `*_sha1.txt`, файлы с `hash:pass` в начале, уже обработанные hash-выходы.

**Требования:** `BUILD.bat`; папка должна существовать.

**Что создаёт:** по одному `{random}_{stem}_md5.txt` на каждый обработанный файл.

**Связанные скрипты:** `MERGE-CLEAN.bat` → этот скрипт → `IMPORT-DB.bat`.

---

## IMPORT-DB.bat

**Назначение:** первичный импорт файла `hash:pass` в LMDB.

**Использование:**

```bat
IMPORT-DB.bat "D:\hashes\hash_pass.txt"
IMPORT-DB.bat "D:\hashes\hash_pass.txt" 280
```

**Параметры:**

| # | Имя | По умолчанию | Описание |
|---|-----|--------------|----------|
| 1 | `hash_pass.txt` | — | Файл формата `32hex:pass` или `40hex:pass` |
| 2 | `map_gb` | `280` | Размер LMDB map в GB |

**Требования:**
- `BUILD.bat`
- Закрытый GUI
- Достаточно места на диске

**Целевой путь:** `engine\target\release\data\hashdb.lmdb`

**Пример вывода:**

```
Importing D:\hashes\hash_pass.txt into ...\data\hashdb.lmdb (map ~280 GB)
This can take many hours for large files. Do not close the window.
```

**Связанные скрипты:** `WORDLIST-HASH*.bat` → этот скрипт → `START-LOCAL-HASH.bat`; для дополнения — `APPEND-DB.bat`.

---

## APPEND-DB.bat

**Назначение:** добавляет новые `hash:pass` в существующую LMDB; дубликаты пропускаются (старый пароль сохраняется).

**Использование:**

```bat
APPEND-DB.bat "D:\new_hashes.txt"
APPEND-DB.bat "D:\new_hashes.txt" 280
```

**Параметры:** те же, что у `IMPORT-DB.bat`.

**Требования:**
- `BUILD.bat`
- Существующая LMDB (`IMPORT-DB.bat` уже выполнен)
- Закрытый GUI

**Связанные скрипты:** после `IMPORT-DB.bat`; альтернатива — Append в GUI.

---

## MERGE.bat

**Назначение:** объединяет `email:hash` и `hash:plainpass` → `email:plainpass`.

**Использование:**

```bat
MERGE.bat "mail_hash.txt" "dehash_good.txt"
```

**Параметры:**

| # | Имя | Описание |
|---|-----|----------|
| 1 | `mail_hash.txt` | Строки `user@gmail.com:md5hash` |
| 2 | `dehash_good.txt` | Строки `md5hash:plainpass` |

**Требования:** `BUILD.bat`.

**Что создаёт:** `{stem}_plain.txt`, `{stem}_plain_nohash.txt` (рядом с mail-файлом).

**Связанные скрипты:** после Lookup в GUI (`*_good.txt` как dehash) или после `IMPORT-DB.bat` + Lookup.

---

## EXTRACT-SQL.bat

**Назначение:** извлекает `email:md5` / `email:sha1` из SQL-дампа через regex.

**Использование:**

```bat
EXTRACT-SQL.bat "dump.sql"
EXTRACT-SQL.bat "dump.sql" "output.txt"
```

**Параметры:**

| # | Имя | Обязательный | Описание |
|---|-----|--------------|----------|
| 1 | `dump.sql` | да | SQL-файл |
| 2 | `output.txt` | нет | Явный путь вывода |

**Требования:** `BUILD.bat`.

**Что создаёт:** `{stem}_emails.txt` (если output не указан) или указанный файл.

**Связанные скрипты:** → Lookup в GUI или → `MERGE.bat`.

---

## Автокоммит README

После правок в `README.md` можно автоматически коммитить и пушить (только этот файл; `SCRIPTS.md` и остальное не трогает).

| Скрипт | Назначение |
|--------|------------|
| `INSTALL-README-AUTO-COMMIT-TASK.bat` | Планировщик Windows: проверка **каждые 15 мин** + Cursor hook |
| `WATCH-README-AUTO-COMMIT.bat` | Watcher: коммит **~8 сек после сохранения** (окно держать открытым) |
| `UNINSTALL-README-AUTO-COMMIT-TASK.bat` | Удалить задачу планировщика |
| `UPDATE-README-HOOK.bat` | Обновить Cursor hook (только README.md) |
| `scripts\auto-commit-readme.ps1` | Ядро: `git add` только README.md → commit → push |

**Установка (один раз):**

```bat
INSTALL-README-AUTO-COMMIT-TASK.bat
```

Нужен `git push` без запроса пароля (Git Credential Manager / `gh auth login`). Сообщение коммита: `docs: auto-commit README (YYYY-MM-DD HH:mm)`.

Cursor hook (`.cursor/hooks.json`) срабатывает при правках README через Agent/Tab. Ручные правки покрывает **планировщик** или **WATCH-README-AUTO-COMMIT.bat**.

---

# English

All `.bat` files live in the **repository root**. Each script sets UTF-8 (`chcp 65001`) and `cd`s to its own directory.

---

## Quick reference

| Script | Purpose |
|--------|---------|
| `BUILD.bat` | Build release binaries |
| `START-LOCAL-HASH.bat` | Launch GUI |
| `START-WEB.bat` | Legacy Node web UI |
| `MERGE-CLEAN.bat` | Merge wordlists → plain passwords |
| `WORDLIST-HASH.bat` | Single file → `hash:pass` |
| `WORDLIST-HASH-FOLDER.bat` | Folder batch → `hash:pass` |
| `IMPORT-DB.bat` | Initial LMDB import |
| `APPEND-DB.bat` | Append to LMDB |
| `MERGE.bat` | mail:hash + hash:pass merge |
| `EXTRACT-SQL.bat` | SQL → email:hash |
| `INSTALL-README-AUTO-COMMIT-TASK.bat` | Schedule auto-commit for README.md only |
| `WATCH-README-AUTO-COMMIT.bat` | Watch README.md and commit after save |

---

## Typical workflows

**Wordlist → LMDB:**

```
BUILD.bat → MERGE-CLEAN.bat → WORDLIST-HASH-FOLDER.bat → IMPORT-DB.bat → START-LOCAL-HASH.bat
```

**Close the GUI** before `IMPORT-DB.bat` / `APPEND-DB.bat`.

---

## Per-script summary (EN)

### BUILD.bat
Builds `LocalHashFinder.exe` and `TextMerger.exe`. No args. Requires Rust/cargo.

### START-LOCAL-HASH.bat
Launches GUI. Creates `LocalHashFinder.cfg` and `data/` on first run.

### START-WEB.bat
Legacy `node server.js` web UI. Requires Node.js and `server.js` in project root.

### MERGE-CLEAN.bat
`TextMerger merge` — merges `.txt` wordlists, strips garbage/combos/hashes, dedupes plain passwords.

Args: `input_folder` `output.txt` [`recursive`]

### WORDLIST-HASH.bat
Hashes one plaintext wordlist to `hash:pass`. Args: `file` [`md5|sha1|both`] [`threads`]. Output: `{random}_{stem}_md5.txt`.

### WORDLIST-HASH-FOLDER.bat
Batch version for a folder of `.txt` files. Default folder: `wordlists\`. Skips existing hash outputs.

### IMPORT-DB.bat
First-time LMDB import. Args: `hash_pass.txt` [`map_gb`]. Target: `engine\target\release\data\hashdb.lmdb`.

### APPEND-DB.bat
Append new entries; duplicate keys skipped. Requires existing LMDB.

### MERGE.bat
Combine `email:hash` + `hash:pass` → `email:pass`. Args: `mail_file` `dehash_file`.

### EXTRACT-SQL.bat
Extract `email:md5` / `email:sha1` from SQL dumps. Args: `dump.sql` [`output.txt`].

---

## Prerequisites (all scripts)

- Windows 10/11
- Run `BUILD.bat` first (except `BUILD.bat` itself and `START-WEB.bat`)
- Binaries: `engine\target\release\LocalHashFinder.exe`, `TextMerger.exe`
- Default LMDB: `engine\target\release\data\hashdb.lmdb`

For full CLI reference: `engine\target\release\LocalHashFinder.exe --help`
