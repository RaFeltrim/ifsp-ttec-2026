# Entregas — Tópicos em Tecnologias Emergentes (IFSP)

Diretório centralizado contendo os artefatos de cada entrega da disciplina.

## Entregas Realizadas

| # | Entrega | Data Limite | Status |
|---|---------|-------------|--------|
| 01 | [Avaliação Rust — CRUD de Livros](01_avaliacao_rust/) | 11/08/2026 | Entregue |
| 02 | [Avaliação Clojure — Loja Diária](02_avaliacao_clojure/) | 08/09/2026 | Entregue |


---

## 01 — Avaliação Rust: CRUD de Livros

**Integrantes:** Rafael Feltrim (SC3038734) e Gustavo Gomes Contiero (SC3037754)

### O que foi solicitado
- Criar uma `struct Livro` com: titulo, autor, editora, num_paginas
- Armazenar os livros em um `Vec` (somente em memória)
- Busca pelo título (sem repetições)
- Funções de CRUD: inserir, alterar (todos os dados), apagar, exibir um livro, exibir todos
- Menu principal para chamar as funções

### O que foi entregue
- Tudo o que foi solicitado acima, devidamente implementado
- Arquitetura modular: lógica de negócio separada (`src/lib.rs`) da interface CLI (`src/main.rs`)
- Testes automatizados BDD com Cucumber (especificação em `features/livros.feature`)
- README documentando o projeto e instruções de execução

### Como executar
```bash
cargo run                      # Aplicação CLI
cargo test --test cucumber     # Testes BDD (Cucumber/Gherkin)
```

---

## 02 — Avaliação Clojure: Loja Diária

**Integrantes:** Rafael Feltrim e Gustavo Gomes Contiero

### O que foi solicitado
- Processamento de transações diárias de uma loja (lidas a partir de um arquivo `dados.json`).
- Geração de informações estratégicas: total de vendas concluídas com sucesso, total de vendas canceladas e saldo do dia.
- Clientes especiais possuem desconto de 10% aplicado automaticamente.

### O que foi entregue
- Lógica completa implementada em Clojure.
- Processamento dos dados estruturados via EDN/JSON.
- Automação via Leiningen.

### Como executar
```bash
cd 02_avaliacao_clojure
lein run
```
