# Entregas — Tópicos em Tecnologias Emergentes (IFSP)

Diretório centralizado contendo os artefatos de cada entrega da disciplina.

## Entregas Realizadas

| Sigla | Entrega | Descrição | Data Limite | Nota | Status |
|:---:|---|---|:---:|:---:|:---:|
| **T1** | [Avaliação Rust — CRUD de Livros](01_avaliacao_rust/) | CRUD em memória e testes BDD (Cucumber) | 11/08/2026 | **10,00** | Entregue |
| **T2** | [Avaliação Clojure — Loja Diária](../../entregas/02_avaliacao_clojure/) | Processamento funcional de transações em JSON | 08/09/2026 | **10,00** | Entregue |
| **T3** | [Palestra Ramos — Redes de Proteínas](../../entregas/03_avaliacao_redes_proteinas/) | Análise topológica e ataque sistemático no LCC | 06/10/2026 | **10,00** | Entregue |


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

---

## 03 — Avaliação Redes de Proteínas e Cancro (Avaliação Ramos)

**Integrantes:** Rafael Feltrim e Gustavo Gomes Contiero  
**Submissão Oficial:** 06/10/2026 para `carlao2005@ifsp.edu.br`

### O que foi solicitado
- **Atividade 1:** Análise e distribuição das 4 métricas clássicas de centralidade (Degree, Clustering, Betweenness, Closeness) comparando genes drivers e não-drivers da via DNA Repair através de boxplots.
- **Atividade 2:** Simulação de ataque individual sistemático para cada um dos 43 genes drivers e medição do impacto ($\Delta B$) na média de betweenness do Maior Componente Conexo (LCC), ranqueando os alvos críticos prioritários.

### O que foi entregue
- Relatório técnico completo em PDF: [`TTEC_Avaliacao_Ramos_corrigido.pdf`](../../entregas/03_avaliacao_redes_proteinas/TTEC_Avaliacao_Ramos_corrigido.pdf)
- Notebook Jupyter integrado com Google Colab: [`avaliacao_ramos.ipynb`](../../entregas/03_avaliacao_redes_proteinas/avaliacao_ramos.ipynb)
- Script Python standalone com execução local: [`avaliacao_ramos.py`](../../entregas/03_avaliacao_redes_proteinas/avaliacao_ramos.py)
- Gráficos gerados (boxplots e histograma de impactos) e base de dados inclusa em `dados/`.

### Como executar
```bash
cd ../../entregas/03_avaliacao_redes_proteinas
python avaliacao_ramos.py
```

