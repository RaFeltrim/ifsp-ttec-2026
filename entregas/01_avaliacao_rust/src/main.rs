use std::io::{self, Write};
use crud_livros::*; // Importa a lógica do lib.rs

// ==========================================
// ENTRADA E SAÍDA (CLI)
// ==========================================

fn ler_teclado() -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Falha ao ler entrada");
    entrada.trim().to_string()
}

fn ler_numero() -> u32 {
    loop {
        let entrada = ler_teclado();
        match entrada.parse::<u32>() {
            Ok(num) => return num,
            Err(_) => {
                print!("Por favor, digite um número válido: ");
                io::stdout().flush().unwrap();
            }
        }
    }
}

fn ler_dados_livro() -> Livro {
    print!("Título: ");
    io::stdout().flush().unwrap();
    let titulo = ler_teclado();

    print!("Autor: ");
    io::stdout().flush().unwrap();
    let autor = ler_teclado();

    print!("Editora: ");
    io::stdout().flush().unwrap();
    let editora = ler_teclado();

    print!("Páginas: ");
    io::stdout().flush().unwrap();
    let paginas = ler_numero();

    Livro { titulo, autor, editora, paginas }
}

fn exibir_livro(livro: &Livro) {
    println!("  Título:  {}", livro.titulo);
    println!("  Autor:   {}", livro.autor);
    println!("  Editora: {}", livro.editora);
    println!("  Páginas: {}", livro.paginas);
}

fn main() {
    let mut biblioteca: Vec<Livro> = Vec::new();

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        println!("=============================");
        println!("       CRUD DE LIVROS");
        println!("=============================");
        println!("1 - Inserir livro");
        println!("2 - Alterar livro");
        println!("3 - Apagar livro");
        println!("4 - Exibir um livro");
        println!("5 - Exibir todos os livros");
        println!("0 - Sair");
        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();

        let opcao = ler_teclado();
        println!("-----------------------------");

        match opcao.as_str() {
            "1" => {
                println!("--- Inserir Novo Livro ---\n");
                let novo_livro = ler_dados_livro();
                match inserir_livro(&mut biblioteca, novo_livro) {
                    Ok(_) => println!("\nLivro inserido com sucesso!"),
                    Err(e) => println!("\n{}", e),
                }
            }
            "2" => {
                println!("--- Alterar Livro ---\n");
                if biblioteca.is_empty() {
                    println!("A biblioteca está vazia. Nada para alterar.");
                } else {
                    print!("Digite o título do livro a alterar: ");
                    io::stdout().flush().unwrap();
                    let titulo_busca = ler_teclado();
                    
                    if buscar_livro(&biblioteca, &titulo_busca).is_some() {
                        println!("\nLivro encontrado! Digite os novos dados:\n");
                        let novos_dados = ler_dados_livro();
                        match alterar_livro(&mut biblioteca, &titulo_busca, novos_dados) {
                            Ok(_) => println!("\nLivro atualizado com sucesso!"),
                            Err(e) => println!("\n{}", e),
                        }
                    } else {
                        println!("\nLivro '{}' não encontrado.", titulo_busca);
                    }
                }
            }
            "3" => {
                println!("--- Apagar Livro ---\n");
                if biblioteca.is_empty() {
                    println!("A biblioteca está vazia. Nada para apagar.");
                } else {
                    print!("Digite o título do livro a apagar: ");
                    io::stdout().flush().unwrap();
                    let titulo_busca = ler_teclado();
                    
                    match apagar_livro(&mut biblioteca, &titulo_busca) {
                        Ok(_) => println!("Livro '{}' removido com sucesso!", titulo_busca),
                        Err(e) => println!("{}", e),
                    }
                }
            }
            "4" => {
                println!("--- Buscar Livro ---\n");
                if biblioteca.is_empty() {
                    println!("A biblioteca está vazia.");
                } else {
                    print!("Digite o título do livro: ");
                    io::stdout().flush().unwrap();
                    let titulo_busca = ler_teclado();
                    
                    match buscar_livro(&biblioteca, &titulo_busca) {
                        Some(livro) => {
                            println!("\nLivro encontrado:\n");
                            exibir_livro(livro);
                        }
                        None => println!("\nLivro '{}' não encontrado.", titulo_busca),
                    }
                }
            }
            "5" => {
                println!("--- Todos os Livros ---\n");
                if biblioteca.is_empty() {
                    println!("A biblioteca está vazia.");
                } else {
                    println!("Total: {} livro(s)\n", biblioteca.len());
                    for (i, livro) in biblioteca.iter().enumerate() {
                        println!("-- Livro #{} --", i + 1);
                        exibir_livro(livro);
                        println!();
                    }
                }
            }
            "0" => {
                println!("Encerrando o sistema. Até mais!");
                break;
            }
            _ => {
                println!("Opção inválida! Tente novamente.");
            }
        }

        println!("\nPressione ENTER para voltar ao menu...");
        ler_teclado();
    }
}
