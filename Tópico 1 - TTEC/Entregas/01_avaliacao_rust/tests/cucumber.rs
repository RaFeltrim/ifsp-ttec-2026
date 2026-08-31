use cucumber::{given, when, then, World};
use crud_livros::{Livro, inserir_livro, alterar_livro, apagar_livro};

#[derive(Debug, Default, World)]
struct LivrosWorld {
    biblioteca: Vec<Livro>,
    ultimo_erro: Option<String>,
}

#[given("que a biblioteca esta vazia")]
fn biblioteca_vazia(world: &mut LivrosWorld) {
    world.biblioteca.clear();
}

#[given(expr = "que a biblioteca ja possui o livro {string}")]
fn biblioteca_com_livro(world: &mut LivrosWorld, titulo: String) {
    world.biblioteca.push(Livro {
        titulo,
        autor: String::from("Autor"),
        editora: String::from("Editora"),
        paginas: 100,
    });
}

#[when(expr = "eu insiro um livro com titulo {string}, autor {string}, editora {string}, paginas {int}")]
fn insiro_livro(world: &mut LivrosWorld, titulo: String, autor: String, editora: String, paginas: u32) {
    let novo = Livro { titulo, autor, editora, paginas };
    match inserir_livro(&mut world.biblioteca, novo) {
        Ok(_) => world.ultimo_erro = None,
        Err(e) => world.ultimo_erro = Some(e.to_string()),
    }
}

#[when(expr = "eu altero o livro {string} para titulo {string}, autor {string}, editora {string}, paginas {int}")]
fn altero_livro(world: &mut LivrosWorld, titulo_busca: String, titulo: String, autor: String, editora: String, paginas: u32) {
    let novos_dados = Livro { titulo, autor, editora, paginas };
    match alterar_livro(&mut world.biblioteca, &titulo_busca, novos_dados) {
        Ok(_) => world.ultimo_erro = None,
        Err(e) => world.ultimo_erro = Some(e.to_string()),
    }
}

#[when(expr = "eu apago o livro {string}")]
fn apago_livro(world: &mut LivrosWorld, titulo_busca: String) {
    match apagar_livro(&mut world.biblioteca, &titulo_busca) {
        Ok(_) => world.ultimo_erro = None,
        Err(e) => world.ultimo_erro = Some(e.to_string()),
    }
}

#[then(expr = "a biblioteca deve conter {int} livro")]
fn a_biblioteca_deve_conter_x_livros(world: &mut LivrosWorld, qtd: usize) {
    assert_eq!(world.biblioteca.len(), qtd);
}

#[then(expr = "a biblioteca deve ficar vazia")]
fn a_biblioteca_deve_ficar_vazia(world: &mut LivrosWorld) {
    assert_eq!(world.biblioteca.len(), 0);
}

#[then(expr = "o livro {string} deve ser o primeiro da lista")]
fn o_livro_deve_ser_o_primeiro(world: &mut LivrosWorld, titulo: String) {
    assert_eq!(world.biblioteca[0].titulo, titulo);
}

#[then(expr = "a insercao deve falhar com a mensagem {string}")]
fn a_insercao_deve_falhar(world: &mut LivrosWorld, mensagem: String) {
    assert_eq!(world.ultimo_erro.as_ref().unwrap(), &mensagem);
}

#[then(expr = "a biblioteca deve conter o livro {string} com {int} paginas")]
fn a_biblioteca_deve_conter_o_livro_com_x_paginas(world: &mut LivrosWorld, titulo: String, paginas: u32) {
    let livro = world.biblioteca.iter().find(|l| l.titulo == titulo).unwrap();
    assert_eq!(livro.paginas, paginas);
}

fn main() {
    futures::executor::block_on(LivrosWorld::run("features/livros.feature"));
}
