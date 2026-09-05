use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};
use heed::types::Bytes;
use heed::{Env, EnvOpenOptions, PutFlags, RoTxn, RwTxn};
use parking_lot::RwLock;

use crate::parser::{hash_to_key, parse_db_line};

const BATCH: usize = 100_000;

type HashDatabase = heed::Database<Bytes, Bytes>;

pub(crate) struct DbInner {
    pub(crate) env: Env,
    count: u64,
}

#[derive(Clone, Default, Debug)]
pub struct AppendStats {
    pub added: u64,
    pub skipped: u64,
    pub bad_lines: u64,
    pub final_count: u64,
}

pub struct HashDb {
    lmdb_path: RwLock<PathBuf>,
    pub(crate) inner: RwLock<Option<DbInner>>,
    source_files: RwLock<Vec<String>>,
}

impl HashDb {
    pub fn new(lmdb_path: PathBuf) -> Arc<Self> {
        Arc::new(Self {
            lmdb_path: RwLock::new(lmdb_path),
            inner: RwLock::new(None),
            source_files: RwLock::new(Vec::new()),
        })
    }

    pub fn lmdb_path(&self) -> PathBuf {
        self.lmdb_path.read().clone()
    }

    pub fn set_lmdb_path(&self, path: PathBuf) {
        *self.inner.write() = None;
        *self.lmdb_path.write() = path;
    }

    pub fn count(&self) -> u64 {
        self.inner
            .read()
            .as_ref()
            .map(|i| i.count)
            .unwrap_or(0)
    }

    pub fn is_open(&self) -> bool {
        self.inner.read().is_some()
    }

    /// Clone LMDB env handle (cheap) for read-only lookups without holding `inner` lock.
    pub fn env(&self) -> Option<Env> {
        self.inner.read().as_ref().map(|i| i.env.clone())
    }

    fn map_size_bytes(requested_gb: u64) -> usize {
        let requested = (requested_gb as usize).saturating_mul(1024 * 1024 * 1024);
        let min = 32 * 1024 * 1024 * 1024;
        requested.max(min) + 80 * 1024 * 1024 * 1024
    }

    fn open_env(map_size_gb: u64, path: &Path) -> Result<Env> {
        unsafe {
            EnvOpenOptions::new()
                .map_size(Self::map_size_bytes(map_size_gb))
                .max_dbs(1)
                .open(path)
        }
        .context("open LMDB — close LocalHashFinder first")
    }

    pub fn open_db(env: &Env, txn: &RoTxn) -> Result<HashDatabase> {
        env.open_database(txn, Some("hashes"))
            .context("open db handle")?
            .ok_or_else(|| anyhow::anyhow!("database missing"))
    }

    pub fn open_db_handle(&self, env: &Env, txn: &RoTxn) -> Result<HashDatabase> {
        Self::open_db(env, txn)
    }

    /// Create `data/` parent and an empty LMDB env if the path does not exist yet.
    fn ensure_lmdb_exists(lmdb_path: &Path) -> Result<()> {
        if let Some(parent) = lmdb_path.parent() {
            fs::create_dir_all(parent)?;
        }
        if lmdb_path.is_dir() {
            return Ok(());
        }
        fs::create_dir_all(lmdb_path)?;
        let env = Self::open_env(280, lmdb_path)?;
        let mut txn = env.write_txn()?;
        let _: HashDatabase = env.create_database(&mut txn, Some("hashes"))?;
        txn.commit()?;
        Ok(())
    }

    pub fn open_existing(&self) -> Result<u64> {
        let lmdb_path = self.lmdb_path();
        Self::ensure_lmdb_exists(&lmdb_path)?;

        let env = Self::open_env(280, &lmdb_path)?;
        let txn = env.read_txn()?;
        let db = Self::open_db(&env, &txn)?;
        let count = db.len(&txn)? as u64;
        drop(txn);

        *self.inner.write() = Some(DbInner { env, count });
        Ok(count)
    }

    pub fn lookup_in_txn<'a>(
        db: &HashDatabase,
        txn: &RoTxn<'a>,
        hex_hash: &str,
    ) -> Result<Option<String>> {
        let key = hash_to_key(hex_hash)?;
        Ok(db
            .get(txn, &key)?
            .map(|v: &[u8]| String::from_utf8_lossy(v).into_owned()))
    }

    pub fn with_read<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Env, &HashDatabase, &RoTxn) -> Result<T>,
    {
        let guard = self.inner.read();
        let inner = guard
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("LMDB not open — run import first"))?;
        let txn = inner.env.read_txn()?;
        let db = Self::open_db(&inner.env, &txn)?;
        f(&inner.env, &db, &txn)
    }

    pub fn lookup(&self, hex_hash: &str) -> Result<Option<String>> {
        if self.inner.read().is_none() {
            return Ok(None);
        }
        self.with_read(|_, db, txn| Self::lookup_in_txn(db, txn, hex_hash))
    }

    fn flush_batch(
        db: &HashDatabase,
        txn: &mut RwTxn,
        batch: &mut Vec<([u8; 16], Vec<u8>)>,
        skip_existing: bool,
        added: &mut u64,
        skipped: &mut u64,
    ) -> Result<()> {
        use heed::Error as HeedError;

        for (k, v) in batch.drain(..) {
            if skip_existing {
                match db.put_with_flags(txn, PutFlags::NO_OVERWRITE, &k, &v) {
                    Ok(()) => *added += 1,
                    Err(HeedError::Mdb(heed::MdbError::KeyExist)) => *skipped += 1,
                    Err(e) => return Err(e.into()),
                }
            } else {
                db.put(txn, &k, &v)?;
                *added += 1;
            }
        }
        Ok(())
    }

    fn ingest_sources<P: AsRef<Path>>(
        db: &HashDatabase,
        txn: &mut RwTxn,
        sources: &[P],
        skip_existing: bool,
    ) -> Result<(u64, u64, u64)> {
        let mut added: u64 = 0;
        let mut skipped: u64 = 0;
        let mut bad_lines: u64 = 0;

        for src in sources {
            let path = src.as_ref();
            if !path.is_file() {
                continue;
            }
            tracing::info!("ingesting {}", path.display());

            let file = fs::File::open(path)?;
            let reader = std::io::BufReader::with_capacity(16 * 1024 * 1024, file);
            let mut batch: Vec<([u8; 16], Vec<u8>)> = Vec::with_capacity(BATCH);

            for line in reader.lines() {
                let line = line?;
                let Some((hash, pass)) = parse_db_line(&line) else {
                    if !line.trim().is_empty() {
                        bad_lines += 1;
                    }
                    continue;
                };
                let key = hash_to_key(&hash)?;
                batch.push((key, pass.into_bytes()));
                if batch.len() >= BATCH {
                    Self::flush_batch(db, txn, &mut batch, skip_existing, &mut added, &mut skipped)?;
                    let processed = added + skipped;
                    if processed.is_multiple_of(10_000_000) {
                        tracing::info!(
                            "ingested {} M (added={}, skipped={})...",
                            processed / 1_000_000,
                            added,
                            skipped
                        );
                    }
                }
            }
            Self::flush_batch(db, txn, &mut batch, skip_existing, &mut added, &mut skipped)?;
        }

        Ok((added, skipped, bad_lines))
    }

    pub fn import<P: AsRef<Path>>(&self, sources: &[P], map_size_gb: u64) -> Result<u64> {
        *self.inner.write() = None;

        let lmdb_path = self.lmdb_path();
        if lmdb_path.exists() {
            fs::remove_dir_all(&lmdb_path)
                .context("remove old LMDB — close LocalHashFinder first")?;
        }
        fs::create_dir_all(&lmdb_path)?;

        let env = Self::open_env(map_size_gb, &lmdb_path)?;
        let mut txn = env.write_txn()?;
        let db: HashDatabase = env.create_database(&mut txn, Some("hashes"))?;

        let mut names = Vec::new();
        for src in sources {
            let path = src.as_ref();
            if path.is_file() {
                names.push(path.file_name().unwrap().to_string_lossy().into_owned());
            }
        }

        let (added, _, _) = Self::ingest_sources(&db, &mut txn, sources, false)?;
        txn.commit()?;

        *self.source_files.write() = names;
        *self.inner.write() = Some(DbInner {
            env,
            count: added,
        });
        Ok(added)
    }

    /// Append new hash:pass entries; skip keys that already exist.
    pub fn append<P: AsRef<Path>>(&self, sources: &[P], map_size_gb: u64) -> Result<AppendStats> {
        *self.inner.write() = None;

        let lmdb_path = self.lmdb_path();
        let exists = lmdb_path.is_dir();
        if !exists {
            fs::create_dir_all(&lmdb_path)?;
        }

        let env = Self::open_env(map_size_gb, &lmdb_path)?;

        let db = if exists {
            let rtxn = env.read_txn()?;
            let db = Self::open_db(&env, &rtxn)?;
            rtxn.commit().context("commit read txn after opening database handle")?;
            db
        } else {
            let mut txn = env.write_txn()?;
            let db = env.create_database(&mut txn, Some("hashes"))?;
            txn.commit()?;
            db
        };

        let mut txn = env.write_txn()?;
        let (added, skipped, bad_lines) = Self::ingest_sources(&db, &mut txn, sources, true)?;
        txn.commit()?;

        let rtxn = env.read_txn()?;
        let final_count = db.len(&rtxn)? as u64;
        drop(rtxn);

        *self.inner.write() = Some(DbInner {
            env,
            count: final_count,
        });

        Ok(AppendStats {
            added,
            skipped,
            bad_lines,
            final_count,
        })
    }

    pub fn list_source_files(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();
        let lmdb = self.lmdb_path();
        let root: PathBuf = lmdb
            .parent()
            .and_then(|p| p.parent())
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));
        let main = root.join("local_hash_db.txt");
        if main.is_file() {
            files.push(main);
        }
        let dir = root.join("local_hash_db");
        if dir.is_dir() {
            let mut entries: Vec<_> = fs::read_dir(&dir)
                .into_iter()
                .flatten()
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.is_file())
                .collect();
            entries.sort();
            files.extend(entries);
        }
        files
    }
}
