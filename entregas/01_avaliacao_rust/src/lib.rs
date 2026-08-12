#[derive(Debug, Clone, PartialEq)]
pub struct Livro {
    pub titulo: String,
    pub autor: String,
    pub editora: String,
    pub paginas: u32,
}

pub fn inserir_livro(biblioteca: &mut Vec<Livro>, livro: Livro) -> Result<(), &'static str> {
    if biblioteca.iter().any(|l| l.titulo == livro.titulo) {
        return Err("Já existe um livro com este título");
    }
    biblioteca.push(livro);
    Ok(())
}

pub fn alterar_livro(biblioteca: &mut Vec<Livro>, titulo_busca: &str, novos_dados: Livro) -> Result<(), &'static str> {
    if let Some(livro) = biblioteca.iter_mut().find(|l| l.titulo == titulo_busca) {
        livro.titulo = novos_dados.titulo;
        livro.autor = novos_dados.autor;
        livro.editora = novos_dados.editora;
        livro.paginas = novos_dados.paginas;
        Ok(())
    } else {
        Err("Livro não encontrado")
    }
}

pub fn apagar_livro(biblioteca: &mut Vec<Livro>, titulo_busca: &str) -> Result<(), &'static str> {
    let tamanho_antes = biblioteca.len();
    biblioteca.retain(|livro| livro.titulo != titulo_busca);
    if biblioteca.len() < tamanho_antes {
        Ok(())
    } else {
        Err("Livro não encontrado")
    }
}

pub fn buscar_livro<'a>(biblioteca: &'a Vec<Livro>, titulo_busca: &str) -> Option<&'a Livro> {
    biblioteca.iter().find(|livro| livro.titulo == titulo_busca)
}
