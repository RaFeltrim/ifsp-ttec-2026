use std::io;

fn ler_teclado() -> String {

    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Falha ao ler a entrada");
    entrada.trim().to_string() // remove \n e retorna a string

}


fn main() {

    println!("Digite algo:");
    let texto = ler_teclado();
    println!("Você digitou: {}", texto);

}

