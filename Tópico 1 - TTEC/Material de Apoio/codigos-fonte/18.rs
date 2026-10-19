
fn main() {

    let linguagem = "PHP";

    let proposito = match linguagem {

        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science"
    };

    println!("O proposito de {} eh {}", linguagem, proposito);
        
}

