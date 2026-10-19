
// Constante global 
// (substituída em tempo de compilação, não ocupa memória real)

const PI: f64 = 3.1415;


// Static global 
// (tem endereço fixo em memória, imutável por padrão)

static VALOR_FIXO: i32 = 100;


// Static mut global 
// (mutável, mas acesso exige unsafe)

static mut CONTADOR: i32 = 0;


fn main() {

    // ---------------- let ----------------
    let x = 10;        // variável local, imutável por padrão
    let mut y = 20;    // variável local mutável

    println!("let x = {}", x);
    println!("let mut y = {}", y);
    y += 1;
    println!("y após incremento = {}", y);

    // ---------------- const ----------------
    println!("const PI = {}", PI);

    // ---------------- static ----------------
    println!("static VALOR_FIXO = {}", VALOR_FIXO);

    // ---------------- static mut (unsafe) ----------------
    unsafe {
        CONTADOR += 1;
        println!("static mut CONTADOR = {}", CONTADOR);
    }

}

