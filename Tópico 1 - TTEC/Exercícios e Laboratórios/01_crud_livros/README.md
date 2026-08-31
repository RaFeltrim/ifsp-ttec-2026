# CRUD de Livros (Avaliação Rust)

Este projeto foi desenvolvido como avaliação prática da disciplina de Tópicos em Tecnologias Emergentes. O objetivo é implementar um sistema de gerenciamento de livros simples utilizando Rust e seus conceitos básicos de gerenciamento de memória (Borrowing, Ownership), structs, vetores (`Vec`) e laços de repetição.

## Integrantes
- Rafael Feltrim (SC3038734)
- Gustavo Gomes Contiero (SC3037754)

## Estrutura do Projeto
- O projeto roda exclusivamente no terminal (CLI).
- Os dados são armazenados na memória (utilizando a Heap, via `Vec<Livro>`).
- Todas as operações exigidas pelo escopo (Inserir, Alterar, Apagar, Buscar um, Buscar todos) estão implementadas.
- Busca realizada unicamente pelo título do livro.

## Arquitetura e Testes (SPDD/BDD)
Para garantir a consistência das operações e validar a separação de responsabilidades (I/O isolado das Regras de Negócio):
- A lógica de negócio está modularizada em `src/lib.rs`.
- A interface de usuário via CLI está em `src/main.rs`.
- Utilizamos **Spec-Driven Development (SPDD)** com testes baseados em comportamento (BDD) através da biblioteca `cucumber`.
- As especificações em linguagem natural (Gherkin) encontram-se em `features/livros.feature`.
- Os executores de testes do Cucumber encontram-se em `tests/cucumber.rs`.

## Como Executar
O projeto foi construído utilizando as ferramentas padrão da linguagem (Cargo). Para rodar a aplicação, basta abrir o terminal na pasta do projeto e executar:

```bash
cargo run
```

Para executar a suíte de testes unitários e de integração (Cucumber) garantindo a consistência das regras, execute:

```bash
cargo test --test cucumber
```
