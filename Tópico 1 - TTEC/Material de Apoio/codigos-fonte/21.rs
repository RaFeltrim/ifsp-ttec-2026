fn main() {

    // Como já vimos, o Rust infere qual o tipo:
    let s1 = String::from("instituto");

    println!("s1: {}, tipo {}", s1, type_of(&s1));

    // Aqui vamos dizer explicitamente qual o tipo:
    let s2: String = String::from("federal");

    println!("s2: {}, tipo {}", s2, type_of(&s2));

    // Lembrando, por default, a variável é imutável.
    // Logo, se descomentadas, as linhas abaixo dão erro,
    // pois não podemos mudar o conteúdo de s2 (o 'objeto' string),
    // nem para onde s2 está apontando (uma outra string, p.ex.):
    //
    // Trocando o conteúdo da string apontada por s2:
    // s2.push_str(" SP");
    //
    // Trocando totalmente o objeto apontado por s2 (para um novo objeto):
    // s2 = String::from("SP");


    // Podemos usar mut:
    let mut s3: String = String::from("sao paulo");

    println!("s3: {}, tipo {}", s3, type_of(&s3));

    s3.push_str(" - campus sao carlos");

    println!("s3: {}, tipo {}", s3, type_of(&s3));

    s3 = String::from("VASCAO!!!");

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

