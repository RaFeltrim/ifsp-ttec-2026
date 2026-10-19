
// Criando uma variável global:
static GLOBAL:i16 = 999;

fn main() {

    println!("Valor de GLOBAL em main: {}", GLOBAL);

    // Variável local 'a':
    let a = 111;
    println!("Valor de 'a' em main: {}", a);

    // Variável local 'b':
    let b = 222;
    println!("Valor de 'b' em main, antes de chamar a fn qualquer: {}", b);

    qualquer();

    println!("Valor de 'b' em main, quando voltou da fn qualquer: {}", b);

}

fn qualquer() {

    println!("Entrei na funcao qualquer");
    println!("Valor de GLOBAL em qualquer: {}", GLOBAL);

    // Se descomentar a linha abaixo, dá erro de compilação,
    // pois a variável 'a' é local em main:
    // println!("Valor de 'a' em qualquer: {}", a);

    // Criando outra variável local 'b':
    let b = 333;
    println!("Valor de 'b' criado em qualquer: {}", b);

    // Criando um escopo com chaves:

    {
        // Criando outra variável local 'b':
        let b = 444;
        println!("Valor de 'b' no escopo extra em qualquer: {}", b);
    }

    println!("Valor de 'b' depois que saiu do escopo extra: {}", b);
}

