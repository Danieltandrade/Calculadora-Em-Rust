use std::io;

/// Soma dois números
pub fn soma(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtrai dois números
pub fn subtracao(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiplica dois números
pub fn multiplicacao(a: f64, b: f64) -> f64 {
    a * b
}

/// Divide dois números (retorna `None` se divisor for zero)
pub fn divisao(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

pub fn calculo_operacao(escolha: i32) {
    // Primeiro número
    println!("Digite o primeiro número:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Erro na leitura");
    let num1: f64 = input1.trim().parse().expect("Digite um número válido!");

    // Segundo número
    println!("Digite o segundo número:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Erro na leitura");
    let num2: f64 = input2.trim().parse().expect("Digite um número válido!");

    // Usa match para decidir a operação
    match escolha {
        1 => println!("Resultado: {}", soma(num1, num2)),
        2 => println!("Resultado: {}", subtracao(num1, num2)),
        3 => println!("Resultado: {}", multiplicacao(num1, num2)),
        4 => match divisao(num1, num2) {
            Some(resultado) => println!("Resultado: {}", resultado),
            None => println!("Erro: divisão por zero!"),
            },
        _ => println!("Opção inválida!"),
        }
}
