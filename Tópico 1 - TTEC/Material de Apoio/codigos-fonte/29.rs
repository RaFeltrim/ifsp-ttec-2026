
// Função para exibir uma string
// recebendo sua referência MUTÁVEL:

fn exibe_string( s: &mut String ) {

    println!( "{}", s );

    // Concatenando algo no final:
    s.push_str(" - São Carlos");

}


fn main() {

    // String alocada na heap (String):

    let mut uma_string = String::from("IFSP");

    // Passando referência, mas o main ainda é dono:
    exibe_string( &mut uma_string );

    // Exibindo direto por aqui também:
    println!( "{}", uma_string );

}




