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
│   └── 02_crud_livros_web/     # CRUD de Livros — Web (Axum + HTML/JS)
└── README.md
```

## PoCs Implementadas

| # | Nome | Tecnologias | Descrição |
|---|------|------------|-----------|
| 01 | [CRUD Livros (CLI)](pocs/01_crud_livros/) | Rust, Vec, Struct, Cucumber (BDD) | Sistema de gerenciamento de livros via terminal com testes orientados a comportamento |
| 02 | [CRUD Livros (Web)](pocs/02_crud_livros_web/) | Rust, Axum, HTML, CSS, JavaScript | Versão web do CRUD com API REST e interface gráfica |

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
