# Cortex

Base de conhecimento pessoal, privada e online, com RAG nativo.

## O que é

Aplicativo de notas pessoal, inspirado no Notion, com foco nativo em RAG. Você escreve, organiza e conversa com seus dados usando IA.

## Stack

- **Frontend:** PWA com React + Vite
- **Backend:** Rust + Axum
- **Banco:** MongoDB Atlas (M0 grátis)
- **Vector search:** Qdrant (self-hosted na VM)
- **IA:** OpenCode Go API
- **Infra:** DigitalOcean VM + Tailscale
- **Futuro:** migrar para Hetzner ou Oracle Cloud quando os créditos acabarem

## Arquitetura

```
[Dispositivo] → Tailscale → [VM DigitalOcean]
                                  ├── Backend Rust (Axum)
                                  └── Qdrant (Docker)
                                  ↓
                           [MongoDB Atlas]
```

- Frontend PWA acessa o backend pela rede Tailscale.
- Backend salva documentos no MongoDB e vetores no Qdrant.
- Worker assíncrono processa edições em lote: atualiza documento, gera chunks e embeddings.

## Princípios

- Só você usa.
- Acesso privado via Tailscale.
- Código simples, sem arquitetura excessiva.
- IA sugere alterações; você aprova antes de aplicar.

## Estrutura

```
cortex/
├── AGENTS.md
├── README.md
├── TODO.md
├── client/       # frontend PWA
└── api/          # backend Rust
```

## Próximos passos

Ver `TODO.md`.
