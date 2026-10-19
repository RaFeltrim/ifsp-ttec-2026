Feature: Gerenciamento de Livros
  Como um usuário do sistema
  Eu quero poder inserir, alterar, buscar e apagar livros
  Para gerenciar a minha biblioteca pessoal

  Scenario: Inserir um novo livro com sucesso
    Given que a biblioteca esta vazia
    When eu insiro um livro com titulo "Rust Book", autor "Steve", editora "No Starch", paginas 500
    Then a biblioteca deve conter 1 livro
    And o livro "Rust Book" deve ser o primeiro da lista

  Scenario: Tentar inserir um livro duplicado
    Given que a biblioteca ja possui o livro "Rust Book"
    When eu insiro um livro com titulo "Rust Book", autor "Clone", editora "Fake", paginas 100
    Then a insercao deve falhar com a mensagem "Já existe um livro com este título"
    And a biblioteca deve conter 1 livro

  Scenario: Alterar os dados de um livro existente
    Given que a biblioteca ja possui o livro "Rust Book"
    When eu altero o livro "Rust Book" para titulo "Rust Book v2", autor "Carol", editora "Editora", paginas 600
    Then a biblioteca deve conter o livro "Rust Book v2" com 600 paginas

  Scenario: Apagar um livro existente
    Given que a biblioteca ja possui o livro "Rust Book"
    When eu apago o livro "Rust Book"
    Then a biblioteca deve ficar vazia
