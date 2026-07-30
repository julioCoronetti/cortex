# TODO — Cortex

## Fase 1 — Setup
- [ ] Instalar cargo-lambda (`cargo install cargo-lambda`)
- [ ] Criar `api/` com Lambda health check (Rust)
- [ ] Criar `template.yaml` (SAM)
- [ ] Configurar DynamoDB tables (Pages, Vectors)
- [ ] Deploy health check

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
- [ ] CI/CD básico

## Ordem
1. Semana 1: Fase 1
2. Semana 2: Fase 2
3. Semana 3: Fase 3
4. Semana 4: Fase 4