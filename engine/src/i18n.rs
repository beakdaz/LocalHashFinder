//! UI strings — Russian / English.

pub const ERR_PREFIX: &str = "LHF_ERR:";
pub const FOLDER_BATCH_PREFIX: &str = "LHF_FOLDER:";

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum Lang {
    #[default]
    Ru,
    En,
}

impl Lang {
    pub fn parse(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "en" | "english" => Lang::En,
            _ => Lang::Ru,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Lang::Ru => "ru",
            Lang::En => "en",
        }
    }

    pub fn toggle(self) -> Self {
        match self {
            Lang::Ru => Lang::En,
            Lang::En => Lang::Ru,
        }
    }
}

pub fn wrap_err(msg: &str) -> String {
    format!("{ERR_PREFIX}{msg}")
}

pub fn is_err(s: &str) -> bool {
    s.starts_with(ERR_PREFIX)
}

pub fn err_body(s: &str) -> &str {
    s.strip_prefix(ERR_PREFIX).unwrap_or(s)
}

pub fn is_folder_batch(s: &str) -> bool {
    s.starts_with(FOLDER_BATCH_PREFIX)
}

pub fn format_error_display(lang: Lang, raw: &str) -> String {
    if is_err(raw) {
        format!("{} {}", tr(lang).error_prefix, err_body(raw))
    } else if let Some(rest) = raw.strip_prefix("Ошибка:") {
        format!("{} {}", tr(lang).error_prefix, rest.trim())
    } else if let Some(rest) = raw.strip_prefix("Error:") {
        format!("{} {}", tr(lang).error_prefix, rest.trim())
    } else {
        raw.to_string()
    }
}

pub struct I18n {
    pub app_subtitle: &'static str,
    pub browse: &'static str,
    pub file_btn: &'static str,
    pub folder_btn: &'static str,
    pub folder_hint_dumps: &'static str,
    pub instruction: &'static str,
    pub btn_instruction: &'static str,
    pub btn_close: &'static str,
    pub lang_label: &'static str,
    pub zoom_label: &'static str,
    pub zoom_out_tip: &'static str,
    pub zoom_in_tip: &'static str,
    pub tab_lookup: &'static str,
    pub tab_merge: &'static str,
    pub tab_sql: &'static str,
    pub tab_columns: &'static str,
    pub tab_regex: &'static str,
    pub tab_combo: &'static str,
    pub combo_tool_compare: &'static str,
    pub combo_tool_filter: &'static str,
    pub combo_tool_email: &'static str,
    pub combo_tool_namepw: &'static str,
    pub combo_tool_mx: &'static str,
    pub combo_tool_scraper: &'static str,
    pub combo_tool_analyze: &'static str,
    pub combo_tool_dedupe: &'static str,
    pub combo_tool_line_filter: &'static str,
    pub combo_tool_merge: &'static str,
    pub combo_tool_split: &'static str,
    pub section_combo: &'static str,
    pub heading_combo: &'static str,
    pub hint_combo_input_b: &'static str,
    pub hint_combo_output_dir: &'static str,
    pub hint_combo_input: &'static str,
    pub hint_combo_output: &'static str,
    pub hint_combo_output_idle: &'static str,
    pub hint_combo_filter: &'static str,
    pub hint_combo_lines: &'static str,
    pub label_combo_tool: &'static str,
    pub combo_subtitle: &'static str,
    pub combo_instr_body: &'static str,
    pub combo_instr_tools: &'static str,
    pub combo_instr_tools_mono: &'static str,
    pub combo_instr_formats: &'static str,
    pub combo_instr_formats_mono: &'static str,
    pub combo_instr_outputs: &'static str,
    pub combo_instr_outputs_mono: &'static str,
    pub combo_instr_controls: &'static str,
    pub combo_instr_controls_body: &'static str,
    pub err_combo_output: &'static str,
    pub err_combo_output_dir: &'static str,
    pub err_combo_input_b: &'static str,
    pub tab_ulp: &'static str,
    pub ulp_tool_sort: &'static str,
    pub ulp_tool_sort_country: &'static str,
    pub ulp_tool_sort_keyword: &'static str,
    pub ulp_tool_search: &'static str,
    pub ulp_tool_extract_ulp: &'static str,
    pub ulp_tool_extract_lp: &'static str,
    pub ulp_tool_extract_up: &'static str,
    pub ulp_tool_clean_dedupe: &'static str,
    pub ulp_tool_clean_empty: &'static str,
    pub ulp_tool_clean_junk: &'static str,
    pub ulp_tool_clean_blacklist: &'static str,
    pub ulp_tool_clean_chars: &'static str,
    pub ulp_tool_clean_weak: &'static str,
    pub ulp_tool_clean_proto: &'static str,
    pub ulp_tool_clean_capture: &'static str,
    pub ulp_tool_misc_merge: &'static str,
    pub ulp_tool_misc_split: &'static str,
    pub ulp_tool_misc_filter: &'static str,
    pub section_ulp: &'static str,
    pub heading_ulp: &'static str,
    pub ulp_subtitle: &'static str,
    pub label_ulp_tool: &'static str,
    pub hint_ulp_input: &'static str,
    pub hint_ulp_output: &'static str,
    pub hint_ulp_output_dir: &'static str,
    pub hint_ulp_keywords: &'static str,
    pub hint_ulp_output_idle: &'static str,
    pub ulp_instr_body: &'static str,
    pub ulp_instr_tools: &'static str,
    pub ulp_instr_tools_mono: &'static str,
    pub ulp_instr_outputs: &'static str,
    pub ulp_instr_outputs_mono: &'static str,
    pub ulp_instr_controls: &'static str,
    pub ulp_instr_controls_body: &'static str,
    pub err_ulp_output: &'static str,
    pub err_ulp_output_dir: &'static str,
    pub badge_ulp: &'static str,
    pub badge_combo: &'static str,
    pub badge_lookup: &'static str,
    pub badge_merge: &'static str,
    pub badge_sql: &'static str,
    pub badge_columns: &'static str,
    pub badge_regex: &'static str,
    pub badge_append: &'static str,
    pub badge_idle: &'static str,
    pub btn_start: &'static str,
    pub btn_merge: &'static str,
    pub btn_extract: &'static str,
    pub btn_pause: &'static str,
    pub btn_resume: &'static str,
    pub btn_stop: &'static str,
    pub btn_results: &'static str,
    pub btn_delete: &'static str,
    pub btn_zip: &'static str,
    pub btn_merge_one: &'static str,
    pub btn_open: &'static str,
    pub btn_add: &'static str,
    pub section_params: &'static str,
    pub section_log: &'static str,
    pub log_waiting: &'static str,
    pub error_prefix: &'static str,
    pub stat_total: &'static str,
    pub stat_found: &'static str,
    pub stat_not_found: &'static str,
    pub stat_trash: &'static str,
    pub section_database: &'static str,
    pub section_append: &'static str,
    pub section_files: &'static str,
    pub section_batch: &'static str,
    pub section_threads: &'static str,
    pub section_progress: &'static str,
    pub section_output: &'static str,
    pub section_source: &'static str,
    pub section_merge: &'static str,
    pub section_mail: &'static str,
    pub section_dehash: &'static str,
    pub section_preset: &'static str,
    pub heading_lmdb: &'static str,
    pub heading_input_file: &'static str,
    pub heading_progress: &'static str,
    pub heading_result: &'static str,
    pub heading_file_or_folder: &'static str,
    pub heading_parallelism: &'static str,
    pub heading_email_hash: &'static str,
    pub heading_mail_plain: &'static str,
    pub heading_custom_regex: &'static str,
    pub heading_loginpass: &'static str,
    pub err_lmdb_path: &'static str,
    pub err_no_lmdb_hint: &'static str,
    pub err_save_config: &'static str,
    pub err_input_file: &'static str,
    pub err_file_not_found: &'static str,
    pub err_path_not_found: &'static str,
    pub err_file_or_folder: &'static str,
    pub err_both_merge_files: &'static str,
    pub err_merge_file_missing: &'static str,
    pub err_regex_source: &'static str,
    pub err_regex_pattern: &'static str,
    pub err_regex_template: &'static str,
    pub err_hashpass_file: &'static str,
    pub err_stop_lookup_before_append: &'static str,
    pub dialog_lmdb_folder: &'static str,
    pub db_entries: &'static str,
    pub db_pick_lmdb: &'static str,
    pub db_path_saved_empty: &'static str,
    pub label_hash_db: &'static str,
    pub tip_hash_db_path: &'static str,
    pub log_start: &'static str,
    pub log_start_threads: &'static str,
    pub log_stop_requested: &'static str,
    pub log_pause: &'static str,
    pub log_resume: &'static str,
    pub log_folder_opened: &'static str,
    pub log_deleted: &'static str,
    pub log_delete_err: &'static str,
    pub log_zip: &'static str,
    pub log_zip_err: &'static str,
    pub log_merged_one: &'static str,
    pub log_merge_err: &'static str,
    pub log_batch_done: &'static str,
    pub status_stopping: &'static str,
    pub status_merge_loading: &'static str,
    pub status_sql_batch: &'static str,
    pub status_sql_parsing: &'static str,
    pub status_sqlcol_batch: &'static str,
    pub status_sqlcol_parsing: &'static str,
    pub status_append: &'static str,
    pub status_regex_extract: &'static str,
    pub status_waiting_start: &'static str,
    pub status_progress_after_start: &'static str,
    pub hint_input_formats: &'static str,
    pub hint_input_placeholder: &'static str,
    pub hint_merge_mail: &'static str,
    pub hint_merge_dehash: &'static str,
    pub hint_merge_output: &'static str,
    pub hint_sql_source: &'static str,
    pub hint_sql_output: &'static str,
    pub hint_sql_threads: &'static str,
    pub hint_sqlcol_output: &'static str,
    pub hint_regex_output: &'static str,
    pub hint_regex_template_label: &'static str,
    pub hint_lmdb_path: &'static str,
    pub hint_append_file: &'static str,
    pub lookup_instr_body: &'static str,
    pub lookup_instr_formats: &'static str,
    pub lookup_instr_formats_mono: &'static str,
    pub lookup_instr_outputs: &'static str,
    pub lookup_instr_outputs_mono: &'static str,
    pub lookup_instr_controls: &'static str,
    pub lookup_instr_controls_body: &'static str,
    pub merge_instr_body: &'static str,
    pub merge_instr_formats: &'static str,
    pub merge_instr_formats_mono: &'static str,
    pub merge_instr_outputs: &'static str,
    pub merge_instr_outputs_mono: &'static str,
    pub merge_subtitle: &'static str,
    pub merge_heading: &'static str,
    pub sql_instr_body: &'static str,
    pub sql_instr_extracted: &'static str,
    pub sql_instr_extracted_mono: &'static str,
    pub sql_instr_outputs: &'static str,
    pub sql_instr_outputs_mono: &'static str,
    pub sql_heading: &'static str,
    pub sql_subtitle: &'static str,
    pub sqlcol_intro: &'static str,
    pub sqlcol_instr_body: &'static str,
    pub sqlcol_login_cols: &'static str,
    pub sqlcol_login_cols_mono: &'static str,
    pub sqlcol_pass_cols: &'static str,
    pub sqlcol_pass_cols_mono: &'static str,
    pub sqlcol_example: &'static str,
    pub sqlcol_example_mono: &'static str,
    pub regex_intro: &'static str,
    pub regex_instr_body: &'static str,
    pub regex_template_heading: &'static str,
    pub regex_template_mono: &'static str,
    pub regex_flags_heading: &'static str,
    pub regex_flags_mono: &'static str,
    pub regex_examples_heading: &'static str,
    pub regex_examples_mono: &'static str,
    pub regex_cli_heading: &'static str,
    pub regex_cli_mono: &'static str,
    pub regex_pattern_label: &'static str,
    pub regex_output_template_label: &'static str,
    pub regex_flag_i: &'static str,
    pub regex_flag_m: &'static str,
    pub regex_flag_s: &'static str,
    pub regex_flag_dedupe: &'static str,
    pub section_regex_engine: &'static str,
    pub section_sql: &'static str,
    pub section_sql_columns: &'static str,
    pub no_result_files: &'static str,
}

static RU: I18n = I18n {
    app_subtitle: "Только ваша база · без лицензий и серверов",
    browse: "Обзор…",
    file_btn: "Файл…",
    folder_btn: "Папка…",
    folder_hint_dumps: "Папка: все .sql / .txt / .dump",
    instruction: "Инструкция",
    btn_instruction: "Инструкция",
    btn_close: "Закрыть",
    lang_label: "Язык",
    zoom_label: "Масштаб",
    zoom_out_tip: "Уменьшить (Ctrl + колёсико)",
    zoom_in_tip: "Увеличить (Ctrl + колёсико)",
    tab_lookup: "Расшифровка",
    tab_merge: "Склейка",
    tab_sql: "SQL Extract",
    tab_columns: "SQL Колонки",
    tab_regex: "Custom Regex",
    tab_combo: "ComboKit",
    combo_tool_compare: "Сравнение списков",
    combo_tool_filter: "Фильтр combo",
    combo_tool_email: "Email-фильтр",
    combo_tool_namepw: "Split name/password",
    combo_tool_mx: "Проверка MX",
    combo_tool_scraper: "Scraper (txt/sql/json)",
    combo_tool_analyze: "Анализ по доменам",
    combo_tool_dedupe: "Dedupe строк",
    combo_tool_line_filter: "Фильтр строк",
    combo_tool_merge: "Merge строк",
    combo_tool_split: "Split файла",
    section_combo: "ComboKit",
    heading_combo: "ComboKit — инструменты для combo-листов",
    hint_combo_input_b: "Второй файл (compare)",
    hint_combo_output_dir: "output/",
    hint_combo_input: "input.txt",
    hint_combo_output: "output.txt",
    hint_combo_output_idle: "→ файл или папка — зависит от выбранного инструмента",
    hint_combo_filter: "Фильтр (contains / regex):",
    hint_combo_lines: "Строк на файл:",
    label_combo_tool: "Инструмент",
    combo_subtitle: "Фильтрация, dedupe, split/merge, scraper, MX и compare — без сервера",
    combo_instr_body: "1. Выберите инструмент ComboKit в списке\n\
2. Укажите входной .txt (для Compare — два файла)\n\
3. Укажите выходной файл или папку (см. таблицу ниже)\n\
4. «Извлечь» внизу → лог и «Результаты» откроют папку\n\
5. База LMDB не нужна — всё локально, без сервера",
    combo_instr_tools: "Инструменты",
    combo_instr_tools_mono: "Compare       — сравнение двух списков (only_a / only_b / both)\n\
Combo filter  — валидация email:pass, отсев мусора\n\
Email filter  — только строки с email\n\
Name/Pass     — разделить на names.txt + passwords.txt\n\
MX check      — проверка MX-записей доменов\n\
Scraper       — извлечь combo из .txt / .sql / .json\n\
Analyze       — группировка по доменам (by_domain/)\n\
Dedupe        — уникальные строки\n\
Line filter   — contains или regex\n\
Merge / Split — склеить или разбить файл по N строк",
    combo_instr_formats: "Входные форматы",
    combo_instr_formats_mono: "combo:   user@gmail.com:password123\n\
lines:   одна строка = одна запись\n\
scraper: .txt / .sql / .json — email:pass из дампов и логов",
    combo_instr_outputs: "Выходные файлы",
    combo_instr_outputs_mono: "Compare / MX / Analyze / Name-Pass → папка:\n\
  only_a.txt, only_b.txt, both.txt · mx_*.txt · by_domain/\n\
  names.txt + passwords.txt\n\
Остальные инструменты → один .txt (output.txt)",
    combo_instr_controls: "Управление",
    combo_instr_controls_body: "Извлечь / Стоп / Пауза — панель «Параметры» внизу.\n\
«Результаты» — открыть папку с последним выходом.\n\
«В архив» / «В один файл» — для нескольких результатов.",
    err_combo_output: "Укажите выходной файл",
    err_combo_output_dir: "Укажите папку результата",
    err_combo_input_b: "Укажите второй файл (compare)",
    tab_ulp: "ULP",
    ulp_tool_sort: "Sort — сортировка по типам",
    ulp_tool_sort_country: "Sort — по стране/TLD",
    ulp_tool_sort_keyword: "Sort — по keyword (папки)",
    ulp_tool_search: "Search — keyword → один файл",
    ulp_tool_extract_ulp: "Extract url:login:pass",
    ulp_tool_extract_lp: "Extract login:pass",
    ulp_tool_extract_up: "Extract user:pass",
    ulp_tool_clean_dedupe: "Clean — dedupe строк",
    ulp_tool_clean_empty: "Clean — пустые строки",
    ulp_tool_clean_junk: "Clean — junk-символы",
    ulp_tool_clean_blacklist: "Clean — blacklist домены",
    ulp_tool_clean_chars: "Clean — пустые символы",
    ulp_tool_clean_weak: "Clean — слабые пароли",
    ulp_tool_clean_proto: "Clean — убрать протоколы",
    ulp_tool_clean_capture: "Clean — capture-строки",
    ulp_tool_misc_merge: "Misc — merge файлов",
    ulp_tool_misc_split: "Misc — split по строкам",
    ulp_tool_misc_filter: "Misc — filter по keyword",
    section_ulp: "SwiftyULP",
    heading_ulp: "ULP-сервисы — sort / extract / clean",
    ulp_subtitle: "Обработка url:login:pass локально, без лицензии и сервера",
    label_ulp_tool: "Сервис",
    hint_ulp_input: "input.txt, .zip/.7z/.rar или папка",
    hint_ulp_output: "output.txt",
    hint_ulp_output_dir: "output/",
    hint_ulp_keywords: "Keywords (через запятую):",
    hint_ulp_output_idle: "→ Sort*: папка · Search/Extract/Clean/Misc → один .txt",
    ulp_instr_body: "1. Выберите ULP-сервис (Sort, Search, Extract, Clean, Misc)\n\
2. Укажите входной .txt, архив (.zip/.7z/.rar) или папку\n\
3. Укажите выходной файл или папку (Sort* → папка)\n\
4. «Извлечь» внизу — результат в логе и «Результаты»",
    ulp_instr_tools: "Сервисы SwiftyULP",
    ulp_instr_tools_mono: "Sort      → Mails / Mail Pass / Phone Pass / User Pass / ULP\n\
Sort TLD  → by_tld/ + by_domain/\n\
Sort KW   → keyword.txt на каждый term\n\
Search    → matched lines → один файл\n\
Extract   → url:login:pass | login:pass | user:pass\n\
Clean     → dedupe, empty, junk, blacklist, weak, protocols, capture\n\
Misc      → merge, split, filter",
    ulp_instr_outputs: "Выходные файлы",
    ulp_instr_outputs_mono: "Sort/     → output/Mails.txt, Mail Pass.txt, ULP.txt …\n\
Sort TLD/ → by_tld/de.txt, by_domain/gmail.com.txt …\n\
Sort KW/  → gmail.com.txt, paypal.txt …\n\
Extract/  → output.txt (один файл)\n\
Clean/    → cleaned.txt",
    ulp_instr_controls: "Управление",
    ulp_instr_controls_body: "Извлечь / Стоп — панель «Параметры» внизу.\n\
Keywords — Sort KW, Search, Extract, Misc filter.\n\
«Результаты» — открыть папку с последним выходом.",
    err_ulp_output: "Укажите выходной файл",
    err_ulp_output_dir: "Укажите папку результата",
    badge_ulp: "ULP",
    badge_combo: "Combo",
    badge_lookup: "Расшифровка",
    badge_merge: "Склейка",
    badge_sql: "SQL",
    badge_columns: "Колонки",
    badge_regex: "Regex",
    badge_append: "База",
    badge_idle: "Idle",
    btn_start: "Старт",
    btn_merge: "Склеить",
    btn_extract: "Извлечь",
    btn_pause: "Пауза",
    btn_resume: "Продолжить",
    btn_stop: "Стоп",
    btn_results: "Результаты",
    btn_delete: "Удалить рез",
    btn_zip: "В архив",
    btn_merge_one: "В один файл",
    btn_open: "Открыть",
    btn_add: "Добавить",
    section_params: "Параметры",
    section_log: "Лог",
    log_waiting: "Ожидание событий…",
    error_prefix: "Ошибка:",
    stat_total: "Всего",
    stat_found: "Найдено",
    stat_not_found: "Не найдено",
    stat_trash: "Мусор",
    section_database: "База данных",
    section_append: "Дополнение",
    section_files: "Файлы",
    section_batch: "Пакет",
    section_threads: "Потоки",
    section_progress: "Прогресс",
    section_output: "Вывод",
    section_source: "Источник",
    section_merge: "Склейка",
    section_mail: "Mail",
    section_dehash: "Dehash",
    section_preset: "Пресет",
    heading_lmdb: "LMDB",
    heading_input_file: "Входной файл",
    heading_progress: "Прогресс",
    heading_result: "Результат",
    heading_file_or_folder: "Файл или папка",
    heading_parallelism: "Параллельность",
    heading_email_hash: "email:hash",
    heading_mail_plain: "hash:pass",
    heading_custom_regex: "Своё извлечение по регулярке",
    heading_loginpass: "login:password по именам колонок",
    err_lmdb_path: "Укажите путь к LMDB",
    err_no_lmdb_hint: "Нет LMDB — выберите папку базы и нажмите «Открыть»",
    err_save_config: "Не удалось сохранить config",
    err_input_file: "Укажите входной файл",
    err_file_not_found: "Файл не найден",
    err_path_not_found: "Путь не найден",
    err_file_or_folder: "Укажите файл или папку",
    err_both_merge_files: "Укажите оба файла: mail и dehash",
    err_merge_file_missing: "Один из файлов не найден",
    err_regex_source: "Укажите исходный файл",
    err_regex_pattern: "Укажите regex",
    err_regex_template: "Укажите шаблон вывода",
    err_hashpass_file: "Укажите файл hash:pass",
    err_stop_lookup_before_append: "Остановите расшифровку перед добавлением в базу",
    dialog_lmdb_folder: "Папка LMDB (hashdb.lmdb, data или любая для новой базы)",
    db_entries: "База: {n} записей",
    db_pick_lmdb: "Выберите папку LMDB и нажмите «Открыть»",
    db_path_saved_empty: "Путь сохранён. База пуста — import или «Добавить»",
    label_hash_db: "База хэшей:",
    tip_hash_db_path: "Полный путь к LMDB. Выделите текст для копирования.",
    log_start: "Старт: {path}",
    log_start_threads: "Старт: {path} (потоков: {threads})",
    log_stop_requested: "Запрошена остановка",
    log_pause: "Пауза",
    log_resume: "Продолжение",
    log_folder_opened: "Открыта папка: {path}",
    log_deleted: "Удалено файлов: {n}",
    log_delete_err: "Ошибка удаления: {e}",
    log_zip: "Архив: {path}",
    log_zip_err: "Ошибка архива: {e}",
    log_merged_one: "Склеено в один файл: {path}",
    log_merge_err: "Ошибка склейки: {e}",
    log_batch_done: "Пакетная обработка завершена",
    status_stopping: "Остановка…",
    status_merge_loading: "Загрузка dehash и склейка…",
    status_sql_batch: "Пакетная обработка папки ({threads} поток.)…",
    status_sql_parsing: "Парсинг SQL (regex, {threads} поток.)…",
    status_sqlcol_batch: "Пакетная обработка папки…",
    status_sqlcol_parsing: "Парсинг SQL по колонкам…",
    status_append: "Добавление в LMDB…",
    status_regex_extract: "Regex extract…",
    status_waiting_start: "Ожидание запуска…",
    status_progress_after_start: "Прогресс и статистика появятся после «Старт»",
    hint_input_formats: "email:hash · hash · email:hash:extra",
    hint_input_placeholder: "email:hash или hash",
    hint_merge_mail: "user@gmail.com:md5…",
    hint_merge_dehash: "input_good.txt",
    hint_merge_output: "→ *_plain.txt и *_plain_nohash.txt",
    hint_sql_source: "dump.sql или папка с дампами",
    hint_sql_output: "файл или папка → {name}_emails.txt и {name}_trash.txt рядом с каждым дампом",
    hint_sql_threads: "Папка: до N файлов одновременно · один файл: пакеты строк regex",
    hint_sqlcol_output: "файл или папка → {name}_loginpass.txt рядом с каждым дампом",
    hint_regex_output: "→ {name}_regex.txt рядом с исходником",
    hint_regex_template_label: "Шаблон:",
    hint_lmdb_path: "D:\\bases\\hashdb.lmdb",
    hint_append_file: "new_hashes.txt",
    lookup_instr_body: "1. Укажите папку LMDB (hashdb.lmdb) и нажмите «Открыть»\n\
2. Выберите .txt с хешами — поддерживаемые форматы ниже\n\
3. Настройте потоки (размер пакета) для больших файлов\n\
4. «Старт» внизу → рядом с файлом появятся результаты\n\
5. «Дополнение» — добавить hash:pass в базу без дубликатов",
    lookup_instr_formats: "Форматы входа",
    lookup_instr_formats_mono: "email:hash\nhash\nemail:hash:extra\nuser@gmail.com:5f4dcc3b5aa765d61d8327deb882cf99",
    lookup_instr_outputs: "Выходные файлы",
    lookup_instr_outputs_mono: "{name}_good.txt    — hash:pass (найдено)\n\
{name}_nohash.txt  — hash без совпадения\n\
{name}_bad.txt     — невалидные строки\n\
{name}_trash.txt   — мусор / лишние поля",
    lookup_instr_controls: "Управление",
    lookup_instr_controls_body: "Старт / Пауза / Стоп / Результаты — панель «Параметры» внизу.\n\
«В один файл» — склеить _good / _nohash / _bad в один .txt (без _trash).",
    merge_instr_body: "1. Mail — файл email:hash (md5-хеш пароля от аккаунта)\n\
2. Dehash — _good.txt из расшифровки (hash:plainpass)\n\
3. «Склеить» внизу → склеивает по совпадению hash\n\
4. «Результаты» откроет папку с выходными файлами",
    merge_instr_formats: "Форматы",
    merge_instr_formats_mono: "mail:   user@gmail.com:5f4dcc3b5aa765d61d8327deb882cf99  (md5)\n\
mail:   user@gmail.com:dabd7bfb00119f1ee6baaddbb5e2150308b70599  (sha1)\n\
dehash: 5f4dcc3b5aa765d61d8327deb882cf99:password123",
    merge_instr_outputs: "Выходные файлы",
    merge_instr_outputs_mono: "{name}_plain.txt       — email:plainpass (успешно)\n\
{name}_plain_nohash.txt — email без dehash\n\
{name}_plain_trash.txt  — мусорные строки",
    merge_subtitle: "mail: md5/sha1-хеш пароля · dehash: _good.txt (hash:pass)",
    merge_heading: "mail:hash + hash:pass → mail:plainpass",
    sql_instr_body: "1. Выберите .sql дамп или папку с дампами\n\
2. Regex ищет email + md5/sha1 в INSERT и VALUES\n\
3. Потоки (1–5): папка — файлы параллельно, файл — пакеты строк\n\
4. «Извлечь» внизу → файлы рядом с каждым дампом\n\
5. Папка: каждый .sql обрабатывается отдельно",
    sql_instr_extracted: "Что извлекается",
    sql_instr_extracted_mono: "email:md5   — 32 hex-символа\n\
email:sha1  — 40 hex-символов\n\
email + 'md5' в одной SQL-строке тоже распознаётся",
    sql_instr_outputs: "Выходные файлы",
    sql_instr_outputs_mono: "{name}_emails.txt — email:hash\n{name}_trash.txt  — строки без совпадения",
    sql_heading: "Извлечение email:md5 / email:sha1",
    sql_subtitle: "Regex: email:hash или email + 'md5' в одной SQL-строке",
    sqlcol_intro: "Ищет таблицы с парами login + password, даже если колонки не рядом в INSERT",
    sqlcol_instr_body: "1. Выберите .sql дамп или папку с дампами\n\
2. Парсер читает CREATE TABLE — находит колонки login + password\n\
3. В INSERT берёт значения по имени колонки (не обязательно рядом)\n\
4. «Извлечь» внизу → {name}_loginpass.txt рядом с дампом\n\
5. Пустые / NULL пропускаются",
    sqlcol_login_cols: "Колонки логина",
    sqlcol_login_cols_mono: "username, users, user, login, email, emal, mail, mal,\ncustomer, customers, member, nickname",
    sqlcol_pass_cols: "Колонки пароля",
    sqlcol_pass_cols_mono: "password, passwords, pass, pwd",
    sqlcol_example: "Пример",
    sqlcol_example_mono: "INSERT (id, password, username) VALUES (1,'secret','a@b.com')\n→ a@b.com:secret",
    regex_intro: "Шаблон вывода: $0 — match, $1 $2 — группы, ${name} — именованная, $$ — $",
    regex_instr_body: "1. Выберите файл (txt, sql, log, csv) — «Обзор…»\n\
2. Задайте regex с группами захвата ( ) — или возьмите пресет\n\
3. Шаблон вывода собирает строку из групп: $1:$2\n\
4. «Извлечь» внизу → рядом появится {name}_regex.txt",
    regex_template_heading: "Шаблон вывода",
    regex_template_mono: "$0 — всё совпадение\n\
$1, $2, … — группы по номеру (скобки в regex)\n\
${email} — именованная группа (?P<email>…)\n\
$$ — буквальный символ $",
    regex_flags_heading: "Флаги",
    regex_flags_mono: "i — без учёта регистра\n\
m — ^ и $ на каждой строке\n\
s — точка . захватывает перевод строки\n\
dedupe — не писать повторяющиеся строки",
    regex_examples_heading: "Примеры",
    regex_examples_mono: "email:md5 из дампа:\n\
regex:  ([\\w.+-]+@[\\w.-]+):([a-f0-9]{32})\n\
шаблон: $1:$2\n\
\n\
hash:pass из _good:\n\
regex:  ([a-f0-9]{32}):(\\S+)\n\
шаблон: $1:$2\n\
\n\
только email:\n\
regex:  (?i)([\\w.+-]+@[\\w.-]+\\.[a-z]{2,})\n\
шаблон: $1",
    regex_cli_heading: "CLI (без GUI)",
    regex_cli_mono: "LocalHashFinder extract-regex dump.sql \\\n\
  --pattern \"(?i)([^@]+@[^:]+):([a-f0-9]{32})\" \\\n\
  --template \"$1:$2\"",
    regex_pattern_label: "Pattern (Rust regex syntax)",
    regex_output_template_label: "Output template",
    regex_flag_i: "i — case insensitive",
    regex_flag_m: "m — multiline",
    regex_flag_s: "s — dot matches newline",
    regex_flag_dedupe: "dedupe",
    section_regex_engine: "Regex engine",
    section_sql: "SQL",
    section_sql_columns: "SQL columns",
    no_result_files: "нет файлов результатов",
};

static EN: I18n = I18n {
    app_subtitle: "Your database only · no licenses or servers",
    browse: "Browse…",
    file_btn: "File…",
    folder_btn: "Folder…",
    folder_hint_dumps: "Folder: all .sql / .txt / .dump",
    instruction: "Instructions",
    btn_instruction: "Instruction",
    btn_close: "Close",
    lang_label: "Language",
    zoom_label: "Zoom",
    zoom_out_tip: "Zoom out (Ctrl + scroll)",
    zoom_in_tip: "Zoom in (Ctrl + scroll)",
    tab_lookup: "Hash Lookup",
    tab_merge: "Merge",
    tab_sql: "SQL Extract",
    tab_columns: "SQL Columns",
    tab_regex: "Custom Regex",
    tab_combo: "ComboKit",
    combo_tool_compare: "Compare lists",
    combo_tool_filter: "Combo filter",
    combo_tool_email: "Email filter",
    combo_tool_namepw: "Name / Password split",
    combo_tool_mx: "MX domain check",
    combo_tool_scraper: "Scraper (txt/sql/json)",
    combo_tool_analyze: "Provider analyze",
    combo_tool_dedupe: "Dedupe lines",
    combo_tool_line_filter: "Filter lines",
    combo_tool_merge: "Merge lines",
    combo_tool_split: "Split lines",
    section_combo: "ComboKit",
    heading_combo: "ComboKit tools for combo lists",
    hint_combo_input_b: "Second file (compare)",
    hint_combo_output_dir: "output/",
    hint_combo_input: "input.txt",
    hint_combo_output: "output.txt",
    hint_combo_output_idle: "→ file or folder depending on the selected tool",
    hint_combo_filter: "Filter (contains / regex):",
    hint_combo_lines: "Lines per file:",
    label_combo_tool: "Tool",
    combo_subtitle: "Filter, dedupe, split/merge, scraper, MX, compare — no server",
    combo_instr_body: "1. Choose a ComboKit tool from the list\n\
2. Set input .txt (Compare needs two files)\n\
3. Set output file or folder (see table below)\n\
4. Extract at the bottom → log and Results open the folder\n\
5. LMDB database is not required — everything runs locally",
    combo_instr_tools: "Tools",
    combo_instr_tools_mono: "Compare       — diff two lists (only_a / only_b / both)\n\
Combo filter  — validate email:pass, drop junk\n\
Email filter  — keep lines with email only\n\
Name/Pass     — split into names.txt + passwords.txt\n\
MX check      — verify domain MX records\n\
Scraper       — extract combos from .txt / .sql / .json\n\
Analyze       — group by domain (by_domain/)\n\
Dedupe        — unique lines\n\
Line filter   — contains or regex\n\
Merge / Split — join or split file by N lines",
    combo_instr_formats: "Input formats",
    combo_instr_formats_mono: "combo:   user@gmail.com:password123\n\
lines:   one record per line\n\
scraper: .txt / .sql / .json — email:pass from dumps and logs",
    combo_instr_outputs: "Output files",
    combo_instr_outputs_mono: "Compare / MX / Analyze / Name-Pass → folder:\n\
  only_a.txt, only_b.txt, both.txt · mx_*.txt · by_domain/\n\
  names.txt + passwords.txt\n\
Other tools → single .txt (output.txt)",
    combo_instr_controls: "Controls",
    combo_instr_controls_body: "Extract / Stop / Pause — Controls panel at the bottom.\n\
Results — open the folder with the latest output.\n\
To archive / Merge to one file — when multiple outputs exist.",
    err_combo_output: "Specify output file",
    err_combo_output_dir: "Specify output folder",
    err_combo_input_b: "Specify second file (compare)",
    tab_ulp: "ULP",
    ulp_tool_sort: "Sort — bucket by type",
    ulp_tool_sort_country: "Sort — by country/TLD",
    ulp_tool_sort_keyword: "Sort — by keyword (folders)",
    ulp_tool_search: "Search — keyword → single file",
    ulp_tool_extract_ulp: "Extract url:login:pass",
    ulp_tool_extract_lp: "Extract login:pass",
    ulp_tool_extract_up: "Extract user:pass",
    ulp_tool_clean_dedupe: "Clean — dedupe lines",
    ulp_tool_clean_empty: "Clean — empty lines",
    ulp_tool_clean_junk: "Clean — junk chars",
    ulp_tool_clean_blacklist: "Clean — blacklist domains",
    ulp_tool_clean_chars: "Clean — empty chars",
    ulp_tool_clean_weak: "Clean — weak passwords",
    ulp_tool_clean_proto: "Clean — strip protocols",
    ulp_tool_clean_capture: "Clean — capture lines",
    ulp_tool_misc_merge: "Misc — merge files",
    ulp_tool_misc_split: "Misc — split by lines",
    ulp_tool_misc_filter: "Misc — filter by keyword",
    section_ulp: "SwiftyULP",
    heading_ulp: "ULP services — sort / extract / clean",
    ulp_subtitle: "Process url:login:pass locally — no license or server",
    label_ulp_tool: "Service",
    hint_ulp_input: "input.txt, .zip/.7z/.rar or folder",
    hint_ulp_output: "output.txt",
    hint_ulp_output_dir: "output/",
    hint_ulp_keywords: "Keywords (comma-separated):",
    hint_ulp_output_idle: "→ Sort*: folder · Search/Extract/Clean/Misc → single .txt",
    ulp_instr_body: "1. Choose a ULP service (Sort, Search, Extract, Clean, Misc)\n\
2. Set input .txt, archive (.zip/.7z/.rar), or folder\n\
3. Set output file or folder (Sort* → folder)\n\
4. Extract at the bottom — check log and Results",
    ulp_instr_tools: "SwiftyULP services",
    ulp_instr_tools_mono: "Sort      → Mails / Mail Pass / Phone Pass / User Pass / ULP\n\
Sort TLD  → by_tld/ + by_domain/\n\
Sort KW   → one keyword.txt per term\n\
Search    → matched lines → single file\n\
Extract   → url:login:pass | login:pass | user:pass\n\
Clean     → dedupe, empty, junk, blacklist, weak, protocols, capture\n\
Misc      → merge, split, filter",
    ulp_instr_outputs: "Output files",
    ulp_instr_outputs_mono: "Sort/     → output/Mails.txt, Mail Pass.txt, ULP.txt …\n\
Sort TLD/ → by_tld/de.txt, by_domain/gmail.com.txt …\n\
Sort KW/  → gmail.com.txt, paypal.txt …\n\
Extract/  → output.txt (single file)\n\
Clean/    → cleaned.txt",
    ulp_instr_controls: "Controls",
    ulp_instr_controls_body: "Extract / Stop — Controls panel at the bottom.\n\
Keywords — Sort KW, Search, Extract, Misc filter.\n\
Results — open the folder with the latest output.",
    err_ulp_output: "Specify output file",
    err_ulp_output_dir: "Specify output folder",
    badge_ulp: "ULP",
    badge_combo: "Combo",
    badge_lookup: "Lookup",
    badge_merge: "Merge",
    badge_sql: "SQL",
    badge_columns: "Columns",
    badge_regex: "Regex",
    badge_append: "Database",
    badge_idle: "Idle",
    btn_start: "Start",
    btn_merge: "Merge",
    btn_extract: "Extract",
    btn_pause: "Pause",
    btn_resume: "Resume",
    btn_stop: "Stop",
    btn_results: "Results",
    btn_delete: "Delete results",
    btn_zip: "To archive",
    btn_merge_one: "Merge to one file",
    btn_open: "Open",
    btn_add: "Append",
    section_params: "Controls",
    section_log: "Log",
    log_waiting: "Waiting for events…",
    error_prefix: "Error:",
    stat_total: "Total",
    stat_found: "Found",
    stat_not_found: "Not found",
    stat_trash: "Trash",
    section_database: "Database",
    section_append: "Append",
    section_files: "Files",
    section_batch: "Batch",
    section_threads: "Threads",
    section_progress: "Progress",
    section_output: "Output",
    section_source: "Source",
    section_merge: "Merge",
    section_mail: "Mail",
    section_dehash: "Dehash",
    section_preset: "Preset",
    heading_lmdb: "LMDB",
    heading_input_file: "Input file",
    heading_progress: "Progress",
    heading_result: "Result",
    heading_file_or_folder: "File or folder",
    heading_parallelism: "Parallelism",
    heading_email_hash: "email:hash",
    heading_mail_plain: "hash:pass",
    heading_custom_regex: "Custom regex extraction",
    heading_loginpass: "login:password by column names",
    err_lmdb_path: "Specify LMDB path",
    err_no_lmdb_hint: "No LMDB — select database folder and click Open",
    err_save_config: "Failed to save config",
    err_input_file: "Specify input file",
    err_file_not_found: "File not found",
    err_path_not_found: "Path not found",
    err_file_or_folder: "Specify file or folder",
    err_both_merge_files: "Specify both files: mail and dehash",
    err_merge_file_missing: "One of the files was not found",
    err_regex_source: "Specify source file",
    err_regex_pattern: "Specify regex pattern",
    err_regex_template: "Specify output template",
    err_hashpass_file: "Specify hash:pass file",
    err_stop_lookup_before_append: "Stop lookup before appending to the database",
    dialog_lmdb_folder: "LMDB folder (hashdb.lmdb, data, or any folder for a new DB)",
    db_entries: "Database: {n} entries",
    db_pick_lmdb: "Select LMDB folder and click Open",
    db_path_saved_empty: "Path saved. Database is empty — import or Append",
    label_hash_db: "Hash DB:",
    tip_hash_db_path: "Full LMDB path. Select text to copy.",
    log_start: "Start: {path}",
    log_start_threads: "Start: {path} (threads: {threads})",
    log_stop_requested: "Stop requested",
    log_pause: "Paused",
    log_resume: "Resumed",
    log_folder_opened: "Opened folder: {path}",
    log_deleted: "Deleted files: {n}",
    log_delete_err: "Delete error: {e}",
    log_zip: "Archive: {path}",
    log_zip_err: "Archive error: {e}",
    log_merged_one: "Merged into one file: {path}",
    log_merge_err: "Merge error: {e}",
    log_batch_done: "Batch processing finished",
    status_stopping: "Stopping…",
    status_merge_loading: "Loading dehash and merging…",
    status_sql_batch: "Batch folder processing ({threads} threads)…",
    status_sql_parsing: "Parsing SQL (regex, {threads} threads)…",
    status_sqlcol_batch: "Batch folder processing…",
    status_sqlcol_parsing: "Parsing SQL by columns…",
    status_append: "Appending to LMDB…",
    status_regex_extract: "Regex extract…",
    status_waiting_start: "Waiting to start…",
    status_progress_after_start: "Progress and stats appear after Start",
    hint_input_formats: "email:hash · hash · email:hash:extra",
    hint_input_placeholder: "email:hash or hash",
    hint_merge_mail: "user@gmail.com:md5…",
    hint_merge_dehash: "input_good.txt",
    hint_merge_output: "→ *_plain.txt and *_plain_nohash.txt",
    hint_sql_source: "dump.sql or folder with dumps",
    hint_sql_output: "file or folder → {name}_emails.txt and {name}_trash.txt next to each dump",
    hint_sql_threads: "Folder: up to N files at once · single file: regex line batches",
    hint_sqlcol_output: "file or folder → {name}_loginpass.txt next to each dump",
    hint_regex_output: "→ {name}_regex.txt next to source",
    hint_regex_template_label: "Preset:",
    hint_lmdb_path: "D:\\bases\\hashdb.lmdb",
    hint_append_file: "new_hashes.txt",
    lookup_instr_body: "1. Set LMDB folder (hashdb.lmdb) and click Open\n\
2. Choose a .txt with hashes — supported formats below\n\
3. Tune threads (batch size) for large files\n\
4. Start at the bottom → result files appear next to the input\n\
5. Append — add hash:pass to the database without duplicates",
    lookup_instr_formats: "Input formats",
    lookup_instr_formats_mono: "email:hash\nhash\nemail:hash:extra\nuser@gmail.com:5f4dcc3b5aa765d61d8327deb882cf99",
    lookup_instr_outputs: "Output files",
    lookup_instr_outputs_mono: "{name}_good.txt    — hash:pass (found)\n\
{name}_nohash.txt  — hash without match\n\
{name}_bad.txt     — invalid lines\n\
{name}_trash.txt   — junk / extra fields",
    lookup_instr_controls: "Controls",
    lookup_instr_controls_body: "Start / Pause / Stop / Results — Controls panel at the bottom.\n\
Merge to one file — combine _good / _nohash / _bad into one .txt (no _trash).",
    merge_instr_body: "1. Mail — email:hash file (account password md5/sha1 hash)\n\
2. Dehash — _good.txt from lookup (hash:plainpass)\n\
3. Merge at the bottom → joins on matching hash\n\
4. Results opens the output folder",
    merge_instr_formats: "Formats",
    merge_instr_formats_mono: "mail:   user@gmail.com:5f4dcc3b5aa765d61d8327deb882cf99  (md5)\n\
mail:   user@gmail.com:dabd7bfb00119f1ee6baaddbb5e2150308b70599  (sha1)\n\
dehash: 5f4dcc3b5aa765d61d8327deb882cf99:password123",
    merge_instr_outputs: "Output files",
    merge_instr_outputs_mono: "{name}_plain.txt       — email:plainpass (success)\n\
{name}_plain_nohash.txt — email without dehash\n\
{name}_plain_trash.txt  — junk lines",
    merge_subtitle: "mail: md5/sha1 password hash · dehash: _good.txt (hash:pass)",
    merge_heading: "mail:hash + hash:pass → mail:plainpass",
    sql_instr_body: "1. Choose a .sql dump or folder with dumps\n\
2. Regex finds email + md5/sha1 in INSERT and VALUES\n\
3. Threads (1–5): folder — files in parallel, file — line batches\n\
4. Extract at the bottom → files next to each dump\n\
5. Folder: each .sql is processed separately",
    sql_instr_extracted: "What is extracted",
    sql_instr_extracted_mono: "email:md5   — 32 hex chars\n\
email:sha1  — 40 hex chars\n\
email + 'md5' on one SQL line is also recognized",
    sql_instr_outputs: "Output files",
    sql_instr_outputs_mono: "{name}_emails.txt — email:hash\n{name}_trash.txt  — non-matching lines",
    sql_heading: "Extract email:md5 / email:sha1",
    sql_subtitle: "Regex: email:hash or email + 'md5' on one SQL line",
    sqlcol_intro: "Finds tables with login + password pairs even when columns are not adjacent in INSERT",
    sqlcol_instr_body: "1. Choose a .sql dump or folder with dumps\n\
2. Parser reads CREATE TABLE — finds login + password columns\n\
3. INSERT values are taken by column name (not necessarily adjacent)\n\
4. Extract at the bottom → {name}_loginpass.txt next to the dump\n\
5. Empty / NULL values are skipped",
    sqlcol_login_cols: "Login columns",
    sqlcol_login_cols_mono: "username, users, user, login, email, emal, mail, mal,\ncustomer, customers, member, nickname",
    sqlcol_pass_cols: "Password columns",
    sqlcol_pass_cols_mono: "password, passwords, pass, pwd",
    sqlcol_example: "Example",
    sqlcol_example_mono: "INSERT (id, password, username) VALUES (1,'secret','a@b.com')\n→ a@b.com:secret",
    regex_intro: "Output template: $0 — match, $1 $2 — groups, ${name} — named, $$ — $",
    regex_instr_body: "1. Choose a file (txt, sql, log, csv) — Browse…\n\
2. Set regex with capture groups ( ) — or pick a preset\n\
3. Output template builds a line from groups: $1:$2\n\
4. Extract at the bottom → {name}_regex.txt appears next to the source",
    regex_template_heading: "Output template",
    regex_template_mono: "$0 — full match\n\
$1, $2, … — groups by number (parentheses in regex)\n\
${email} — named group (?P<email>…)\n\
$$ — literal $",
    regex_flags_heading: "Flags",
    regex_flags_mono: "i — case insensitive\n\
m — ^ and $ per line\n\
s — dot . matches newline\n\
dedupe — skip duplicate output lines",
    regex_examples_heading: "Examples",
    regex_examples_mono: "email:md5 from dump:\n\
regex:  ([\\w.+-]+@[\\w.-]+):([a-f0-9]{32})\n\
template: $1:$2\n\
\n\
hash:pass from _good:\n\
regex:  ([a-f0-9]{32}):(\\S+)\n\
template: $1:$2\n\
\n\
email only:\n\
regex:  (?i)([\\w.+-]+@[\\w.-]+\\.[a-z]{2,})\n\
template: $1",
    regex_cli_heading: "CLI (no GUI)",
    regex_cli_mono: "LocalHashFinder extract-regex dump.sql \\\n\
  --pattern \"(?i)([^@]+@[^:]+):([a-f0-9]{32})\" \\\n\
  --template \"$1:$2\"",
    regex_pattern_label: "Pattern (Rust regex syntax)",
    regex_output_template_label: "Output template",
    regex_flag_i: "i — case insensitive",
    regex_flag_m: "m — multiline",
    regex_flag_s: "s — dot matches newline",
    regex_flag_dedupe: "dedupe",
    section_regex_engine: "Regex engine",
    section_sql: "SQL",
    section_sql_columns: "SQL columns",
    no_result_files: "no result files",
};

pub fn tr(lang: Lang) -> &'static I18n {
    match lang {
        Lang::Ru => &RU,
        Lang::En => &EN,
    }
}

pub fn db_status_entries(lang: Lang, n: u64) -> String {
    tr(lang).db_entries.replace("{n}", &n.to_string())
}

pub fn log_start_path(lang: Lang, path: &str) -> String {
    tr(lang).log_start.replace("{path}", path)
}

pub fn log_start_threads(lang: Lang, path: &str, threads: u32) -> String {
    tr(lang)
        .log_start_threads
        .replace("{path}", path)
        .replace("{threads}", &threads.to_string())
}

pub fn err_no_lmdb(lang: Lang, path: &str) -> String {
    match lang {
        Lang::Ru => format!(
            "Нет LMDB: {path} — выберите папку базы и нажмите «Открыть»"
        ),
        Lang::En => format!("No LMDB: {path} — select database folder and click Open"),
    }
}

pub fn append_status(lang: Lang, added: u64, skipped: u64, bad: u64, total: u64) -> String {
    match lang {
        Lang::Ru => format!(
            "added={added} skipped={skipped} bad={bad} → итого {total}"
        ),
        Lang::En => format!("added={added} skipped={skipped} bad={bad} → total {total}"),
    }
}

pub fn merge_done_short(lang: Lang, merged: u64, nohash: u64, trash: u64) -> String {
    match lang {
        Lang::Ru => format!("Готово: merged={merged} nohash={nohash} trash={trash}"),
        Lang::En => format!("Done: merged={merged} nohash={nohash} trash={trash}"),
    }
}

pub fn merge_done_full(
    lang: Lang,
    merged: u64,
    nohash: u64,
    bad: u64,
    trash: u64,
    total: u64,
    plain: &str,
    nohash_path: &str,
    trash_path: &str,
) -> String {
    let head = match lang {
        Lang::Ru => format!(
            "Готово: merged={merged} nohash={nohash} bad={bad} trash={trash} total={total}"
        ),
        Lang::En => format!(
            "Done: merged={merged} nohash={nohash} bad={bad} trash={trash} total={total}"
        ),
    };
    format!("{head}\n{plain}\n{nohash_path}\n{trash_path}")
}

pub fn lookup_done_short(lang: Lang, good: u64, nohash: u64, bad: u64, trash: u64) -> String {
    match lang {
        Lang::Ru => format!("Готово: good={good} nohash={nohash} bad={bad} trash={trash}"),
        Lang::En => format!("Done: good={good} nohash={nohash} bad={bad} trash={trash}"),
    }
}

pub fn lookup_done_full(
    lang: Lang,
    good: u64,
    nohash: u64,
    bad: u64,
    trash: u64,
    good_path: &str,
    nohash_path: &str,
    trash_path: &str,
) -> String {
    let head = lookup_done_short(lang, good, nohash, bad, trash);
    format!("{head}\n{good_path}\n{nohash_path}\n{trash_path}")
}

pub fn sql_done_short(lang: Lang, total: u64, trash: u64) -> String {
    match lang {
        Lang::Ru => format!("Готово: {total} email:hash, trash={trash}"),
        Lang::En => format!("Done: {total} email:hash, trash={trash}"),
    }
}

pub fn sql_done_full(
    lang: Lang,
    total: u64,
    md5: u64,
    sha1: u64,
    trash: u64,
    lines: u64,
    output: &str,
    trash_path: &str,
) -> String {
    match lang {
        Lang::Ru => format!(
            "Готово: {total} email:hash (md5={md5}, sha1={sha1}), trash={trash}, scanned {lines} lines\n{output}\n{trash_path}"
        ),
        Lang::En => format!(
            "Done: {total} email:hash (md5={md5}, sha1={sha1}), trash={trash}, scanned {lines} lines\n{output}\n{trash_path}"
        ),
    }
}

pub fn regex_done_log(lang: Lang, written: u64) -> String {
    match lang {
        Lang::Ru => format!("Готово: written={written}"),
        Lang::En => format!("Done: written={written}"),
    }
}

pub fn regex_done_full(
    lang: Lang,
    written: u64,
    matches: u64,
    dup: u64,
    empty_skip: u64,
    lines: u64,
    output: &str,
) -> String {
    match lang {
        Lang::Ru => format!(
            "Готово: written={written} matches={matches} dup={dup} empty_skip={empty_skip} lines={lines}\n{output}"
        ),
        Lang::En => format!(
            "Done: written={written} matches={matches} dup={dup} empty_skip={empty_skip} lines={lines}\n{output}"
        ),
    }
}

pub fn sqlcol_done_log(lang: Lang, written: u64) -> String {
    match lang {
        Lang::Ru => format!("Готово: written={written}"),
        Lang::En => format!("Done: written={written}"),
    }
}

pub fn sqlcol_done_full(
    lang: Lang,
    written: u64,
    skipped: u64,
    tables: u64,
    inserts: u64,
    lines: u64,
    output: &str,
) -> String {
    match lang {
        Lang::Ru => format!(
            "Готово: written={written} skipped={skipped} tables={tables} inserts={inserts} lines={lines}\n{output}"
        ),
        Lang::En => format!(
            "Done: written={written} skipped={skipped} tables={tables} inserts={inserts} lines={lines}\n{output}"
        ),
    }
}

pub fn folder_sql_summary(
    lang: Lang,
    root: &str,
    ok: u32,
    err: u32,
    total: u32,
    email_hash: u64,
    md5: u64,
    sha1: u64,
    trash: u64,
    extra_errors: &[String],
) -> String {
    let body = match lang {
        Lang::Ru => format!(
            "Папка: {root}\nФайлов: {ok} ok, {err} err из {total}\nemail:hash={email_hash} (md5={md5}, sha1={sha1}), trash={trash}"
        ),
        Lang::En => format!(
            "Folder: {root}\nFiles: {ok} ok, {err} err of {total}\nemail:hash={email_hash} (md5={md5}, sha1={sha1}), trash={trash}"
        ),
    };
    append_error_tail(lang, body, extra_errors)
}

pub fn folder_columns_summary(
    lang: Lang,
    root: &str,
    ok: u32,
    err: u32,
    total: u32,
    written: u64,
    skipped: u64,
    tables: u64,
    extra_errors: &[String],
) -> String {
    let body = match lang {
        Lang::Ru => format!(
            "Папка: {root}\nФайлов: {ok} ok, {err} err из {total}\nwritten={written} skipped={skipped} tables={tables}"
        ),
        Lang::En => format!(
            "Folder: {root}\nFiles: {ok} ok, {err} err of {total}\nwritten={written} skipped={skipped} tables={tables}"
        ),
    };
    append_error_tail(lang, body, extra_errors)
}

fn append_error_tail(lang: Lang, mut s: String, errors: &[String]) -> String {
    for e in errors.iter().take(5) {
        s.push('\n');
        s.push_str(e);
    }
    if errors.len() > 5 {
        s.push_str(&match lang {
            Lang::Ru => format!("\n…ещё {} ошибок", errors.len() - 5),
            Lang::En => format!("\n…{} more errors", errors.len() - 5),
        });
    }
    format!("{FOLDER_BATCH_PREFIX}{s}")
}

pub fn folder_no_dumps(lang: Lang) -> &'static str {
    match lang {
        Lang::Ru => "В папке нет файлов .sql / .txt / .dump",
        Lang::En => "Folder has no .sql / .txt / .dump files",
    }
}

pub fn stopped_by_user(lang: Lang) -> &'static str {
    match lang {
        Lang::Ru => "остановлено пользователем",
        Lang::En => "stopped by user",
    }
}

pub fn batch_file_line(lang: Lang, index: u32, total: u32, name: &str) -> String {
    match lang {
        Lang::Ru => format!("Файл {index}/{total}: {name}"),
        Lang::En => format!("File {index}/{total}: {name}"),
    }
}

pub fn batch_sql_stats_line(
    lang: Lang,
    ok: u32,
    err: u32,
    total: u64,
    md5: u64,
    sha1: u64,
    trash: u64,
) -> String {
    match lang {
        Lang::Ru => format!(
            "готово: {ok} ok, {err} err  |  email:hash: {total} (md5={md5}, sha1={sha1})  trash={trash}"
        ),
        Lang::En => format!(
            "done: {ok} ok, {err} err  |  email:hash: {total} (md5={md5}, sha1={sha1})  trash={trash}"
        ),
    }
}

pub fn batch_columns_stats_line(
    lang: Lang,
    ok: u32,
    err: u32,
    written: u64,
    skipped: u64,
    tables: u64,
) -> String {
    match lang {
        Lang::Ru => format!(
            "готово: {ok} ok, {err} err  |  login:pass: {written}  skipped={skipped}  tables={tables}"
        ),
        Lang::En => format!(
            "done: {ok} ok, {err} err  |  login:pass: {written}  skipped={skipped}  tables={tables}"
        ),
    }
}

pub fn batch_lines_scanned(lang: Lang, n: u64) -> String {
    match lang {
        Lang::Ru => format!("строк просмотрено: {n}"),
        Lang::En => format!("lines scanned: {n}"),
    }
}

pub fn batch_inserts_lines(lang: Lang, inserts: u64, lines: u64) -> String {
    match lang {
        Lang::Ru => format!("inserts={inserts}  строк: {lines}"),
        Lang::En => format!("inserts={inserts}  lines: {lines}"),
    }
}
