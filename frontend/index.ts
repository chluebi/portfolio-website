import { serve } from "bun";
import { join } from "path";

const PORT = 3000;
const PUBLIC_DIR = "./public";

serve({
  port: PORT,
  fetch(req) {
    const url = new URL(req.url);
    console.log(url);
    let filePath = join(PUBLIC_DIR, url.pathname);
    if (url.pathname == '/') {
      filePath += 'index.html'
    }
    
    const file = Bun.file(filePath);
    return new Response(file);
  },
});

console.log(`Server running at http://localhost:${PORT}`);