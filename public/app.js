"use strict";

const $ = (id) => document.getElementById(id);

const dropzone = $("dropzone");
const fileInput = $("fileInput");
const pickBtn = $("pickBtn");
const fileList = $("fileList");
const threadsInput = $("threads");
const startBtn = $("startBtn");
const stopBtn = $("stopBtn");
const reloadDbBtn = $("reloadDbBtn");
const dbStatus = $("dbStatus");
const statTotal = $("statTotal");
const statFound = $("statFound");
const statNohash = $("statNohash");
const statBad = $("statBad");
const progressBar = $("progressBar");
const progressText = $("progressText");
const speedText = $("speedText");
const logEl = $("log");

let selectedFiles = [];
let jobRunning = false;

function appendLog(msg) {
  const line = `[${new Date().toLocaleTimeString("ru-RU")}] ${msg}`;
  logEl.textContent += line + "\n";
  logEl.scrollTop = logEl.scrollHeight;
}

function renderFiles() {
  fileList.innerHTML = "";
  for (const f of selectedFiles) {
    const li = document.createElement("li");
    li.textContent = f.name || f.path || String(f);
    fileList.appendChild(li);
  }
}

function setJobUi(running) {
  jobRunning = running;
  startBtn.disabled = running;
  stopBtn.disabled = !running;
  pickBtn.disabled = running;
}

async function refreshStatus() {
  try {
    const res = await fetch("/api/status");
    const data = await res.json();
    dbStatus.textContent = `DB: ${data.db_count.toLocaleString("ru-RU")} хешей`;
    if (data.job_running !== jobRunning) {
      setJobUi(data.job_running);
    }
  } catch (err) {
    dbStatus.textContent = "DB: ошибка";
    appendLog("Не удалось загрузить статус БД: " + err.message);
  }
}

async function reloadDb() {
  reloadDbBtn.disabled = true;
  try {
    const res = await fetch("/api/reload-db", { method: "POST" });
    const data = await res.json();
    dbStatus.textContent = `DB: ${data.db_count.toLocaleString("ru-RU")} хешей`;
    appendLog(`БД обновлена: ${data.db_count} хешей, файлов: ${data.db_files}`);
  } catch (err) {
    appendLog("Ошибка обновления БД: " + err.message);
  } finally {
    reloadDbBtn.disabled = false;
  }
}

function updateProgress(p) {
  if (p.total) {
    const pct = Math.min(100, Math.round((p.processed / p.total) * 100));
    progressBar.style.width = pct + "%";
    progressText.textContent = `${p.processed.toLocaleString("ru-RU")} / ${p.total.toLocaleString("ru-RU")}${p.file ? " · " + p.file : ""}`;
  }
  statTotal.textContent = (p.processed ?? 0).toLocaleString("ru-RU");
  statFound.textContent = (p.found ?? 0).toLocaleString("ru-RU");
  statNohash.textContent = (p.nohash ?? 0).toLocaleString("ru-RU");
  statBad.textContent = (p.bad ?? 0).toLocaleString("ru-RU");

  if (p.elapsed && p.processed) {
    const rate = Math.round((p.processed / p.elapsed) * 1000);
    speedText.textContent = rate.toLocaleString("ru-RU") + " строк/с";
  }
}

async function startJob() {
  if (!selectedFiles.length) {
    appendLog("Выберите хотя бы один файл.");
    return;
  }

  const threads = Number(threadsInput.value) || 200;
  setJobUi(true);
  progressBar.style.width = "0%";
  progressText.textContent = "Запуск…";
  appendLog(`Старт: ${selectedFiles.length} файл(ов), пакет ${threads}`);

  try {
    const res = await fetch("/api/process", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        files: selectedFiles.map((f) => f.path || f.name),
        threads,
      }),
    });
    const data = await res.json();
    if (!res.ok) {
      appendLog("Ошибка запуска: " + (data.error || res.status));
      setJobUi(false);
    }
  } catch (err) {
    appendLog("Ошибка запуска: " + err.message);
    setJobUi(false);
  }
}

async function stopJob() {
  try {
    await fetch("/api/stop", { method: "POST" });
    appendLog("Запрошена остановка…");
  } catch (err) {
    appendLog("Ошибка остановки: " + err.message);
  }
}

function addFiles(fileListLike) {
  for (const f of fileListLike) {
    if (!selectedFiles.some((x) => x.name === f.name && x.size === f.size)) {
      selectedFiles.push(f);
    }
  }
  renderFiles();
}

function connectSse() {
  const es = new EventSource("/api/events");
  es.onmessage = (ev) => {
    let data;
    try {
      data = JSON.parse(ev.data);
    } catch {
      return;
    }

    switch (data.type) {
      case "job_start":
        appendLog("Задача #" + data.job_id + " начата");
        setJobUi(true);
        break;
      case "file_start":
        appendLog("Файл: " + data.file);
        break;
      case "progress":
        updateProgress(data);
        break;
      case "job_done":
        appendLog(data.stopped ? "Задача остановлена." : "Задача завершена.");
        setJobUi(false);
        progressText.textContent = data.stopped ? "Остановлено" : "Готово";
        refreshStatus();
        break;
      case "job_error":
        appendLog("Ошибка: " + data.error);
        setJobUi(false);
        break;
    }
  };
  es.onerror = () => {
    appendLog("SSE: переподключение…");
  };
}

pickBtn.addEventListener("click", () => fileInput.click());

fileInput.addEventListener("change", () => {
  addFiles(fileInput.files);
  fileInput.value = "";
});

dropzone.addEventListener("dragover", (e) => {
  e.preventDefault();
  dropzone.classList.add("dragover");
});

dropzone.addEventListener("dragleave", () => {
  dropzone.classList.remove("dragover");
});

dropzone.addEventListener("drop", (e) => {
  e.preventDefault();
  dropzone.classList.remove("dragover");
  addFiles(e.dataTransfer.files);
});

startBtn.addEventListener("click", startJob);
stopBtn.addEventListener("click", stopJob);
reloadDbBtn.addEventListener("click", reloadDb);

refreshStatus();
connectSse();
appendLog("Интерфейс готов.");
