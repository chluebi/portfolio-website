FROM debian:slim

RUN apt-get update && \
    apt-get install -y curl && \
    curl -fsSL https://bun.sh/install | bash && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

ENV BUN_INSTALL="/root/.bun"
ENV PATH="$BUN_INSTALL/bin:$PATH"

WORKDIR /app

COPY . .

RUN bun install

EXPOSE 3000

CMD ["bunx", "tsc", "&&", "bun", "run", "index.ts"]