use std::time::Instant;

use super::filter::GarbageKind;

/// Counters for merge/clean/dedupe run.
#[derive(Debug)]
pub struct MergeStats {
    pub min_len: usize,
    pub files_processed: u64,
    pub lines_read: u64,
    pub lines_kept: u64,
    pub lines_written: u64,
    pub empty_trash: u64,
    pub short_trash: u64,
    pub null_trash: u64,
    pub hash_trash: u64,
    pub combo_trash: u64,
    pub comment_trash: u64,
    pub special_trash: u64,
    pub duplicates: u64,
    started: Instant,
    last_report: Instant,
}

impl Default for MergeStats {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            min_len: 3,
            files_processed: 0,
            lines_read: 0,
            lines_kept: 0,
            lines_written: 0,
            empty_trash: 0,
            short_trash: 0,
            null_trash: 0,
            hash_trash: 0,
            combo_trash: 0,
            comment_trash: 0,
            special_trash: 0,
            duplicates: 0,
            started: now,
            last_report: now,
        }
    }
}

impl MergeStats {
    pub fn new(min_len: usize, files_processed: u64) -> Self {
        Self {
            min_len,
            files_processed,
            ..Self::default()
        }
    }

    pub fn record_garbage(&mut self, kind: GarbageKind) {
        match kind {
            GarbageKind::Empty => self.empty_trash += 1,
            GarbageKind::Short => self.short_trash += 1,
            GarbageKind::Null => self.null_trash += 1,
            GarbageKind::Hash => self.hash_trash += 1,
            GarbageKind::Combo => self.combo_trash += 1,
            GarbageKind::Comment => self.comment_trash += 1,
            GarbageKind::SpecialOnly => self.special_trash += 1,
        }
    }

    pub fn report_progress(&mut self, force: bool) {
        if !force && self.last_report.elapsed().as_secs() < 2 {
            return;
        }
        self.last_report = Instant::now();
        eprintln!(
            "[TextMerger] read={} kept={} | trash: empty={} short={} null={} hash={} combo={} comment={} special={} | dupes={}",
            self.lines_read,
            self.lines_kept,
            self.empty_trash,
            self.short_trash,
            self.null_trash,
            self.hash_trash,
            self.combo_trash,
            self.comment_trash,
            self.special_trash,
            self.duplicates,
        );
    }

    pub fn print_final(&self, output: &str) {
        let secs = self.started.elapsed().as_secs_f64();
        eprintln!();
        eprintln!("=== TextMerger done ({secs:.1}s) ===");
        eprintln!("  files processed: {}", self.files_processed);
        eprintln!("  lines read:      {}", self.lines_read);
        eprintln!("  lines written:   {}", self.lines_written);
        eprintln!("  empty_trash:     {}", self.empty_trash);
        eprintln!("  short_trash:     {}", self.short_trash);
        eprintln!("  null_trash:      {}", self.null_trash);
        eprintln!("  hash_trash:      {}", self.hash_trash);
        eprintln!("  combo_trash:     {}", self.combo_trash);
        eprintln!("  comment_trash:   {}", self.comment_trash);
        eprintln!("  special_trash:   {}", self.special_trash);
        eprintln!("  duplicates:      {}", self.duplicates);
        eprintln!("  output:          {output}");
    }
}
