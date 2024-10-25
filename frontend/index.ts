import { serve } from "bun";
import { join } from "path";
import { createConnection } from "net";

import { portfolio } from "./src/generated/portfolio.ts";

const PORT = 3000;
const PUBLIC_DIR = "./public";


function generateUniqueString(length: number): string {
  const randomBytes = new Uint8Array(length);
  crypto.getRandomValues(randomBytes);
  return Array.from(randomBytes, byte => byte.toString(16).padStart(2, '0')).join('');
}


const pendingResponses: Map<string, any> = new Map();

const client = createConnection({ port: 5000 }, () => {
  console.log("Connected to server");
});

client.on("data", (data) => {
  const response = JSON.parse(data.toString());
  const { id, result } = response;

  if (pendingResponses.has(id)) {
    const resolve = pendingResponses.get(id);
    resolve(result);
    pendingResponses.delete(id);
  }
});

client.on("end", () => {
  console.log("Disconnected from server");
});

client.on("error", (err) => {
  console.error(`Connection error: ${err.message}`);
});

serve({
  development: false,
  port: PORT,
  async fetch(req) {
    const url = new URL(req.url);

    if (url.pathname.startsWith("/api")) {
      const queryString = url.searchParams.get("q");
      if (queryString) {
        const requestId = generateUniqueString(32);

        const responsePromise = new Promise((resolve) => {
          pendingResponses.set(requestId, resolve);
        });

        const query: portfolio.Query = new portfolio.Query();
        query.uuid = requestId;
        query.query = queryString;      

        client.write(query.serializeBinary());
        const result = await responsePromise;

        return new Response(JSON.stringify({ result }), {
          headers: { "Content-Type": "application/json" },
        });
      }
    }

    let filePath = join(PUBLIC_DIR, url.pathname);

    if (url.pathname === "/") {
      filePath += "index.html";
    }

    const file = Bun.file(filePath);
    return new Response(file);
  },
});

console.log(`Server running at http://localhost:${PORT}`);