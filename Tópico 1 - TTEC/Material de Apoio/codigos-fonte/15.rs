
fn main() {

    let multiplicador:u8 = 5;

    let mut contador:u8 = 0;

    loop {

        contador += 1;

        println!("{} x {} = {}", multiplicador, contador, multiplicador*contador);

        if contador == 10 {
            break;
        }
    }
}

