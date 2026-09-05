use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;

use super::cred::{is_email_login, match_phone_pass, Cred};

const SHARD_COUNT: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    Mails,
    MailPass,
    PhonePass,
    UserPass,
    Ulp,
}

struct BucketShard {
    mails: HashSet<String>,
    mail_pass: HashSet<String>,
    phone_pass: HashSet<String>,
    user_pass: HashSet<String>,
    ulp: HashSet<String>,
}

impl Default for BucketShard {
    fn default() -> Self {
        Self {
            mails: HashSet::new(),
            mail_pass: HashSet::new(),
            phone_pass: HashSet::new(),
            user_pass: HashSet::new(),
            ulp: HashSet::new(),
        }
    }
}

pub struct Buckets {
    shards: [Mutex<BucketShard>; SHARD_COUNT],
}

impl Default for Buckets {
    fn default() -> Self {
        Self::new()
    }
}

impl Buckets {
    pub fn new() -> Self {
        Self {
            shards: std::array::from_fn(|_| Mutex::new(BucketShard::default())),
        }
    }

    fn shard_index(s: &str) -> usize {
        let mut h = std::collections::hash_map::DefaultHasher::new();
        s.hash(&mut h);
        (h.finish() as usize) % SHARD_COUNT
    }

    fn add(&self, cat: Category, key: &str) -> bool {
        if key.is_empty() {
            return false;
        }
        let idx = Self::shard_index(key);
        let mut sh = self.shards[idx].lock().unwrap();
        let set = match cat {
            Category::Mails => &mut sh.mails,
            Category::MailPass => &mut sh.mail_pass,
            Category::PhonePass => &mut sh.phone_pass,
            Category::UserPass => &mut sh.user_pass,
            Category::Ulp => &mut sh.ulp,
        };
        set.insert(key.to_string())
    }

    pub fn ingest_cred(&self, c: &Cred) -> u32 {
        let login = c.login.trim();
        let pass = c.pass.trim();
        if login.is_empty() || pass.is_empty() {
            return 0;
        }
        let mut added = 0u32;
        let line = c.line();
        if is_email_login(login) {
            if self.add(Category::MailPass, &line) {
                added += 1;
            }
            if self.add(Category::Mails, login) {
                added += 1;
            }
            return added;
        }
        if match_phone_pass(&line) {
            if self.add(Category::PhonePass, &line) {
                added += 1;
            }
            return added;
        }
        if login.contains('@') {
            if self.add(Category::Mails, login) {
                added += 1;
            }
        }
        if !c.url.trim().is_empty() {
            if self.add(Category::Ulp, &line) {
                added += 1;
            }
            if self.add(Category::UserPass, &line) {
                added += 1;
            }
            return added;
        }
        if self.add(Category::UserPass, &format!("{login}:{pass}")) {
            added += 1;
        }
        added
    }

    pub fn ingest_line(&self, line: &str) -> u32 {
        super::cred::parse_line(line)
            .map(|c| self.ingest_cred(&c))
            .unwrap_or(0)
    }

    pub fn counts(&self) -> (usize, usize, usize, usize, usize) {
        let mut mails = 0;
        let mut mail_pass = 0;
        let mut phone = 0;
        let mut user = 0;
        let mut ulp = 0;
        for shard in &self.shards {
            let sh = shard.lock().unwrap();
            mails += sh.mails.len();
            mail_pass += sh.mail_pass.len();
            phone += sh.phone_pass.len();
            user += sh.user_pass.len();
            ulp += sh.ulp.len();
        }
        (mails, mail_pass, phone, user, ulp)
    }

    pub fn each<F: FnMut(&str, &str)>(&self, mut f: F) {
        for shard in &self.shards {
            let sh = shard.lock().unwrap();
            for k in &sh.mails {
                f("Mails.txt", k);
            }
            for k in &sh.mail_pass {
                f("Mail Pass.txt", k);
            }
            for k in &sh.phone_pass {
                f("Phone Pass.txt", k);
            }
            for k in &sh.user_pass {
                f("User Pass.txt", k);
            }
            for k in &sh.ulp {
                f("ULP.txt", k);
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SortStats {
    pub files: usize,
    pub mail: usize,
    pub mail_pass: usize,
    pub phone_pass: usize,
    pub user_pass: usize,
    pub ulp_lines: usize,
    pub lines_read: u64,
    pub output_dir: String,
    pub message: String,
}

impl SortStats {
    pub fn from_buckets(b: &Buckets, files: usize, out_dir: &str) -> Self {
        let (m, mp, ph, u, ulp) = b.counts();
        let message = format!(
            "Sort: Mails={m} | Mail Pass={mp} | Phone Pass={ph} | User Pass={u} | ULP={ulp} → {out_dir}"
        );
        Self {
            files,
            mail: m,
            mail_pass: mp,
            phone_pass: ph,
            user_pass: u,
            ulp_lines: ulp,
            lines_read: 0,
            output_dir: out_dir.to_string(),
            message,
        }
    }
}
