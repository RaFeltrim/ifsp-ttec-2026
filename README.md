# Tópicos em Tecnologias Emergentes (SCLTTEC) — IFSP

Repositório acadêmico para armazenamento e organização das **Provas de Conceito (PoCs)** desenvolvidas ao longo da disciplina de Tópicos em Tecnologias Emergentes do Instituto Federal de São Paulo.

## Integrantes
- Rafael Feltrim (SC3038734)
- Gustavo Gomes Contiero (SC3037754)

## Estrutura do Repositório

```
ifsp-ttec-2026/
├── .github/workflows/ci.yml   # CI/CD com GitHub Actions (Rust build + testes)
├── docs/                       # Documentação e referências
│   ├── setup_ambiente.md
│   └── referencias.md
├── pocs/                       # Provas de Conceito organizadas por tema
│   ├── 00_template/            # Template base para novas PoCs
│   ├── 01_crud_livros/         # CRUD de Livros — CLI (Rust, Vec, Struct, BDD/Cucumber)
│   ├── 02_crud_livros_web/     # CRUD de Livros — Web (Axum + HTML/JS)
│   ├── 03_avaliacao_clojure/   # Avaliação Clojure — Loja Diária (EDN, JSON, Leiningen)
│   └── 04_redes_proteinas/     # Redes de Proteínas e Cancro — Topologia e Resiliência (NetworkX)
├── entregas/                   # Artefatos oficiais submetidos para avaliação
│   ├── 02_avaliacao_clojure/
│   └── 03_avaliacao_redes_proteinas/
└── README.md
```

## PoCs Implementadas

| # | Nome | Tecnologias | Descrição |
|---|------|------------|-----------|
| 01 | [CRUD Livros (CLI)](pocs/01_crud_livros/) | Rust, Vec, Struct, Cucumber (BDD) | Sistema de gerenciamento de livros via terminal com testes orientados a comportamento |
| 02 | [CRUD Livros (Web)](pocs/02_crud_livros_web/) | Rust, Axum, HTML, CSS, JavaScript | Versão web do CRUD com API REST e interface gráfica |
| 03 | [Avaliação Clojure](pocs/03_avaliacao_clojure/) | Clojure, Leiningen, EDN/JSON | Processamento de transações diárias de uma loja a partir de um arquivo JSON |
| 04 | [Redes de Proteínas e Cancro](pocs/04_redes_proteinas/) | Python, NetworkX, Matplotlib, Pandas | Análise topológica de interactomas tumorais e simulação de resiliência a ataques direcionados |

## Entregas Oficiais

Os artefatos finais enviados para avaliação estão centralizados na pasta [`entregas/`](entregas/):

| # | Entrega | Tema | Submissão |
|---|---|---|---|
| 01 | [Avaliação Rust](Tópico%201%20-%20TTEC/Entregas/01_avaliacao_rust/) | CRUD em memória e testes BDD | 11/08/2026 |
| 02 | [Avaliação Clojure](entregas/02_avaliacao_clojure/) | Processamento funcional de dados diários de loja | 08/09/2026 |
| 03 | [Avaliação Ramos (Redes de Proteínas)](entregas/03_avaliacao_redes_proteinas/) | Análise topológica, centralidades e ataque a LCC | 06/10/2026 |

## Como Executar

Cada PoC possui seu próprio `README.md` com instruções detalhadas. De forma geral:

```bash
# Navegar até a PoC desejada
cd pocs/01_crud_livros

# Executar a aplicação
cargo run

# Executar os testes BDD (Cucumber)
cargo test --test cucumber
```

## Como Contribuir

1. Copie o diretório `pocs/00_template/` para criar uma nova PoC.
2. Siga o padrão de nomenclatura: `pocs/XX_nome_da_poc/`.
3. Documente o setup no `README.md` de cada PoC.
4. Garanta cobertura de testes na pasta `tests/` ou `features/`.
