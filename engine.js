"use strict";

const fs = require("fs");
const path = require("path");
const readline = require("readline");
const { createLocalHashDb, HASH_RE } = require("../Tools/local_hash_db");

const ROOT = path.resolve(__dirname, "..");
const db = createLocalHashDb(ROOT);

const jobState = {
  running: false,
  stop: false,
  id: 0,
};

function parseInputLine(line) {
  const raw = line.trim();
  if (!raw || raw.startsWith("#") || raw.startsWith(";")) return null;

  if (HASH_RE.test(raw)) {
    return { raw, hash: raw.toLowerCase(), prefix: "" };
  }

  const colon = raw.lastIndexOf(":");
  if (colon > 0) {
    const tail = raw.slice(colon + 1).trim().toLowerCase();
    if (HASH_RE.test(tail)) {
      return { raw, hash: tail, prefix: raw.slice(0, colon + 1) };
    }
  }

  for (const part of raw.split(/[\s|,;|]+/)) {
    const p = part.trim().toLowerCase();
    if (HASH_RE.test(p)) {
      return { raw, hash: p, prefix: raw };
    }
  }

  return { raw, hash: "", bad: true };
}

async function countLines(filePath) {
  let total = 0;
  const rl = readline.createInterface({
    input: fs.createReadStream(filePath, { encoding: "utf8" }),
    crlfDelay: Infinity,
  });
  for await (const line of rl) {
    if (line.trim()) total += 1;
  }
  return total;
}

function outputPaths(inputPath) {
  const dir = path.dirname(inputPath);
  const base = path.basename(inputPath, path.extname(inputPath));
  return {
    good: path.join(dir, `${base}_good.txt`),
    nohash: path.join(dir, `${base}_nohash.txt`),
    bad: path.join(dir, `${base}_bad.txt`),
  };
}

async function processFile(filePath, options, onProgress) {
  const batchSize = Math.max(1, Math.min(5000, Number(options.threads) || 100));
  const jobId = options.jobId;
  db.load(true);

  const out = outputPaths(filePath);
  fs.writeFileSync(out.good, "");
  fs.writeFileSync(out.nohash, "");
  fs.writeFileSync(out.bad, "");

  const total = await countLines(filePath);
  let processed = 0;
  let found = 0;
  let nohash = 0;
  let bad = 0;
  const started = Date.now();

  const rl = readline.createInterface({
    input: fs.createReadStream(filePath, { encoding: "utf8" }),
    crlfDelay: Infinity,
  });

  let batch = [];

  function appendLines(target, lines) {
    if (!lines.length) return;
    fs.appendFileSync(target, lines.join("\n") + "\n");
  }

  async function flushBatch() {
    if (!batch.length) return true;
    const chunk = batch;
    batch = [];

    const goodLines = [];
    const nohashLines = [];
    const badLines = [];

    for (const row of chunk) {
      if (jobState.stop || jobState.id !== jobId) return false;
      if (row.bad || !row.hash) {
        bad += 1;
        badLines.push(row.raw);
        continue;
      }
      const pass = db.lookup(row.hash);
      if (pass) {
        found += 1;
        goodLines.push(row.prefix ? `${row.prefix}${pass}` : `${row.hash}:${pass}`);
      } else {
        nohash += 1;
        nohashLines.push(`${row.raw}:NULL`);
      }
    }

    appendLines(out.good, goodLines);
    appendLines(out.nohash, nohashLines);
    appendLines(out.bad, badLines);
    processed += chunk.length;

    onProgress({
      processed,
      total,
      found,
      nohash,
      bad,
      elapsed: Date.now() - started,
      file: path.basename(filePath),
    });
    return true;
  }

  for await (const line of rl) {
    if (jobState.stop || jobState.id !== jobId) break;
    const row = parseInputLine(line);
    if (!row) continue;
    batch.push(row);
    if (batch.length >= batchSize) {
      const ok = await flushBatch();
      if (!ok) break;
    }
  }

  if (!jobState.stop && jobState.id === jobId) {
    await flushBatch();
  }

  const result = {
    processed,
    total,
    found,
    nohash,
    bad,
    outputs: out,
    elapsed: Date.now() - started,
    stopped: jobState.stop || jobState.id !== jobId,
  };

  onProgress({ ...result, file: path.basename(filePath), done: true });
  return result;
}

module.exports = { ROOT, db, jobState, parseInputLine, processFile, outputPaths };
