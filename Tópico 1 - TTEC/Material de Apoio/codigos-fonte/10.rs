
fn soma( a:i32, b:i32 ) -> i32
{
    println!( "{} + {} = {}", a, b, a + b );
    a + b   // sem o ; é uma expressão, está retornando este valor
}

fn main() {

    println!( "Soma = {}", soma(3, 7) );

}

