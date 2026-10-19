
// Função para exibir uma string criada na stack (&str)
// recebida por parâmetro:

fn exibe_string( s: &str ) {

    println!( "{}", s );

}


fn main() {

    // String alocada na stack (&str):

    let uma_string = "IFSP";

    // Chamando a função acima pra exibir:
    exibe_string( uma_string );

    // Exibindo direto por aqui também:
    println!( "{}", uma_string );

}




