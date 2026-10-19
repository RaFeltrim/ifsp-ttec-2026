
fn main() {
    let variavel = 300;
    println!("variavel={}, tamanho={} bytes", variavel, std::mem::size_of_val(&variavel));
}
