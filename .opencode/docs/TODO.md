# TODO — Cortex

## Fase 1 — Setup
- [x] Instalar Rust (mínimo: rustc, cargo, clippy, rustfmt)
- [x] Corrigir versões do Cargo.toml
- [x] Adaptar main.rs para lambda_http 1.x
- [ ] Instalar cargo-lambda (`cargo install cargo-lambda`)
- [ ] Criar `template.yaml` (SAM)
- [ ] Configurar DynamoDB tables (Pages, Vectors)
- [ ] Deploy health check

## Fase 1.5 — CI/CD
- [ ] Configurar GitHub Actions (lint, test, build)
- [ ] Adicionar scripts no package.json

## Fase 2 — CRUD
- [ ] CRUD de páginas (GET/POST/PUT/DELETE)
- [ ] Validação + erros

## Fase 3 — RAG
- [ ] Vector search (Scan + cosine)
- [ ] Embeddings (Hugging Face free tier)
- [ ] Chat com RAG

## Fase 4 — Deploy
- [ ] S3 + CloudFront (frontend)
- [ ] Cognito (auth)