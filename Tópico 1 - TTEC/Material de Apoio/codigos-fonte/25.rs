
// Função para exibir uma string criada na heap (String)
// recebida por parâmetro:

fn exibe_string( s: String ) {

    println!( "{}", s );

}


fn main() {

    // String alocada na heap (String):

    let uma_string = String::from("IFSP");

    // Chamando a função acima pra exibir:
    exibe_string( uma_string );

    // Exibindo direto por aqui também:
    println!( "{}", uma_string );

}




