
fn main() {

    let mut idade = 16;

    let mut maioridade = if idade > 17 { "maior" } else { "menor" };

    println!("A pessoa é {} de idade", maioridade);

    idade = 20;
    maioridade = if idade > 17 { "maior" } else { "menor" };
    println!("A pessoa é {} de idade", maioridade);

}

