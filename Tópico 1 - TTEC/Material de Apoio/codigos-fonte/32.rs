
// Função que divide os parâmetros, a / b.
// Se OK, devolve um f64 (o resultado).
// Se erro, devolve uma String.

fn divide(a: f64, b: f64) -> Result<f64, String> {

    if b == 0.0 {
        Err(String::from("Não é possível dividir por zero"))
    } else {
        Ok(a / b)
    }
}


fn main() {

    let resultado1 = divide(10.0, 2.0);
    let resultado2 = divide(5.0, 0.0);

    match resultado1 {
        Ok(valor) => println!("Resultado da divisão 10/2 : {}", valor),
        Err(erro) => println!("Erro: {}", erro),
    }

    match resultado2 {
        Ok(valor) => println!("Resultado da divisão 5/0 : {}", valor),
        Err(erro) => println!("Erro: {}", erro),
    }
}

