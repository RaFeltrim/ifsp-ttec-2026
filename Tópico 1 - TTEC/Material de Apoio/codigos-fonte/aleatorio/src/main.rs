use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let numero: i32 = rng.gen_range(1..=100);
    println!("Número aleatório gerado: {}", numero);
}

