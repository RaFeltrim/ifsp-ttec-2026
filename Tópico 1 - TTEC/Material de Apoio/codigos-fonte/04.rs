 
fn main() {

    let decimal:f32 = 2.5;
    println!("decimal = {}, tamanho={}", decimal, std::mem::size_of_val(&decimal));

    let booleana = false;
    println!("Tamanho booleana = {} bytes", std::mem::size_of_val(&booleana));

    let letra:char = 'C';
    println!("Tamanho do char = {} bytes", std::mem::size_of_val(&letra));
}
