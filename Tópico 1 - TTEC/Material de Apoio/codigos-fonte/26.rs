
// Função para exibir uma string criada na heap (String)
// recebida por parâmetro:

fn exibe_string( s: String ) -> String {

    println!( "{}", s );

    // Lembrando, sem ; é uma expressão, retorna um valor.
    // Vamos retornar a String s:

    s

}


fn main() {

    // String alocada na heap (String):

    let uma_string = String::from("IFSP");

    // Chamando a função e recuperando a String no final:
    let uma_string = exibe_string( uma_string );

    // Exibindo direto por aqui também:
    println!( "{}", uma_string );

}




