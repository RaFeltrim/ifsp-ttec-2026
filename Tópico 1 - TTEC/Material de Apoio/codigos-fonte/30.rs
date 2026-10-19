// Função que recebe referência imutável e imprime

fn exibe_ref(s: &String) {
    println!("{}", s);
}

fn main() {

    // Variável imutável
    let uma_string = String::from("IFSP");

    // Variável mutável
    let mut outra_string = String::from("Campus São Carlos");
    outra_string.push_str(" - SP");

    // Duas referências imutáveis para a variável imutável
    let ref1 = &uma_string;
    let ref2 = &uma_string;

    // Duas referências imutáveis para a variável mutável
    let ref3 = &outra_string;
    let ref4 = &outra_string;

    // Chamando a função para cada referência
    exibe_ref(ref1);
    exibe_ref(ref2);
    exibe_ref(ref3);
    exibe_ref(ref4);

    println!("Podemos usar as variáveis originais normalmente:");
    println!("uma_string: {}", uma_string);
    println!("outra_string: {}", outra_string);

}

