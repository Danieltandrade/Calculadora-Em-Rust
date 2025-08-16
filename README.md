# Calculadora em Rust

[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![GitHub](https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white)](https://github.com/Danieltandrade/Calculadora-Em-Rust)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://github.com/Danieltandrade/Calculadora-Em-Rust/blob/main/LICENSE)

Uma calculadora simples desenvolvida em **Rust**, que realiza operações básicas: soma, subtração, multiplicação e divisão. O projeto é modular, utilizando um módulo separado (`math_ops`) para as operações matemáticas, e inclui testes unitários para validação das funções.

---

## Estrutura do Projeto 🗃️

    ```bash
    calculadora/
    ├── .gitignore
    ├── Cargo.lock # Arquivo de lock
    ├── Cargo.toml
    ├── LICENSE
    ├── README.md
    └── src/
        ├── main.rs # Arquivo principal com interface de interação
        └── math_ops.rs # Módulo com as operações matemáticas    
    ```

- **main.rs**: Contém a interface principal da calculadora, loop de interação com o usuário e chamadas para o módulo de operações.  
- **math_ops.rs**: Define funções para soma, subtração, multiplicação, divisão e a função `calculo_operacao` que processa a escolha do usuário.  
- **Testes**: Incluídos em `main.rs` para validar cada função matemática.

---

## Instalação do Rust 💻

Para instalar o Rust, visite [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/pt-BR/tools/install) e siga os passos para instalação de acordo com seu sistema operacional.

> [!WARNING]
> Caso você for instalar o Rust em um ambiente Windows, ele requer o [Visual C++ Build Tools](https://visualstudio.microsoft.com/pt-br/vs/older/visual-cpp-build-tools/) para ser compilado corretamente.

---

## Funcionalidades 📑	

- Operações suportadas:
  - ➕ Soma
  - ➖ Subtração
  - ✖️ Multiplicação
  - ➗ Divisão (com tratamento de divisão por zero)
- Menu interativo com opção de sair (`0`)
- Entrada de dados via terminal
- Testes unitários para todas as funções matemáticas

---

## Como Executar 🔧

1. Clone o projeto:

    ```bash
    git clone https://github.com/Danieltandrade/Calculadora-Em-Rust.git
    cd calculadora
    ```

2. Compile e execute:

    ```bash
    cargo run
    ```

3. Siga as instruções no terminal para escolher a operação e informar os números:

    ```rust
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
        Running `target\debug\Calculadora-Em-Rust.exe`
    === Calculadora ===
    Escolha a operação:
    1 - Soma
    2 - Subtração
    3 - Multiplicação
    4 - Divisão
    0 - Sair
    Digite a operação desejada:
    3
    Digite o primeiro número:
    5
    Digite o segundo número:
    6
    Resultado: 30
    ```

4. Executando Testes:
 
    ```bash
    cargo test
    ```

    ```rust
        Finished `test` profile [unoptimized + debuginfo] target(s) in 0.02s
         Running unittests src\main.rs (target\debug\deps\Calculadora_Em_Rust-55449d4d3e4164e9.exe)

    running 4 tests
    test tests::test_multiplicacao ... ok
    test tests::test_divisao ... ok      
    test tests::test_soma ... ok
    test tests::test_subtracao ... ok

    test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    ```
---

## Tecnologias Utilizadas ⚙️

- Linguagem: Rust
- Ferramentas: Cargo (gerenciador de pacotes e compilação)
- Modularização com módulos internos
- Testes com framework de testes nativo do Rust

---

## Considerações Finais 📝

Este projeto serve como exemplo de:

- Estruturação de projetos Rust com modularização
- Uso de match para decisões de menu
- Entrada de dados via terminal
- Implementação de testes unitários para funções matemáticas

---

## Contribua 🤝

Se você deseja contribuir para o projeto, sinta-se a vontade para abrir uma issue ou enviar uma pull request. Agradecemos por suas contribuições!

---

## Licenca 📜

Este projeto segue a licenca [Apache-2.0](https://github.com/Danieltandrade/Calculadora-Em-Rust/blob/main/LICENSE).

---

## Conclusão 🎉

Este projeto foi meu primeiro contato com a linguagem de programação __RUST__, e foi um aprendizado muito interessante.
Mesmo explorando pouco as funcionalidades da linguagem, ainda aprendi bastante, e sou muito feliz com o resultado final.
Obrigado por visitar meu trabalho!
