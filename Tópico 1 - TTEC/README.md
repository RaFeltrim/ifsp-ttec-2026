# Tópico 1 — Rust e Concorrência: Do CLI com BDD ao Web Service

Módulo dedicado ao aprendizado da linguagem de programação de sistemas **Rust**, cobrindo desde conceitos fundamentais de segurança de memória sem garbage collector (*ownership*, *borrowing*, *lifetimes*) até testes orientados a comportamento (BDD) e construção de APIs web modernas com **Axum**.

---

## 🗂️ Estrutura do Tópico

```text
Tópico 1 - TTEC/
├── 📁 Entregas/
│   └── 📁 01_avaliacao_rust/          # [ENTREGA T1 - NOTA 10,00] CRUD Livros CLI + BDD
│       ├── Cargo.toml
│       ├── README.md
│       ├── 📁 features/               # Especificação Gherkin (livros.feature)
│       ├── 📁 src/                    # lib.rs (negócio) e main.rs (CLI)
│       └── 📁 tests/                  # cucumber.rs (step definitions)
│
├── 📁 Exercícios e Laboratórios/
│   ├── 📁 00_template/                # Scaffold padrão para novas PoCs em Rust
│   ├── 📁 01_crud_livros/             # Implementação laboratorial com Cargo.lock e testes
│   └── 📁 02_crud_livros_web/         # Servidor Axum REST + Frontend interativo (HTML/JS)
│
└── 📁 Material de Apoio/
    ├── cont01_apresentacao.pdf        # Slides oficiais — Apresentação da Disciplina
    ├── cont02_rust_parte1.pdf         # Slides oficiais — Aula 01: Rust Parte 1
    ├── cont02_rust_parte2.pdf         # Slides oficiais — Aula 02: Rust Parte 2
    ├── codigos-fonte.zip              # Pacote original compactado dos códigos de aula
    ├── 📁 codigos-fonte/              # Exemplos de aula do docente (01.rs a 33.rs e aleatorio)
    ├── setup_ambiente.md              # Passo a passo de instalação de rustup e cargo
    └── referencias.md                 # Links para The Rust Book, Rustlings e tutoriais
```

---

## 🧪 Práticas Desenvolvidas

### 1. CRUD de Livros CLI com BDD (`01_crud_livros`)
- **Conceitos:** Gerenciamento de memória com `Vec<Livro>`, `struct`, `match`, busca por título sem duplicidade.
- **Engenharia de Qualidade:** Testes BDD usando **Cucumber-rs** com cenários declarativos em Gherkin (`features/livros.feature`).
- **Arquitetura:** Separação estrita de responsabilidades entre regras de negócio (`src/lib.rs`) e camada de apresentação no terminal (`src/main.rs`).

### 2. CRUD Web com Axum (`02_crud_livros_web`)
- **Conceitos:** Servidor assíncrono baseado em Tokio e Axum.
- **Endpoints REST:** `GET /livros`, `POST /livros`, `PUT /livros/:titulo`, `DELETE /livros/:titulo`.
- **Frontend Integrado:** Interface gráfica reativa em Vanilla HTML, CSS e JavaScript sem dependências externas.

---

## 🚀 Como Executar os Laboratórios

```bash
# 1. Executar a versão CLI
cd "Tópico 1 - TTEC/Exercícios e Laboratórios/01_crud_livros"
cargo run

# 2. Executar os testes BDD com Cucumber
cargo test --test cucumber

# 3. Executar a versão Web
cd "../02_crud_livros_web"
cargo run
# Acessar via navegador em: http://localhost:3000
```
