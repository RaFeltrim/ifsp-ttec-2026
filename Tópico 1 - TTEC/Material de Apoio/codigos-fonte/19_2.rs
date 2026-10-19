
fn main() {

    // gerar valores para x, de 1 até 20 (inclusive):

    for x in 1..=20 {

        // Vamos exibir x e uma mensagem relacionada ao valor de x:

        println!( "{}: {}", x, match x {

            // x = 1, exiba pouco:
            1 => "Pouco",

            // se x for 2 ou 3, exiba um pouquinho:
            2 | 3 => "Um pouquinho",

            // se x for entre 4 e 10 (inclusive), exiba "um bocado":
            4..=10 => "Um bocado",



            // Tirando o que já foi definido acima, qualquer valor par
            // exiba "Uma boa quantidade par",
            _ if x % 2 == 0 => "Uma boa quantidade par",

            // Qualquer outro valor que não se encaixou acima,
            // exiba "Muito"
            _ => "Muito"

        });
    }
}


