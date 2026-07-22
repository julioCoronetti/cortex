# AGENTS.md

## Stack

- Frontend: React + Vite (PWA)
- Backend: Rust + Axum
- Banco: MongoDB Atlas (M0 grátis)
- Vector search: Qdrant (na VM)
- IA: OpenCode Go API
- Infra: DigitalOcean VM + Tailscale

## O que não fazer

- Não sugira banco relacional.
- Não sugira SolidJS, Next.js ou frameworks excessivos.
- Não sugira arquitetura complexa (clean architecture, microserviços).
- Não sugira domínio público, login social ou App Store.
- Não sugira AWS/Azure.
- Não ignore a privacidade.

## Padrões

- Backend Rust: arquivos diretos (`main.rs`, `routes.rs`, `db.rs`, etc).
- Frontend React: componentes diretos, sem over-engineering.
- Código em inglês, conceitos em português.
- Só commit se o usuário pedir.

## Antes de codar

1. Leia `README.md`.
2. Leia `TODO.md` para saber o próximo passo.
3. Confirme antes de mudar stack ou arquitetura.

## Como interagir

- Respostas diretas e curtas.
- O usuário odeia repetir explicações longas.
- Simplicidade > organização perfeita.
