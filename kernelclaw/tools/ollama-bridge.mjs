#!/usr/bin/env node
/**
 * ollama-bridge.mjs — Thin Ollama-compatible LLM backend
 *
 * Implements the Ollama /api/chat and /api/generate endpoints,
 * proxying to z-ai-web-dev-sdk for actual LLM completions.
 *
 * Usage:  node ollama-bridge.mjs [--port 11434]
 */

import http from "node:http";
import { URL } from "node:url";
import ZAI from "/home/z/.bun/install/global/node_modules/z-ai-web-dev-sdk/dist/index.js";

// ── Config ────────────────────────────────────────────────────────────
const PORT = parseInt(process.argv.find((a) => a === "--port") ||
  process.argv[process.argv.indexOf("--port") + 1]) || 11434;

// We lazily init so the server can bind even before SDK is ready
let zai = null;

// ── Ollama model registry (virtual) ───────────────────────────────────
const MODELS = {
  "kernelclaw:latest": {
    size: 0,
    family: "kernelclaw",
    parameter_size: "0B",
    quantization_level: "Q4_0",
  },
};

// ── KernelClaw ParsedGoal system prompt ───────────────────────────────
const KERNELCLAW_SYSTEM = `You are a goal parser for KernelClaw, an agent kernel.
Given a natural-language goal, respond with ONLY a valid JSON object (no markdown fences).
Available tools: file_read, file_read_dir, echo, calendar_summary, file_metadata, health_check
Risk levels: low, medium, high
Required capabilities are always ["file_read"].
Expected output type is always "text".
Justification: one sentence explaining why this tool.
Parameters: include "input" (the raw goal text) and, if the tool is file_read or file_read_dir, include "path" with the most likely file/directory path inferred from context. If unknown, use "/tmp/unknown".`;

// ── Helpers ───────────────────────────────────────────────────────────
async function ensureZai() {
  if (!zai) {
    zai = await ZAI.create();
  }
  return zai;
}

/** Parse JSON out of an LLM response (tolerant of markdown fences). */
function extractJSON(raw) {
  let text = raw.trim();
  // Strip ```json ... ``` fences
  const fenceMatch = text.match(/```(?:json)?\s*([\s\S]*?)```/);
  if (fenceMatch) text = fenceMatch[1].trim();
  // Try to find first { ... } block
  const start = text.indexOf("{");
  const end = text.lastIndexOf("}");
  if (start !== -1 && end > start) {
    text = text.slice(start, end + 1);
  }
  return JSON.parse(text);
}

/** Ollama /api/chat handler */
async function handleChat(body) {
  const { model, messages, stream = false } = body;

  // Build messages array for z-ai-web-dev-sdk
  const sdkMessages = messages.map((m) => ({
    role: m.role,
    content: m.content,
  }));

  const zaiInst = await ensureZai();
  const completion = await zaiInst.chat.completions.create({
    messages: sdkMessages,
    temperature: 0.3,
  });

  const content = completion.choices?.[0]?.message?.content || "";

  // Ollama response format
  return {
    model: model || "kernelclaw:latest",
    message: { role: "assistant", content },
    done: true,
    total_duration: 0,
    eval_count: 0,
  };
}

/** Ollama /api/generate handler — this is what KernelClaw uses */
async function handleGenerate(body) {
  const { model, prompt, system, stream = false, format } = body;

  const zaiInst = await ensureZai();

  const messages = [];

  // Use provided system prompt or default KernelClaw one
  if (system) {
    messages.push({ role: "system", content: system });
  } else {
    messages.push({ role: "system", content: KERNELCLAW_SYSTEM });
  }

  messages.push({ role: "user", content: prompt });

  const completion = await zaiInst.chat.completions.create({
    messages,
    temperature: 0.2,
  });

  let content = completion.choices?.[0]?.message?.content || "";

  // If JSON format requested, extract and validate
  if (format === "json") {
    try {
      const parsed = extractJSON(content);
      content = JSON.stringify(parsed);
    } catch {
      // If LLM didn't return valid JSON, wrap in our structure
      content = JSON.stringify({
        task_id: `goal_${Date.now()}`,
        tool_name: "echo",
        parameters: { input: prompt },
        justification: `Auto-parsed (LLM JSON extraction failed) from: ${prompt}`,
        risk_level: "low",
        required_capabilities: ["file_read"],
        expected_output_type: "text",
      });
    }
  }

  // Ollama generate response format
  return {
    model: model || "kernelclaw:latest",
    response: content,
    done: true,
    total_duration: 0,
    eval_count: 0,
  };
}

/** Ollama /api/tags handler — list available models */
function handleTags() {
  return {
    models: Object.entries(MODELS).map(([name, info]) => ({
      name,
      model: name,
      modified_at: new Date().toISOString(),
      size: info.size,
      digest: "sha256:kernelclaw",
      details: info,
    })),
  };
}

// ── HTTP Server ───────────────────────────────────────────────────────
const server = http.createServer(async (req, res) => {
  // CORS headers
  res.setHeader("Access-Control-Allow-Origin", "*");
  res.setHeader("Access-Control-Allow-Methods", "GET, POST, OPTIONS");
  res.setHeader("Access-Control-Allow-Headers", "Content-Type");

  if (req.method === "OPTIONS") {
    res.writeHead(204);
    return res.end();
  }

  const url = new URL(req.url, `http://localhost:${PORT}`);

  // Route: GET /api/tags
  if (req.method === "GET" && url.pathname === "/api/tags") {
    return sendJSON(res, 200, handleTags());
  }

  // Route: POST /api/chat
  if (req.method === "POST" && url.pathname === "/api/chat") {
    try {
      const body = await readBody(req);
      const result = await handleChat(body);
      return sendJSON(res, 200, result);
    } catch (err) {
      return sendJSON(res, 500, { error: err.message });
    }
  }

  // Route: POST /api/generate
  if (req.method === "POST" && url.pathname === "/api/generate") {
    try {
      const body = await readBody(req);
      const result = await handleGenerate(body);
      return sendJSON(res, 200, result);
    } catch (err) {
      return sendJSON(res, 500, { error: err.message });
    }
  }

  // Route: GET / (health)
  if (req.method === "GET" && url.pathname === "/") {
    return sendJSON(res, 200, {
      status: "ok",
      backend: "ollama-bridge (z-ai-web-dev-sdk)",
      models: Object.keys(MODELS),
    });
  }

  // 404
  sendJSON(res, 404, { error: "not found" });
});

function readBody(req) {
  return new Promise((resolve, reject) => {
    const chunks = [];
    req.on("data", (c) => chunks.push(c));
    req.on("end", () => {
      try {
        resolve(JSON.parse(Buffer.concat(chunks).toString()));
      } catch {
        reject(new Error("invalid JSON body"));
      }
    });
    req.on("error", reject);
  });
}

function sendJSON(res, status, obj) {
  const body = JSON.stringify(obj);
  res.writeHead(status, { "Content-Type": "application/json" });
  res.end(body);
}

server.listen(PORT, () => {
  console.log(
    `🦀 ollama-bridge listening on http://localhost:${PORT}`,
  );
  console.log(`   Models: ${Object.keys(MODELS).join(", ")}`);
  console.log(`   Endpoints: /api/generate, /api/chat, /api/tags`);
});