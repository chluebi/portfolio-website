import { Socket, serve } from "bun";
import { join } from "path";
import { createConnection } from "net";

import { portfolio } from "./src/generated/portfolio.ts";

const BACKEND_HOST = Bun.env.BACKEND_HOST || "127.0.0.1";
const BACKEND_PORT = Number(Bun.env.BACKEND_PORT) || 5000;
const PORT = Bun.env.FRONTEND_PORT || 3000;
const PUBLIC_DIR = "./public";


function generateUniqueString(length: number): string {
  const randomBytes = new Uint8Array(length);
  crypto.getRandomValues(randomBytes);
  return Array.from(randomBytes, byte => byte.toString(16).padStart(2, '0')).join('');
}


const pendingResponses: Map<string, any> = new Map();

const MAX_RETRIES = 10;    // Number of retry attempts
const RETRY_INTERVAL = 2000; // Delay between retries in milliseconds

let client: ReturnType<typeof createConnection>;

function connectToBackend(retries: number) {
  console.log("Trying to connect to http://" + BACKEND_HOST + ":" + BACKEND_PORT);
  client = createConnection({ host: BACKEND_HOST, port: BACKEND_PORT }, () => {
    console.log("Connected to server");
  });

  client.on("data", (data) => {
    const response = portfolio.Response.deserializeBinary(data);
    const projects = response.projects.projects;
    
    if (response.uuid) {
      console.log("uuid " + response.uuid);
    }
  });

  client.on("end", () => {
    console.log("Disconnected from server");
  });

  client.on("error", (err) => {
    console.error(`Connection error: ${err.message}`);
    if (retries > 0) {
      console.log(`Retrying connection in ${RETRY_INTERVAL / 1000} seconds...`);
      setTimeout(() => connectToBackend(retries - 1), RETRY_INTERVAL);
    } else {
      console.error("Max retries reached. Could not connect to backend.");
    }
  });
}

// Start the connection with retry logic
connectToBackend(MAX_RETRIES);

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