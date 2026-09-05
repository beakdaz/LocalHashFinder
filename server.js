"use strict";

const http = require("http");
const fs = require("fs");
const path = require("path");
const { URL } = require("url");
const { ROOT, db, jobState, processFile } = require("./engine");

const PORT = Number(process.env.LOCAL_HASH_PORT || 8787);
const PUBLIC = path.join(__dirname, "public");

const sseClients = new Set();

function sendSse(data) {
  const payload = `data: ${JSON.stringify(data)}\n\n`;
  for (const res of sseClients) {
    res.write(payload);
  }
}

function json(res, status, body) {
  const text = JSON.stringify(body);
  res.writeHead(status, {
    "Content-Type": "application/json; charset=utf-8",
    "Content-Length": Buffer.byteLength(text),
  });
  res.end(text);
}

function readBody(req) {
  return new Promise((resolve, reject) => {
    const chunks = [];
    req.on("data", (c) => chunks.push(c));
    req.on("end", () => resolve(Buffer.concat(chunks).toString("utf8")));
    req.on("error", reject);
  });
}

function serveStatic(req, res, url) {
  let filePath = path.join(PUBLIC, url.pathname === "/" ? "index.html" : url.pathname);
  filePath = path.normalize(filePath);
  if (!filePath.startsWith(PUBLIC)) {
    res.writeHead(403);
    return res.end("Forbidden");
  }
  if (!fs.existsSync(filePath) || fs.statSync(filePath).isDirectory()) {
    res.writeHead(404);
    return res.end("Not found");
  }
  const ext = path.extname(filePath).toLowerCase();
  const types = {
    ".html": "text/html; charset=utf-8",
    ".css": "text/css; charset=utf-8",
    ".js": "application/javascript; charset=utf-8",
  };
  const data = fs.readFileSync(filePath);
  res.writeHead(200, { "Content-Type": types[ext] || "application/octet-stream" });
  res.end(data);
}

async function handleApi(req, res, url) {
  if (req.method === "GET" && url.pathname === "/api/status") {
    const info = db.load(true);
    return json(res, 200, {
      db_count: info.count,
      db_files: info.files,
      db_path: path.join(ROOT, "local_hash_db.txt"),
      job_running: jobState.running,
    });
  }

  if (req.method === "POST" && url.pathname === "/api/reload-db") {
    const info = db.load(true);
    return json(res, 200, { ok: true, db_count: info.count, db_files: info.files });
  }

  if (req.method === "GET" && url.pathname === "/api/events") {
    res.writeHead(200, {
      "Content-Type": "text/event-stream",
      "Cache-Control": "no-cache",
      Connection: "keep-alive",
    });
    res.write("\n");
    sseClients.add(res);
    req.on("close", () => sseClients.delete(res));
    return;
  }

  if (req.method === "POST" && url.pathname === "/api/stop") {
    jobState.stop = true;
    return json(res, 200, { ok: true });
  }

  if (req.method === "POST" && url.pathname === "/api/process") {
    if (jobState.running) {
      return json(res, 409, { error: "already_running" });
    }

    const body = JSON.parse(await readBody(req));
    const files = Array.isArray(body.files) ? body.files : [];
    const threads = Number(body.threads) || 100;

    if (!files.length) {
      return json(res, 400, { error: "no_files" });
    }

    for (const file of files) {
      if (!fs.existsSync(file)) {
        return json(res, 400, { error: "file_not_found", file });
      }
    }

    jobState.running = true;
    jobState.stop = false;
    jobState.id += 1;
    const jobId = jobState.id;

    json(res, 200, { ok: true, job_id: jobId, files: files.length });

    (async () => {
      sendSse({ type: "job_start", job_id: jobId, files });
      try {
        for (const file of files) {
          if (jobState.stop || jobState.id !== jobId) break;
          sendSse({ type: "file_start", file });
          await processFile(file, { threads, jobId }, (progress) => {
            sendSse({ type: "progress", job_id: jobId, ...progress });
          });
        }
        sendSse({
          type: "job_done",
          job_id: jobId,
          stopped: jobState.stop || jobState.id !== jobId,
        });
      } catch (err) {
        sendSse({ type: "job_error", job_id: jobId, error: String(err) });
      } finally {
        jobState.running = false;
      }
    })();
    return;
  }

  json(res, 404, { error: "not_found" });
}

const server = http.createServer(async (req, res) => {
  const url = new URL(req.url, `http://${req.headers.host}`);
  if (url.pathname.startsWith("/api/")) {
    try {
      await handleApi(req, res, url);
    } catch (err) {
      json(res, 500, { error: String(err) });
    }
    return;
  }
  serveStatic(req, res, url);
});

server.listen(PORT, "127.0.0.1", () => {
  const info = db.load(true);
  console.log("Local Hash Finder");
  console.log("UI:      http://127.0.0.1:" + PORT);
  console.log("DB:      " + info.count + " hashes");
  console.log("DB file: " + path.join(ROOT, "local_hash_db.txt"));
});

module.exports = { server, PORT };
