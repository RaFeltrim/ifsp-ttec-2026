
// Função para exibir uma string
// recebendo sua referência:

fn exibe_string( s: &String ) {

    println!( "{}", s );

}


fn main() {

    // String alocada na heap (String):

    let uma_string = String::from("IFSP");

    // Passando referência, mas o main ainda é dono:
    exibe_string( &uma_string );

    // Exibindo direto por aqui também:
    println!( "{}", uma_string );

}




