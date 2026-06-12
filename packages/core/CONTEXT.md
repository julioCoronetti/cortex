# Contexto - Backend Core

## Ultima atualizacao

2026-06-12: criado mapa conceitual do backend core. Nenhum codigo implementado ainda.

Este arquivo descreve o que o backend precisa sustentar para web, desktop e mobile. Nao define ordem de implementacao.

## Papel do backend

Backend core = nucleo logico compartilhado do Cortex.

Objetivo: manter uma base pessoal, local, privada, autoral e auditavel, parecida com Notion no modelo de organizacao, mas com foco nativo em RAG.

Caracteristicas centrais:

- Uso pessoal, sem modelo SaaS.
- Local-first, com sincronizacao remota ainda em decisao.
- Mesma logica consumida por web, desktop e mobile.
- Codigo simples, modular e auditavel.
- Dominios claros, sem arquitetura complexa demais.
- RAG como parte estrutural do produto, nao plugin posterior.

## Fronteiras

Dentro do backend core:

- Modelo de conteudo.
- Operacoes de escrita, leitura, busca e organizacao.
- Banco local e migracoes.
- Camada de sincronizacao abstrata.
- Indexacao full-text.
- Chunking, embeddings, recuperacao semantica e montagem de contexto RAG.
- Historico, auditoria, backup e restauracao.
- Contratos logicos para apps web, desktop e mobile.

Fora do backend core:

- UI e estado visual.
- Componentes de design.
- Codigo especifico de Tauri, Expo ou browser.
- Provedor concreto de IA.
- Provedor concreto de sincronizacao remota.
- Fluxos SaaS: usuarios, times, billing, roles, permissoes multiusuario.

## Premissas atuais

- Sistema tem um unico dono.
- Privacidade importa mais que conveniencia.
- Dados devem continuar acessiveis sem internet.
- Sincronizacao deve ser substituivel: Google Drive, S3 ou outro storage.
- Banco local provavelmente sera fonte primaria de verdade.
- Storage remoto deve guardar snapshots, blobs ou log de sincronizacao, nao virar dependencia rigida do dominio.
- Cada operacao relevante deve ser auditavel.
- RAG precisa citar origem do contexto usado.

## Dominios logicos

### Workspace

Representa instancia pessoal do Cortex.

Responsabilidades:

- Configuracoes globais.
- Identidade local do workspace.
- Chaves, politicas de criptografia e metadata tecnica.
- Status de saude do banco.
- Versao de schema e migracoes.

### Conteudo

Representa paginas, documentos, blocos e registros estruturados.

Responsabilidades:

- Criar, editar, mover, arquivar e remover conteudo.
- Manter hierarquia.
- Manter ordem de blocos.
- Permitir conteudo livre e conteudo estruturado.
- Preservar metadata: criado em, atualizado em, origem, checksum, versao.

### Colecoes

Representam databases estilo Notion, mas simples.

Responsabilidades:

- Definir schemas de propriedades.
- Criar registros.
- Filtrar, ordenar e agrupar registros.
- Relacionar registros com paginas, tags e arquivos.

### Relacoes

Representam grafo pessoal de conhecimento.

Responsabilidades:

- Links manuais entre conteudos.
- Backlinks.
- Tags.
- Relacoes tipadas.
- Referencias usadas por RAG.

### Assets

Representam arquivos anexos.

Responsabilidades:

- Importar arquivos.
- Gerar checksum.
- Evitar duplicacao.
- Extrair texto e metadata quando possivel.
- Relacionar arquivo com conteudo.
- Controlar blobs locais e remotos.

### Busca

Representa recuperacao lexical e semantica.

Responsabilidades:

- Busca full-text.
- Busca por filtros.
- Busca por tags, links e propriedades.
- Busca semantica via embeddings.
- Busca hibrida.
- Reindexacao.

### RAG

Representa pipeline de preparacao e recuperacao de contexto para IA.

Responsabilidades:

- Chunking de documentos.
- Geracao e atualizacao de embeddings.
- Selecao de contexto relevante.
- Montagem de pacote de contexto.
- Citacao de fontes.
- Controle de escopo da consulta.
- Registro do que foi usado em cada resposta.

### Conversas e IA

Representam interacoes com modelos.

Responsabilidades:

- Salvar prompts, respostas e fontes usadas.
- Associar conversas a conteudos.
- Permitir perguntas sobre workspace inteiro ou subconjunto.
- Suportar resumo, classificacao e extracao.
- Manter provedores de IA substituiveis.

### Sincronizacao

Representa propagacao de dados entre dispositivos.

Responsabilidades:

- Detectar alteracoes locais.
- Enviar estado para storage remoto.
- Baixar estado remoto.
- Resolver conflito.
- Registrar versao sincronizada.
- Separar dominio de provider concreto.

Provider remoto ainda em aberto:

- Google Drive: simples para uso pessoal, mas acoplamento e API podem incomodar.
- S3: mais neutro, bom para blobs/snapshots, exige credenciais e estrategia de seguranca.
- Outro provider: manter interface pequena para permitir troca.

### Backup e restauracao

Representam seguranca operacional.

Responsabilidades:

- Criar snapshot local.
- Criar snapshot remoto.
- Validar integridade.
- Restaurar estado anterior.
- Exportar pacote auditavel.

### Auditoria

Representa trilha de eventos.

Responsabilidades:

- Registrar operacoes importantes.
- Permitir inspecao do que mudou.
- Associar alteracoes a entidades.
- Ajudar debug de sync, RAG e migracoes.

## Operacoes logicas mapeadas

### Workspace

- Inicializar workspace.
- Abrir workspace existente.
- Ler configuracoes.
- Atualizar configuracoes.
- Validar integridade.
- Executar migracoes.
- Listar capacidades disponiveis.
- Exportar metadata tecnica.

### Conteudo

- Criar pagina.
- Criar documento.
- Criar bloco.
- Atualizar bloco.
- Reordenar blocos.
- Mover bloco entre paginas.
- Renomear conteudo.
- Arquivar conteudo.
- Restaurar conteudo arquivado.
- Remover conteudo.
- Duplicar conteudo.
- Listar arvore de conteudo.
- Buscar conteudo por id.
- Buscar filhos de um conteudo.
- Registrar versao ou snapshot.

### Colecoes

- Criar colecao.
- Atualizar schema de colecao.
- Remover colecao.
- Criar registro.
- Atualizar propriedades de registro.
- Remover registro.
- Consultar registros por filtro.
- Ordenar registros.
- Agrupar registros.
- Relacionar registro com pagina.

### Relacoes

- Criar tag.
- Renomear tag.
- Mesclar tags.
- Remover tag.
- Adicionar tag a entidade.
- Remover tag de entidade.
- Criar link entre entidades.
- Remover link entre entidades.
- Listar backlinks.
- Listar grafo de vizinhanca.
- Criar relacao tipada.

### Assets

- Importar arquivo local.
- Registrar arquivo externo.
- Calcular checksum.
- Detectar duplicata.
- Extrair texto.
- Extrair metadata.
- Vincular asset a conteudo.
- Desvincular asset.
- Remover asset.
- Listar assets orfaos.

### Busca

- Indexar entidade.
- Remover entidade do indice.
- Reindexar workspace.
- Buscar texto.
- Buscar por filtros.
- Buscar por tags.
- Buscar por backlinks.
- Executar busca semantica.
- Executar busca hibrida.
- Retornar resultados com fonte e score.

### RAG

- Quebrar conteudo em chunks.
- Atualizar chunks apos edicao.
- Gerar embeddings.
- Invalidar embeddings obsoletos.
- Buscar chunks relevantes.
- Montar contexto com citacoes.
- Limitar contexto por escopo.
- Registrar contexto usado.
- Reprocessar entidade para RAG.

### IA

- Criar conversa.
- Adicionar mensagem.
- Executar pergunta com contexto.
- Executar resumo de conteudo.
- Executar extracao estruturada.
- Associar resposta a fontes.
- Listar historico de conversa.
- Remover conversa.

### Sync e storage remoto

- Configurar provider remoto.
- Validar credenciais.
- Criar snapshot remoto.
- Enviar alteracoes.
- Baixar alteracoes.
- Comparar versoes.
- Detectar conflito.
- Resolver conflito.
- Listar snapshots remotos.
- Restaurar snapshot remoto.
- Rodar sync dry-run.

### Backup, exportacao e importacao

- Criar backup local.
- Validar backup.
- Restaurar backup.
- Exportar workspace completo.
- Exportar subconjunto.
- Importar Markdown.
- Importar JSON.
- Importar arquivo bruto.
- Gerar relatorio de importacao.

### Auditoria e manutencao

- Registrar evento.
- Listar eventos por entidade.
- Listar eventos por periodo.
- Consultar log de erros.
- Compactar banco.
- Verificar integridade de blobs.
- Verificar integridade de indices.
- Gerar diagnostico do workspace.

## Contratos esperados para apps

Apps devem consumir backend por contratos simples:

- Comandos para mutacoes.
- Queries para leitura.
- Eventos para mudancas observaveis.
- DTOs estaveis para transporte entre plataformas.
- Erros estruturados com codigo, mensagem e causa tecnica.

Apps nao devem conhecer detalhes de:

- Schema fisico do banco.
- Provider remoto.
- Provider de embeddings.
- Estrategia interna de chunking.
- Formato interno de auditoria.

## Decisoes em aberto

- Banco local final: SQLite/SQLCipher segue como candidato principal.
- Formato de storage remoto: snapshot completo, log incremental, blobs separados ou combinacao.
- Provider remoto inicial: Google Drive, S3 ou outro.
- Estrategia de criptografia: chave local, senha mestra, chave por dispositivo ou combinacao.
- Estrategia de conflito: last-write-wins, merge por entidade, merge por campo ou resolucao manual.
- Modelo de conteudo: blocos livres, documentos Markdown, banco de registros ou hibrido.
- Motor de busca vetorial: dentro do SQLite, arquivo separado ou provider local.
- Provedor de IA e embeddings: local, OpenAI ou interface multiprovider.

## Checklist norteadora

- [x] Definir que backend e pessoal, local-first e nao SaaS.
- [x] Separar dominio de provider remoto.
- [x] Mapear dominios logicos iniciais.
- [x] Mapear operacoes logicas principais.
- [ ] Escolher modelo de conteudo base.
- [ ] Escolher banco local.
- [ ] Escolher estrategia de sync remoto.
- [ ] Escolher estrategia de RAG e embeddings.
- [ ] Definir contratos de comandos, queries e eventos.
- [ ] Definir formato de auditoria.
- [ ] Definir formato de backup/exportacao.
