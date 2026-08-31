# Avaliação Rust — CRUD de Livros

Código fonte referente à avaliação prática de Rust da disciplina de Tópicos em Tecnologias Emergentes (IFSP).

## Integrantes
- Rafael Feltrim (SC3038734)
- Gustavo Gomes Contiero (SC3037754)

## Estrutura
```
01_avaliacao_rust/
├── Cargo.toml                 # Configuração do projeto Rust
├── src/
│   ├── lib.rs                 # Lógica de negócio (struct Livro + funções CRUD)
│   └── main.rs                # Interface CLI (menu interativo)
├── features/
│   └── livros.feature         # Especificação BDD em Gherkin
└── tests/
    └── cucumber.rs            # Motor de testes Cucumber
```

## Como Executar
```bash
cargo run                      # Rodar a aplicação
cargo test --test cucumber     # Rodar os testes BDD
```
