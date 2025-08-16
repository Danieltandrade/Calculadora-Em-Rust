use std::io;

// Importando o módulo de operações matemáticas
mod math_ops;

fn main() {
    loop {
        println!("=== Calculadora ===");
        println!("Escolha a operação:");
        println!("1 - Soma");
        println!("2 - Subtração");
        println!("3 - Multiplicação");
        println!("4 - Divisão");
        println!("0 - Sair");

        // Lê a escolha do usuário
        println!("Digite a operação desejada: ");
        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).expect("Erro na leitura");
        let escolha: i32 = escolha.trim().parse().expect("Digite um número válido!");

        // Verifica se o usuário deseja sair
        if escolha == 0 {
            println!("Saindo da calculadora...");
            break;
        
        } else {
            // Chama a função de cálculo com a escolha do usuário
            math_ops::calculo_operacao(escolha);
        }

        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soma() {
        assert_eq!(math_ops::soma(2.0, 3.0), 5.0);
        assert_eq!(math_ops::soma(-1.0, 1.0), 0.0);
    }

    #[test]
    fn test_subtracao() {
        assert_eq!(math_ops::subtracao(5.0, 3.0), 2.0);
        assert_eq!(math_ops::subtracao(3.0, 5.0), -2.0);
    }

    #[test]
    fn test_multiplicacao() {
        assert_eq!(math_ops::multiplicacao(4.0, 2.0), 8.0);
        assert_eq!(math_ops::multiplicacao(-2.0, 3.0), -6.0);
    }

    #[test]
    fn test_divisao() {
        assert_eq!(math_ops::divisao(10.0, 2.0), Some(5.0));
        assert_eq!(math_ops::divisao(7.0, 2.0), Some(3.5));
        assert_eq!(math_ops::divisao(5.0, 0.0), None);
    }
}
