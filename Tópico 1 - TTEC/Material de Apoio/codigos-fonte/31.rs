
fn main() {

    let mut uma_string = String::from("IFSP");

    let ref1 = &uma_string;    // referência imutável

    let ref_mut = &mut uma_string; // ❌ erro: não pode misturar com imutável

    println!("{}", ref1);

}

