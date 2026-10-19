# SCLTTEC — Tópicos em Tecnologias Emergentes (2026)

[![IFSP - São Carlos](https://img.shields.io/badge/IFSP-S%C3%A3o%20Carlos-red.svg)](https://scl.ifsp.edu.br/)
[![Disciplina](https://img.shields.io/badge/Disciplina-SCLTTEC-blue.svg)]()
[![Docente](https://img.shields.io/badge/Docente-Prof.%20Carl%C3%A3o-informational.svg)]()
[![Etapa 1](https://img.shields.io/badge/Etapa%201-M%C3%A9dia%2010%2C00%20(100%25)-success.svg)]()
[![CI - Rust](https://github.com/RaFeltrim/ifsp-ttec-2026/actions/workflows/ci.yml/badge.svg)](https://github.com/RaFeltrim/ifsp-ttec-2026/actions)

Repositório acadêmico central de estudos, projetos práticos, provas de conceito (PoCs), resumos teóricos e materiais de apoio da disciplina **SCLTTEC - Tópicos em Tecnologias Emergentes**, ministrada no Instituto Federal de Educação, Ciência e Tecnologia de São Paulo (**IFSP — Câmpus São Carlos**).

---

## 📌 Informações Gerais

- **Instituição:** Instituto Federal de Educação, Ciência e Tecnologia de São Paulo (IFSP) — Câmpus São Carlos
- **Código da Disciplina:** SCLTTEC
- **Semestre Letivo:** 2026.2
- **Docente Responsável:** Prof. Carlos José de Almeida Pereira (Carlão)
  - *Prontuário:* 2362353
  - *E-mail institucional:* [carlao2005@ifsp.edu.br](mailto:carlao2005@ifsp.edu.br)
- **Discentes:**
  - Rafael Feltrim (SC3038734) — [GitHub](https://github.com/RaFeltrim)
  - Gustavo Gomes Contiero (SC3037754)
- **Status Acadêmico:**
  - **Etapa 1:** Concluída com **Média 10,00** (100% de aproveitamento em T1, T2 e T3)
  - **Etapa Final:** Prova A1 (Pendente)

---

## 🗂️ Estrutura Oficial do Repositório por Módulos & Tópicos

```text
ifsp-ttec-2026/
│
├── 📁 .github/
│   └── 📁 workflows/
│       └── ci.yml                                            # CI/CD automatizado via GitHub Actions (Rust build + testes BDD)
│
├── 📁 docs/                                                  # Central de documentação e onboarding do ambiente
│   ├── README.md                                             # Índice da central de documentação
│   ├── setup_ambiente.md                                     # Guia de instalação de Rustup, Leiningen, Java e Python
│   └── referencias.md                                        # Bibliografia recomendada e documentações oficiais
│
├── 📁 Tópico 0 - Informacoes Gerais/
│   └── README.md                                             # Ementa, corpo docente, critérios de avaliação e notas
│
├── 📁 Tópico 1 - TTEC/                                       # Módulo 1: Rust e Concorrência
│   ├── README.md                                             # Panorama do módulo de Rust, Ownership e BDD
│   ├── 📁 Material de Apoio/
│   │   ├── setup_ambiente.md
│   │   └── referencias.md
│   ├── 📁 Exercícios e Laboratórios/
│   │   ├── 📁 00_template/                                   # Template base para novas PoCs em Rust
│   │   ├── 📁 01_crud_livros/                                # [LAB 1] CRUD CLI com Rust, Vec, Struct e Cucumber (BDD)
│   │   └── 📁 02_crud_livros_web/                            # [LAB 2] CRUD Web com Axum, REST API e Frontend HTML/JS
│   └── 📁 Entregas/
│       ├── README.md                                         # Histórico detalhado de entregas do módulo
│       └── 📁 01_avaliacao_rust/                             # [ENTREGA T1 - NOTA 10,00] CRUD de Livros CLI com BDD
│
├── 📁 Tópico 2 - Programacao Funcional Clojure/               # Módulo 2: Programação Funcional na JVM
│   ├── README.md                                             # Panorama funcional, sintaxe Lisp, REPL e Leiningen
│   ├── 📁 Material de Apoio/
│   │   └── README.md                                         # Links e guias oficiais de Clojure
│   └── 📁 Entregas/
│       └── 📁 02_avaliacao_clojure/                          # [ENTREGA T2 - NOTA 10,00] Loja Diária (EDN/JSON)
│
├── 📁 Tópico 3 - Redes de Proteinas e Cancer/                 # Módulo 3: Ciência de Redes e Bioinformática
│   ├── README.md                                             # Teoria de redes biológicas, interactomas e robustez a falhas
│   ├── 📁 Material de Apoio/
│   │   ├── Redes de Proteínas e Cancer - Ataque Resiliencia e Alvos.pdf # Slides da palestra Prof. Carlão Ramos
│   │   ├── Atividade.ipynb                                   # Notebook de partida disponibilizado pelo docente
│   │   ├── input.zip                                         # Pacote de dados brutos compactados
│   │   └── 📁 input/                                         # Redes de pathways (DNA Repair, etc.) e canonical_drivers.txt
│   └── 📁 Entregas/
│       └── 📁 03_avaliacao_redes_proteinas/                  # [ENTREGA T3 - NOTA 10,00] Avaliação Ramos
│           ├── README.md                                     # Relatório técnico analítico em Markdown
│           ├── TTEC_Avaliacao_Ramos_corrigido.pdf           # [RELATÓRIO OFICIAL CORRIGIDO] Documento final em PDF
│           ├── TTEC_Avaliacao_Ramos.pdf                     # Relatório original da primeira submissão
│           ├── avaliacao_ramos.ipynb                         # Notebook Jupyter resolvido (Google Colab)
│           ├── avaliacao_ramos.py                           # Pipeline analítico executável via terminal
│           ├── 📁 dados/                                     # Subconjunto de dados para execução imediata
│           └── 📁 img/                                       # Boxplots de centralidade e histograma de impacto no LCC
│
├── 📁 entregas/                                              # Portal centralizado de artefatos finais para avaliação
│   ├── README.md                                             # Sumário consolidado de entregas
│   ├── 📁 01_avaliacao_rust/                                 # [T1] CRUD Rust + BDD
│   ├── 📁 02_avaliacao_clojure/                              # [T2] Loja Diária Clojure
│   └── 📁 03_avaliacao_redes_proteinas/                      # [T3] Relatório e Código de Redes de Proteínas
│
├── 📁 pocs/                                                  # Provas de Conceito (PoCs) mantidas no repositório
│   ├── 📁 00_template/
│   ├── 📁 01_crud_livros/
│   ├── 📁 02_crud_livros_web/
│   ├── 📁 03_avaliacao_clojure/
│   └── 📁 04_redes_proteinas/
│
├── .gitignore                                                # Filtro de binários, target/, .env e temporários
└── README.md                                                 # Portal central de navegação
```

---

## 📚 Mapa Geral de Atividades & Entregas da Disciplina

| Sigla | Atividade / Tema | Módulo / Tópico | Prazo | Status | Nota | Arquivo Final de Entrega |
| :---: | :--- | :--- | :---: | :---: | :---: | :--- |
| **T1** | **Avaliação Rust (CRUD de Livros)** | [Tópico 1 - Rust](Tópico%201%20-%20TTEC/) | 11/08/2026 | ✅ **Entregue** | **10,00** | [`entregas/01_avaliacao_rust/`](entregas/01_avaliacao_rust/) |
| **T2** | **Avaliação Clojure (Loja Diária)** | [Tópico 2 - Clojure](Tópico%202%20-%20Programacao%20Funcional%20Clojure/) | 08/09/2026 | ✅ **Entregue** | **10,00** | [`entregas/02_avaliacao_clojure/`](entregas/02_avaliacao_clojure/) |
| **T3** | **Palestra Ramos (Redes de Proteínas e Câncer)** | [Tópico 3 - Bioinformática](Tópico%203%20-%20Redes%20de%20Proteinas%20e%20Cancer/) | 06/10/2026 | ✅ **Entregue** | **10,00** | [`TTEC_Avaliacao_Ramos_corrigido.pdf`](entregas/03_avaliacao_redes_proteinas/TTEC_Avaliacao_Ramos_corrigido.pdf) |
| **A1** | **Prova Final da Disciplina** | Avaliação Geral | A definir | ⏳ **Pendente** | — | *Aguardando agendamento* |

> 🏆 **Desempenho da Etapa 1:** **Média 10,00** (100% de aproveitamento em todas as avaliações)

---

## 🧪 Provas de Conceito (PoCs) Implementadas

| # | Nome | Tecnologias | Descrição |
|---|---|---|---|
| 01 | [CRUD Livros (CLI)](pocs/01_crud_livros/) | Rust, Vec, Struct, Cucumber (BDD) | Sistema de gerenciamento de livros em memória com cobertura BDD declarativa |
| 02 | [CRUD Livros (Web)](pocs/02_crud_livros_web/) | Rust, Axum, HTML, CSS, JavaScript | API REST assíncrona com interface web reativa integrada |
| 03 | [Avaliação Clojure](pocs/03_avaliacao_clojure/) | Clojure, Leiningen, EDN/JSON | Processamento funcional de transações diárias de loja com desconto inteligente |
| 04 | [Redes de Proteínas e Cancro](pocs/04_redes_proteinas/) | Python, NetworkX, Matplotlib, Pandas | Análise topológica de interactomas tumorigênicos e ataque sistemático no LCC |

---

## 🚀 Como Executar os Projetos

### 1. PoCs e Entregas em Rust (Tópico 1)
```bash
# CRUD Livros (CLI)
cd "Tópico 1 - TTEC/Exercícios e Laboratórios/01_crud_livros"
cargo run
cargo test --test cucumber

# CRUD Livros (Web)
cd "../02_crud_livros_web"
cargo run
```

### 2. Entrega em Clojure (Tópico 2)
```bash
cd "Tópico 2 - Programacao Funcional Clojure/Entregas/02_avaliacao_clojure"
lein run
```

### 3. Entrega em Python / NetworkX (Tópico 3)
```bash
cd "Tópico 3 - Redes de Proteinas e Cancer/Entregas/03_avaliacao_redes_proteinas"
python avaliacao_ramos.py
```

---

## 👥 Autores

- **Rafael Feltrim** ([GitHub](https://github.com/RaFeltrim)) — IFSP Câmpus São Carlos
- **Gustavo Gomes Contiero** — IFSP Câmpus São Carlos
