# Cortex API

Backend do Cortex em Rust.

## Stack

- Rust
- Axum
- MongoDB driver (futuro)
- Qdrant client (futuro)

## Como rodar (desenvolvimento)

No Windows, o Rust precisa das Build Tools do VS (linker MSVC). O `cargo` está em `%USERPROFILE%\.cargo\bin`.

```powershell
# a partir da raiz do repo
cd api
cargo run
```

Servidor sobe em `http://127.0.0.1:8080`.

## Endpoints

- `GET /health` → `{ "status": "ok" }`

## Como testar ponta-a-ponta (backend + frontend)

Abra **dois terminais**.

**Terminal 1 — backend:**
```powershell
cd api
cargo run
```
Deve aparecer: `cortex-api listening on 0.0.0.0:8080`

**Terminal 2 — frontend:**
```powershell
cd client
npm run dev
```
Deve aparecer: `Local: http://localhost:5173/`

Abra `http://localhost:5173` no navegador.

O que você deve ver:
- Título "Cortex"
- "Backend: online" (se o backend estiver no ar)
- "Backend: offline" (se o backend estiver parado)

**Como funciona:**
1. O frontend faz `fetch('/health')`.
2. O Vite dev server (porta 5173) tem um proxy configurado que encaminha `/health` para `http://127.0.0.1:8080/health` (o backend Rust).
3. Se o backend responder `{"status":"ok"}`, o front mostra "online".
4. Se o backend estiver parado, o `fetch` falha e o front mostra "offline".

Para testar o "offline": pare o backend (Ctrl+C no Terminal 1) e recarregue a página.

## Estrutura dos arquivos

```
api/src/
├── main.rs     # entrypoint: sobe o servidor Axum na porta 8080
└── routes.rs   # rotas: por enquanto só /health
```
