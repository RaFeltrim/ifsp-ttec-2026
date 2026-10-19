
fn main() {

    // Adultos (18 ou mais anos) podem sair do IFSP.
    // Jovens (15 até 17 anos) só podem sair se autorizados.

    let idade:u8 = 15;
    let saida_autorizada = true;

    // OU lógico: ||
    // E  lógico: &&

    if idade > 17 || idade > 14 && saida_autorizada {

        println!("Pode sair do IFSP");

    } else {

        println!("Não pode sair do IFSP");

    }

}

