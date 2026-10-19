
fn main() {

    // Como já vimos, o Rust infere qual o tipo:
    let s1 = "instituto";

    println!("s1: {}, tipo {}", s1, type_of(&s1));

    // Aqui vamos dizer explicitamente qual o tipo:
    let s2: &str = "federal";

    println!("s2: {}, tipo {}", s2, type_of(&s2));

    // Lembrando, por default, a variável é imútavel.
    // Logo, se descomentada, a linha abaixo dá erro,
    // pois não podemos mudar o conteúdo de s2:
    // s2 = "de";


    // Podemos usar mut:

    let mut s3: &str = "sao paulo";

    println!("s3: {}, tipo {}", s3, type_of(&s3));    

    s3 = "sao carlos";

    println!("s3: {}, tipo {}", s3, type_of(&s3));

}



///////////////////////////////////////////////////////////////////////
//
// A função abaixo serve para ver o tipo de dados de uma variável.
// Não vamos estudar nesse momomento! Está aqui só pra ajudar a gente.

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

///////////////////////////////////////////////////////////////////////

