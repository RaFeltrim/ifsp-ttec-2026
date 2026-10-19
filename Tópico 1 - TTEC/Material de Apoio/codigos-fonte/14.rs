
fn main() {

    let multiplicador:u8 = 5;

    let mut contador:u8 = 1;

    while contador <= 10 {

        println!("{} x {} = {}", multiplicador, contador, multiplicador*contador);

        contador += 1;

    }

}

