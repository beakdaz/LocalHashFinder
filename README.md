# Local Hash Finder

**Офлайн desktop-набор для расшифровки хешей, обработки combo-листов и ULP-инструментов.**

Local Hash Finder — портативное Windows-приложение на Rust. Все операции выполняются локально: без облака, без лицензий и без отправки данных на сервер. Вы сами выбираете LMDB-базу хешей и свои входные файлы.

---

## Содержание

- [Возможности](#возможности)
- [Интерфейс](#интерфейс)
- [Требования](#требования)
- [Быстрый старт](#быстрый-старт)
- [Источники wordlist (plaintext)](#источники-wordlist-plaintext)
- [База данных LMDB](#база-данных-lmdb)
- [Конфигурация](#конфигурация)
- [CLI и bat-скрипты](#cli-и-bat-скрипты)
- [SCRIPTS.md — полный справочник bat-скриптов](SCRIPTS.md)
- [Структура проекта](#структура-проекта)
- [Технологии](#технологии)
- [Скриншоты](#скриншоты)
- [Отказ от ответственности](#отказ-от-ответственности)
- [Лицензия](#лицензия)
- [English](#english)

---

## Возможности

Приложение состоит из **7 модулей** (вкладок в боковой панели):

### 1. Расшифровка (Hash Lookup)

Пакетный поиск паролей по MD5/SHA1 в локальной LMDB-базе.

- Открытие и выбор пути к `hashdb.lmdb`
- Пакетная обработка файлов (`email:hash`, `hash`, `email:hash:extra`)
- Настраиваемое число потоков (до 512)
- Выход: `{name}_good.txt`, `{name}_nohash.txt`, `{name}_bad.txt`, `{name}_trash.txt`
- **Import** — первичная загрузка `hash:pass` в LMDB (CLI / `IMPORT-DB.bat`)
- **Append** — дополнение существующей базы без перезаписи дубликатов (GUI / `APPEND-DB.bat`)
- Пауза, стоп, просмотр результатов в проводнике

### 2. Склейка (Merge)

Объединение `mail:hashedpass` и `hash:plainpass` → `mail:plainpass`.

- Вход: список `email:hash` + файл расшифровки (`_good.txt` или `hash:pass`)
- Выход: `{stem}_plain.txt`, `{stem}_plain_nohash.txt`, `{stem}_trash.txt`
- CLI: `merge --mail … --dehash …` / `MERGE.bat`

### 3. SQL Extract

Извлечение `email:md5` / `email:sha1` из SQL-дампов через regex.

- Один файл или папка (`.sql`, `.txt`, `.dump`)
- Параллельная обработка (до 5 потоков)
- UTF-8 и Windows-1252, поддержка BOM
- Выход: `{stem}_emails.txt`, `{stem}_trash.txt`
- CLI: `extract-sql` / `EXTRACT-SQL.bat`

### 4. SQL Колонки (SQL Columns)

Извлечение `login:password` по именам колонок в `CREATE TABLE` / `INSERT`.

- Распознавание колонок login/email/user и password/pass/pwd
- Несколько кортежей в одной строке INSERT
- Пакетная обработка папки с дампами
- Выход: `{stem}_loginpass.txt`
- CLI: `extract-sql-columns`

### 5. Custom Regex

Произвольное извлечение по regex с шаблоном вывода (`$1`, `$2`, `${name}`).

- Пресеты: `email:md5`, `email:sha1`, `hash:pass`, email only
- Флаги: case-insensitive, multiline, dotall, dedupe
- CLI: `extract-regex --pattern … --template …`

### 6. ComboKit

11 инструментов для combo-листов (без LMDB):

| Инструмент | Описание |
|------------|----------|
| **Compare** | Сравнение двух списков → `only_a`, `only_b`, `both` |
| **Combo filter** | Валидация `email:pass`, отсев мусора |
| **Email filter** | Только строки с email |
| **Split name/password** | Разделение на `names.txt` + `passwords.txt` |
| **MX check** | Проверка MX-записей доменов |
| **Scraper** | Извлечение combo из `.txt` / `.sql` / `.json` |
| **Analyze** | Группировка по доменам (`by_domain/`) |
| **Dedupe** | Уникальные строки |
| **Line filter** | Фильтр по подстроке или regex |
| **Merge** | Склейка строк в один файл |
| **Split** | Разбиение файла по N строк |

### 7. ULP / SwiftyULP

18 сервисов для обработки `url:login:pass` (архивы `.zip` / `.7z` / `.rar` и папки):

| Сервис | Описание |
|--------|----------|
| **Sort** | Сортировка по типам (Mails, Mail Pass, Phone Pass, User Pass, ULP) |
| **Sort Country** | Сортировка по TLD → `by_tld/`, `by_domain/` |
| **Sort Keyword** | Разложение по keyword в отдельные файлы |
| **Search** | Поиск по keyword → один файл |
| **Extract url:login:pass** | Извлечение полного ULP |
| **Extract login:pass** | Извлечение `login:pass` |
| **Extract user:pass** | Извлечение `user:pass` |
| **Clean — dedupe** | Удаление дубликатов строк |
| **Clean — empty lines** | Удаление пустых строк |
| **Clean — junk** | Удаление junk-символов |
| **Clean — blacklist** | Фильтр blacklist-доменов |
| **Clean — empty chars** | Удаление пустых символов |
| **Clean — weak** | Отсев слабых паролей |
| **Clean — protocols** | Удаление протоколов (`http://`, …) |
| **Clean — capture** | Обработка capture-строк |
| **Misc — merge** | Склейка файлов |
| **Misc — split** | Split по числу строк |
| **Misc — filter** | Фильтр по keyword |

---

## Интерфейс

- **Crypto dashboard** — тёмная тема в стиле RecehTok (фиолетовый акцент, stat-tiles, нижняя панель управления)
- **RU / EN** — переключение языка в шапке, строки в `engine/src/i18n.rs`
- **Frameless window** — окно без системной рамки, собственные кнопки свернуть / закрыть, скруглённые углы (DWM на Windows)
- Вкладки: Расшифровка · Склейка · SQL Extract · SQL Колонки · Custom Regex · ComboKit · ULP
- Лог, результаты, инструкции на каждой вкладке

---

## Требования

- **Windows 10 / 11** (основная платформа; сборка заточена под Windows)
- Для **сборки из исходников**: [Rust](https://rustup.rs) (stable), `cargo`
- Достаточно места на диске под LMDB (для больших баз — сотни GB map size)

---

## Быстрый старт

```bat
BUILD.bat
START-LOCAL-HASH.bat
```

1. `BUILD.bat` — `cargo build --release` в `engine/`, копирует `LocalHashFinder.exe` в `engine/target/release/`
2. `START-LOCAL-HASH.bat` — создаёт `data/` и `LocalHashFinder.cfg` при первом запуске, открывает GUI

Исполняемый файл: `engine/target/release/LocalHashFinder.exe`

---

## Источники wordlist (plaintext)

Публичные ресурсы для **plain-password** списков (используйте только в рамках закона и для своих систем):

**Каталоги и сайты**

| Ресурс | Описание |
|--------|----------|
| [HashMob](https://hashmob.net/) | База и сообщество по hash lookup; wordlist-ы и материалы для исследований |
| [Weakpass](https://weakpass.com/wordlist) | Большие публичные wordlist-ы (часть бесплатна) |
| [g0tmi1k — wordlists](https://download.g0tmi1k.com/wordlists) | Каталог wordlist-ов для pentest и аудита |
| [SecLists — Passwords](https://github.com/danielmiessler/SecLists/tree/master/Passwords) | Классика: `rockyou.txt`, `10k-most-common.txt`, `best1050.txt` |

**Крупные plaintext-дампы (Mail.ru Cloud, .txt)**

| Ссылка | Размер |
|--------|--------|
| [260M passwords](https://cloud.mail.ru/public/HsHb/JamxkSKRF) | ~260 млн |
| [243M passwords](https://cloud.mail.ru/public/hEUf/XJkHjc6Ny) | ~243 млн |
| [358M passwords](https://cloud.mail.ru/public/EnHr/Qx1hYDDrC) | ~358 млн |
| [112M passwords](https://cloud.mail.ru/public/K6Ho/HCcQFxHNH) | ~112 млн |
| [950M passwords](https://cloud.mail.ru/public/eo1N/SYYp5gELP) | ~950 млн |
| [999M+ passwords](https://cloud.mail.ru/public/Coni/QgPayRbhv) | 999+ млн |

Для файлов на сотни миллионов строк закладывайте место на диске; перед хешированием прогоните через `MERGE-CLEAN.bat`.

**Цепочка в LocalHashFinder:**

```
скачать .txt → MERGE-CLEAN.bat (только пароли) → WORDLIST-HASH-FOLDER.bat → IMPORT-DB.bat
```

---

## База данных LMDB

Путь по умолчанию: `engine/target/release/data/hashdb.lmdb`

| Действие | Способ |
|----------|--------|
| Первичный импорт | `IMPORT-DB.bat "D:\path\to\hash_pass.txt" [map_gb]` |
| Дополнение базы | `APPEND-DB.bat "D:\new_hashes.txt" [map_gb]` или вкладка «Расшифровка» → Append |
| Формат строки | `hash:password` (32 или 40 hex-символов) |

Подготовка wordlist → `hash:pass` (по умолчанию MD5): `WORDLIST-HASH.bat "passwords.txt"` (один файл) или `WORDLIST-HASH-FOLDER.bat "D:\wordlists"` (все `*.txt` в папке, по умолчанию `wordlists\`). Выходной файл: `{random}_{имя}_md5.txt` (например `847291_passwords_md5.txt`). Опционально: второй аргумент `sha1` или `both`.

`map_gb` — размер map LMDB в GB (по умолчанию 280; для ~200 GB исходника). Импорт больших файлов может занять много часов — не закрывайте окно.

Перед import/append **закройте** GUI (LMDB не поддерживает одновременную запись из двух процессов).

---

## Конфигурация

Файл рядом с exe: `LocalHashFinder.cfg`

```ini
# LocalHashFinder — settings
lmdb_path=C:\...\engine\target\release\data\hashdb.lmdb
ui_lang=ru
ui_zoom=1.10
```

- `lmdb_path` — каталог LMDB
- `ui_lang` — `ru` или `en`
- `ui_zoom` — фиксированный масштаб UI (1.10)

Файл создаётся автоматически при первом запуске через `START-LOCAL-HASH.bat`.

---

## CLI и bat-скрипты

> **Полная документация:** [SCRIPTS.md](SCRIPTS.md) — назначение, параметры, примеры и цепочки workflow для каждого `.bat`.

| Скрипт | Команда |
|--------|---------|
| `BUILD.bat` | Сборка release |
| `START-LOCAL-HASH.bat` | Запуск GUI |
| `START-WEB.bat` | Legacy web UI (Node.js, `server.js`) |
| `WORDLIST-HASH.bat` | plaintext wordlist → `hash:pass` MD5 (один файл, выход `{random}_{stem}_md5.txt`) |
| `WORDLIST-HASH-FOLDER.bat` | то же для всех `*.txt` в папке (MD5 по умолчанию; пропуск hash-выходов) |
| `IMPORT-DB.bat` | `import` в LMDB |
| `APPEND-DB.bat` | `append` в LMDB |
| `MERGE.bat` | `merge --mail … --dehash …` |
| `EXTRACT-SQL.bat` | `extract-sql` |
| `MERGE-CLEAN.bat` | `TextMerger merge` — склейка wordlist `.txt`, чистка мусора, dedupe |

### TextMerger — склейка словарей паролей

Отдельная утилита `TextMerger.exe` для объединения `.txt` wordlist-ов в один файл: **только plain-пароли**, по одному на строку, с dedupe.

**Удаляется (мусор):**
- строки с `:NULL` (регистронезависимо)
- строки короче 3 символов после trim (`--min-len 3`)
- пустые строки, комментарии (`#`, `;`)
- **hash-строки:** чистый MD5/SHA1 hex (32/40 символов), `hash:pass`, `email:hash`
- **combo-строки:** любая строка с символом `:` (`login:pass`, `email:pass`, `domain:pass`)
- строки только из спецсимволов

**Сохраняется:** только plain passwords без `:` — `password123`, `qwerty`, `MyP@ss!2024`.

**Dedupe:** hash-partition (256 buckets) + in-memory set на bucket — без OOM на GB-файлах.

```bat
MERGE-CLEAN.bat "D:\wordlists" "D:\merged_clean.txt"
MERGE-CLEAN.bat "D:\wordlists" "merged.txt" recursive

engine\target\release\TextMerger.exe merge --input wordlists --output merged.txt --recursive --min-len 3 --threads 32
```

Статистика: `empty_trash`, `short_trash`, `null_trash`, `hash_trash`, `combo_trash`, `duplicates`.

Примеры:

```bat
engine\target\release\LocalHashFinder.exe --db data lookup 5f4dcc3b5aa765d61d8327deb882cf99
engine\target\release\LocalHashFinder.exe process input.txt --threads 64
engine\target\release\LocalHashFinder.exe extract-regex dump.sql --pattern "..." --template "$1:$2"
```

Полный список подкоманд: `LocalHashFinder.exe --help`

---

## Структура проекта

```
LocalHashFinder/
├── BUILD.bat
├── START-LOCAL-HASH.bat
├── START-WEB.bat
├── IMPORT-DB.bat
├── APPEND-DB.bat
├── WORDLIST-HASH.bat
├── WORDLIST-HASH-FOLDER.bat
├── MERGE.bat
├── MERGE-CLEAN.bat
├── EXTRACT-SQL.bat
├── SCRIPTS.md
├── LICENSE
├── CONTRIBUTING.md
├── README.md
├── .gitignore
├── .github/workflows/rust.yml
├── design/
│   ├── figma-reference.html
│   ├── figma-reference-lookup.html
│   └── assets-b64.txt          # dev-артефакт (в .gitignore)
└── engine/
    ├── Cargo.toml
    ├── assets/                   # иконки UI
    └── src/
        ├── main.rs               # CLI + точка входа GUI
        ├── app.rs                # egui UI, 7 вкладок
        ├── db.rs                 # LMDB import/append/lookup
        ├── processor.rs          # пакетная расшифровка
        ├── merger.rs             # склейка mail+dehash
        ├── sql_extract.rs        # SQL regex extract
        ├── sql_columns.rs        # SQL column extract
        ├── regex_extract.rs      # custom regex
        ├── config.rs             # LocalHashFinder.cfg
        ├── i18n.rs               # RU/EN строки
        ├── combo/                # ComboKit (11 tools)
        └── ulp/                  # SwiftyULP (18 services)
```

---

## Технологии

| Компонент | Стек |
|-----------|------|
| Язык | Rust 2021 |
| GUI | [egui](https://github.com/emilk/egui) / [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) |
| База хешей | [LMDB](https://www.symas.com/lmdb) через [heed](https://github.com/meilisearch/heed) |
| Параллелизм | [rayon](https://github.com/rayon-rs/rayon) |
| Архивы ULP | [zip](https://docs.rs/zip), [sevenz-rust](https://docs.rs/sevenz-rust) |
| CLI | [clap](https://docs.rs/clap) |
| Regex | [regex](https://docs.rs/regex) |

---



---

## Отказ от ответственности

Local Hash Finder — **офлайн-инструмент** для работы с **вашими собственными данными** на вашем компьютере.

- Приложение **не подключается** к серверам для расшифровки или хранения данных
- Разработчики **не предоставляют** базы паролей и не поощряют несанкционированный доступ
- Вы несёте ответственность за соблюдение законодательства и правил использования данных
- Не публикуйте в Issues реальные дампы, combo-листы или персональные данные

---

## Лицензия

[MIT License](LICENSE) — Copyright (c) 2026 LocalHashFinder contributors

---

# English

**Offline desktop toolkit for hash lookup, combo processing, and ULP tools.**

Local Hash Finder is a portable Windows application written in Rust. All processing runs locally — no cloud, no license server, no data upload. You provide your own LMDB hash database and input files.

---

## Table of contents (EN)

- [Features](#features)
- [User interface](#user-interface)
- [Requirements](#requirements)
- [Quick start](#quick-start)
- [Wordlist sources (plaintext)](#wordlist-sources-plaintext)
- [LMDB database](#lmdb-database)
- [Configuration](#configuration)
- [CLI and batch scripts](#cli-and-batch-scripts)
- [SCRIPTS.md — full batch script reference](SCRIPTS.md)
- [Project structure](#project-structure)
- [Tech stack](#tech-stack)
- [Screenshots](#screenshots-en)
- [Disclaimer](#disclaimer)
- [License](#license-en)

---

## Features

Seven sidebar modules:

### 1. Hash Lookup

Batch password lookup for MD5/SHA1 against a local LMDB database.

- Open/select `hashdb.lmdb` path
- Batch file processing (`email:hash`, `hash`, `email:hash:extra`)
- Configurable thread count (up to 512)
- Outputs: `{name}_good.txt`, `{name}_nohash.txt`, `{name}_bad.txt`, `{name}_trash.txt`
- **Import** — initial `hash:pass` load (`IMPORT-DB.bat` / CLI)
- **Append** — add entries, skip duplicate keys (`APPEND-DB.bat` / GUI)
- Pause, stop, open results in Explorer

### 2. Merge

Combine `mail:hashedpass` + `hash:plainpass` → `mail:plainpass`.

- Outputs: `{stem}_plain.txt`, `{stem}_plain_nohash.txt`, `{stem}_trash.txt`
- CLI: `merge --mail … --dehash …` / `MERGE.bat`

### 3. SQL Extract

Extract `email:md5` / `email:sha1` from SQL dumps via regex.

- Single file or folder batch (`.sql`, `.txt`, `.dump`)
- Parallel processing (up to 5 threads), UTF-8 / Windows-1252
- CLI: `extract-sql` / `EXTRACT-SQL.bat`

### 4. SQL Columns

Extract `login:password` by column names in `CREATE TABLE` / `INSERT`.

- Detects login/email/user and password/pass/pwd columns
- Folder batch mode
- CLI: `extract-sql-columns`

### 5. Custom Regex

Custom regex extraction with output templates (`$1`, `$2`, `${name}`).

- Presets: `email:md5`, `email:sha1`, `hash:pass`, email only
- Flags: case-insensitive, multiline, dotall, dedupe
- CLI: `extract-regex`

### 6. ComboKit (11 tools)

Compare · Combo filter · Email filter · Name/Password split · MX check · Scraper (txt/sql/json) · Provider analyze · Dedupe · Filter lines · Merge lines · Split file

### 7. ULP / SwiftyULP (18 services)

Sort · Sort Country · Sort Keyword · Search · Extract url:login:pass · Extract login:pass · Extract user:pass · Clean (dedupe, empty, junk, blacklist, chars, weak, protocols, capture) · Misc (merge, split, filter)

Supports `.zip` / `.7z` / `.rar` archives and folders.

---

## User interface

- Crypto-dashboard dark theme (RecehTok-inspired)
- **RU / EN** language toggle
- **Frameless** custom window chrome (rounded corners on Windows)
- Per-tab log, results panel, and instructions

---

## Requirements

- Windows 10 / 11
- Rust (stable) + `cargo` for building from source
- Sufficient disk space for LMDB (large maps for huge hash files)

---

## Quick start

```bat
BUILD.bat
START-LOCAL-HASH.bat
```

Binary: `engine/target/release/LocalHashFinder.exe`

---

## Wordlist sources (plaintext)

Public resources for **plain-password** lists (use only legally and on systems you own):

**Catalogs and sites**

| Resource | Description |
|----------|-------------|
| [HashMob](https://hashmob.net/) | Hash lookup community; wordlists and research materials |
| [Weakpass](https://weakpass.com/wordlist) | Large public wordlists (partially free) |
| [g0tmi1k — wordlists](https://download.g0tmi1k.com/wordlists) | Wordlist catalog for pentest and security audits |
| [SecLists — Passwords](https://github.com/danielmiessler/SecLists/tree/master/Passwords) | Classics: `rockyou.txt`, `10k-most-common.txt`, `best1050.txt` |

**Large plaintext dumps (Mail.ru Cloud, .txt)**

| Link | Size |
|------|------|
| [260M passwords](https://cloud.mail.ru/public/HsHb/JamxkSKRF) | ~260M |
| [243M passwords](https://cloud.mail.ru/public/hEUf/XJkHjc6Ny) | ~243M |
| [358M passwords](https://cloud.mail.ru/public/EnHr/Qx1hYDDrC) | ~358M |
| [112M passwords](https://cloud.mail.ru/public/K6Ho/HCcQFxHNH) | ~112M |
| [950M passwords](https://cloud.mail.ru/public/eo1N/SYYp5gELP) | ~950M |
| [999M+ passwords](https://cloud.mail.ru/public/Coni/QgPayRbhv) | 999M+ |

Plan for large disk space on hundred-million-line files; run `MERGE-CLEAN.bat` before hashing.

**LocalHashFinder pipeline:**

```
download .txt → MERGE-CLEAN.bat (passwords only) → WORDLIST-HASH-FOLDER.bat → IMPORT-DB.bat
```

---

## LMDB database

Default path: `engine/target/release/data/hashdb.lmdb`

- **Import:** `IMPORT-DB.bat "D:\hash_pass.txt" [map_gb]`
- **Append:** `APPEND-DB.bat "D:\new_hashes.txt" [map_gb]`
- Line format: `hash:password` (32 or 40 hex chars)
- Wordlist → `hash:pass` (MD5 by default): `WORDLIST-HASH.bat "passwords.txt"` (single file) or `WORDLIST-HASH-FOLDER.bat "D:\wordlists"` (all top-level `*.txt`; default folder `wordlists\`). Output: `{random}_{stem}_md5.txt` (e.g. `847291_passwords_md5.txt`). Optional 2nd arg: `sha1` or `both`.

Close the GUI before import/append operations.

---

## Configuration

`LocalHashFinder.cfg` next to the executable:

```ini
lmdb_path=...\data\hashdb.lmdb
ui_lang=en
ui_zoom=1.10
```

---

## CLI and batch scripts

> **Full reference:** [SCRIPTS.md](SCRIPTS.md) — purpose, arguments, examples, and workflows for every `.bat`.

| Script | Purpose |
|--------|---------|
| `BUILD.bat` | Release build |
| `START-LOCAL-HASH.bat` | Launch GUI |
| `START-WEB.bat` | Legacy web UI (Node.js, requires `server.js`) |
| `WORDLIST-HASH.bat` | plaintext wordlist → `hash:pass` MD5 (single file; output `{random}_{stem}_md5.txt`) |
| `WORDLIST-HASH-FOLDER.bat` | same for all `*.txt` in a folder (MD5 default; skips hash outputs) |
| `IMPORT-DB.bat` | LMDB import |
| `APPEND-DB.bat` | LMDB append |
| `MERGE.bat` | Mail + dehash merge |
| `EXTRACT-SQL.bat` | SQL email:hash extract |
| `MERGE-CLEAN.bat` | `TextMerger merge` — merge wordlists, clean `:NULL`/hashes/short lines, dedupe |

### TextMerger (password wordlists)

Standalone `TextMerger.exe` merges `.txt` files into one **plain-password-only** wordlist (one password per line, deduplicated).

**Removed as garbage:** `:NULL` lines, lines shorter than 3 chars, empty/comment lines, pure MD5/SHA1 hex, `hash:pass`, `email:hash`, any line containing `:` (combo formats).

**Kept:** plain passwords only — `password123`, `qwerty`, `MyP@ss!2024` (no colons).

```bat
MERGE-CLEAN.bat "D:\wordlists" "merged_clean.txt"
engine\target\release\TextMerger.exe merge --input wordlists --output merged.txt --recursive
```

Run `LocalHashFinder.exe --help` for all subcommands.

---

## Project structure

See the tree in the [Russian section](#структура-проекта) above.

---

## Tech stack

Rust · egui/eframe · LMDB (heed) · rayon · zip · sevenz-rust · clap · regex

CI: `.github/workflows/rust.yml` (`cargo check`, `cargo test` on Windows)

---

## Screenshots

| Description | Path |
|-------------|------|
| UI mockup | `design/figma-reference.html` |
| Lookup tab mockup | `design/figma-reference-lookup.html` |
| App screenshot | `docs/screenshot-main.png` *(add before release)* |

---

## Disclaimer

Local Hash Finder is an **offline tool** for **your own data** on **your machine**.

- No server-side cracking or storage
- No hash databases are shipped with the project
- You are responsible for lawful use
- Do not attach real dumps or PII to GitHub Issues

---

## License

[MIT License](LICENSE) — Copyright (c) 2026 LocalHashFinder contributors
