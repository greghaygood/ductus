/**
 * ductus — pi extension bridge to the ductus runtime.
 *
 * Pi has no built-in MCP client (by design), so this extension wraps the
 * ductus runtime's own MCP server — `.ductus/bin/ductus mcp` — over stdio
 * and re-registers every tool it lists as a pi tool under the `ductus__`
 * prefix. The runtime's MCP server stays the single source of truth for tool
 * names and schemas: this file holds no second copy of the registry, and a
 * runtime release that adds or changes a primitive reaches pi with no edit
 * here.
 *
 * Written to `.pi/extensions/ductus.ts` by /ductus (Shared Files manifest,
 * strategy `update`). Zero npm dependencies — the MCP-over-stdio transport is
 * newline-delimited JSON-RPC 2.0, which this file speaks directly. Keep it
 * dependency-free: the README promises nothing enters the adopter's
 * dependency manifest, and the live-schema property depends on the bridge
 * staying a thin transport.
 *
 * Failures are loud, never silent: when `.ductus/bin/ductus` is missing or
 * not executable, the tools return an error envelope naming the file and
 * pointing at `/ductus` (re-acquisition), and a one-time notice is shown at
 * session start. No markdown-only fallback — a wired host that lost its
 * binary must stop, not degrade (§runtime-boundary).
 *
 * spec 058 (pi host support). See framework/bootstrap/ductus.md §MCP
 * registration.
 */

import { spawn } from "node:child_process";
import { resolve, dirname, join } from "node:path";
import { existsSync } from "node:fs";

/** The MCP protocol version the rmcp-based runtime serves (2025-03-26). */
const MCP_PROTOCOL_VERSION = "2025-03-26";

type Json = unknown;
interface JsonRpcResponse {
  jsonrpc: "2.0";
  id: number;
  result?: Json;
  error?: { code: number; message: string };
}

interface McpTool {
  name: string;
  description?: string;
  inputSchema: Record<string, Json>;
}

interface ToolResult {
  content: { type: "text"; text: string }[];
  isError: boolean;
}

/** A single ductus runtime child process, held for the session. */
class DuctusServer {
  private child: ReturnType<typeof spawn> | null = null;
  private lineBuffer = "";
  private pending = new Map<number, { resolve: (r: JsonRpcResponse) => void }>();
  private nextId = 1;
  /** tools/call is strictly request/response over one stdio pipe, so calls
   *  are serialized through a promise chain — a burst would interleave lines
   *  and break framing, and deterministic pipeline order is preserved. */
  private callChain: Promise<ToolResult> = Promise.resolve({
    content: [],
    isError: false,
  });
  private toolsCache: McpTool[] | null = null;

  pointerPath: string;

  constructor(pointerPath: string) {
    this.pointerPath = pointerPath;
  }

  private ensureChild(): ReturnType<typeof spawn> {
    if (this.child && (this.child as { pid?: number }).pid) return this.child;
    const proc = spawn(this.pointerPath, ["mcp"]);
    this.child = proc;
    this.lineBuffer = "";
    proc.stdout?.on("data", (chunk: Buffer) => this.onData(chunk));
    /* Settle every in-flight request as a transport error on exit, so no call
     * hangs the session on a dead child; the next call respawns. */
    proc.on("exit", () => {
      const dying = this.pending;
      this.pending = new Map();
      for (const [, waiter] of dying) {
        waiter.resolve({
          jsonrpc: "2.0",
          id: -1,
          error: { code: -32000, message: "ductus child exited" },
        });
      }
    });
    return proc;
  }

  private onData(chunk: Buffer) {
    this.lineBuffer += chunk.toString();
    let nl: number;
    while ((nl = this.lineBuffer.indexOf("\n")) >= 0) {
      const line = this.lineBuffer.slice(0, nl).trim();
      this.lineBuffer = this.lineBuffer.slice(nl + 1);
      if (!line) continue;
      let msg: JsonRpcResponse;
      try {
        msg = JSON.parse(line) as JsonRpcResponse;
      } catch {
        continue; /* not our response */
      }
      if (typeof msg.id === "number") {
        const waiter = this.pending.get(msg.id);
        if (waiter) {
          this.pending.delete(msg.id);
          waiter.resolve(msg);
        }
      }
    }
  }

  private rpc(method: string, params: Record<string, Json>): Promise<JsonRpcResponse> {
    const proc = this.ensureChild();
    const id = this.nextId++;
    /* Cap every call so a wedged child cannot freeze the session; the
     * deterministic server answers well under this. */
    const timeout = setTimeout(() => {
      const waiter = this.pending.get(id);
      if (waiter) {
        this.pending.delete(id);
        waiter.resolve({
          jsonrpc: "2.0",
          id,
          error: { code: -32001, message: "ductus call timed out" },
        });
      }
    }, 120_000);
    const promise = new Promise<JsonRpcResponse>((res) => {
      this.pending.set(id, { resolve: res });
    }).then((resp) => {
      clearTimeout(timeout);
      return resp;
    });
    proc.stdin?.write(
      JSON.stringify({ jsonrpc: "2.0", id, method, params }) + "\n",
    );
    return promise;
  }

  /** Tools are fetched once per child (the child lives for the session). */
  async listTools(): Promise<McpTool[]> {
    if (this.toolsCache) return this.toolsCache;
    const init = await this.rpc("initialize", {
      protocolVersion: MCP_PROTOCOL_VERSION,
      capabilities: {},
      clientInfo: { name: "ductus-bridge", version: "0.53.0" },
    });
    if (init.error) {
      throw new Error(`ductus initialize failed: ${init.error.message}`);
    }
    await this.rpc("notifications/initialized", {});
    const listed: McpTool[] = [];
    let cursor: string | undefined;
    do {
      const resp = await this.rpc("tools/list", cursor ? { cursor } : {});
      if (resp.error) {
        throw new Error(`ductus tools/list failed: ${resp.error.message}`);
      }
      const result = resp.result as { tools?: McpTool[]; nextCursor?: string };
      listed.push(...(result.tools ?? []));
      cursor = result.nextCursor;
    } while (cursor);
    this.toolsCache = listed;
    return listed;
  }

  callTool(name: string, args: Record<string, Json>): Promise<ToolResult> {
    this.callChain = this.callChain.then(async () => {
      const resp = await this.rpc("tools/call", { name, arguments: args });
      if (resp.error) {
        return {
          content: [
            { type: "text", text: `ductus: ${name} failed: ${resp.error.message}` },
          ],
          isError: true,
        };
      }
      const result = resp.result as {
        content?: { type?: string; text?: string }[];
        isError?: boolean;
      };
      const blocks = (result.content ?? []).map((block) =>
        block.type === "text" && typeof block.text === "string"
          ? { type: "text", text: block.text }
          : { type: "text", text: JSON.stringify(block) }
      );
      return { content: blocks, isError: result.isError === true };
    });
    return this.callChain;
  }
}

/** Repo root: the directory containing `.ductus/` *and* `.pi/`. The
 *  extension file anchor works on most loaders (`<root>/.pi/extensions/`,
 *  two directories up), but pi loads extensions through jiti where the file
 *  path and `__dirname` are not guaranteed to align — so the working
 *  directory (pi starts in the repo root) is walked upward for the `.ductus/`
 *  marker instead, which is the same anchor /ductus itself uses. Falls back
 *  to the process cwd when no marker is found (the pointer will then be
 *  reported missing by the error envelope, naming the actual path). */
function projectRoot(): string {
  let dir = resolve(process.cwd());
  while (true) {
    if (existsSync(join(dir, ".ductus"))) return dir;
    const parent = dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  try {
    if (typeof __dirname !== "undefined") {
      return resolve(dirname(__dirname), "..", "..");
    }
  } catch {
    /* fall through to cwd */
  }
  return resolve(process.cwd());
}

/** The pointer /ductus materializes per project; the bridge's sole contract
 *  with the runtime is that path (same as every wired host). */
function pointerPath(): string {
  return resolve(projectRoot(), ".ductus/bin/ductus");
}

/** Deferred registration work, shared between load and the tools. */
let server: DuctusServer | null = null;
let loadError: Error | null = null;
let noticeShown = false;
let toolsRegistered = false;

export default function (pi: ExtensionAPI) {
  /* Eagerly registered tools would each carry a tool-call that lazily spawns
   * on first use — but the tool list comes from the runtime, so registration
   * itself must complete before the pipeline can run. Do it at load; route
   * the one-time failure notice to a session_start ctx (ExtensionAPI exposes
   * no ui of its own). */
  pi.on("session_start", (_event, ctx) => {
    if (noticeShown) return;
    if (!loadError) return;
    ctx.ui.notify(
      `ductus: ${loadError.message}`,
      "error",
    );
    noticeShown = true;
  });

  async function registerTools(): Promise<void> {
    if (toolsRegistered) return;
    if (!server) server = new DuctusServer(pointerPath());
    let tools: McpTool[];
    try {
      tools = await server.listTools();
    } catch (err) {
      loadError =
        err instanceof Error
          ? err
          : new Error(`${pointerPath()} is missing or not executable — run /ductus to re-acquire it`);
      if (!loadError.message.includes("/ductus")) {
        loadError = new Error(
          `${pointerPath()} is missing or not executable — run /ductus to re-acquire it (${loadError.message})`,
        );
      }
      throw loadError;
    }
    for (const tool of tools) {
      const inputSchema = tool.inputSchema ?? { type: "object" };
      const body = {
        name: `ductus__${tool.name}`,
        label: tool.name,
        description: tool.description ?? `ductus primitive ${tool.name}`,
        parameters: inputSchema,
        execute: async (_id: string, params: Record<string, Json>) => {
          if (!server) server = new DuctusServer(pointerPath());
          try {
            return await server.callTool(tool.name, params ?? {});
          } catch (err) {
            return {
              content: [
                {
                  type: "text",
                  text: `ductus: ${pointerPath()} is missing or unusable — run /ductus to re-acquire it (${err instanceof Error ? err.message : String(err)})`,
                },
              ],
              isError: true,
            };
          }
        },
      };
      pi.registerTool(body);
    }
    toolsRegistered = true;
  }

  /* Register at load; surface the failure eagerly through the one-time
   * session_start notice, and every tool call keeps returning the error
   * envelope while the runtime is missing. */
  registerTools().catch(() => {});
}