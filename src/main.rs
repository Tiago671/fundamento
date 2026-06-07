mod basico;
mod ownership;
mod mods;
mod paradigma;
mod macros;
use std::io;
fn main() {
    
    let mut op = String::new();
    println!("Fundamentos de Rust");
    println!("Escolha um tópico para aprender: ");
    println!("1 - Variáveis e Tipos de Dados");
    println!("2 - Operadores");
    println!("3 - Controle de Fluxo");
    println!("4 - Funções");
    println!("5 - Estruturas de Dados");
    println!("6 - Módulos e Pacotes");
    println!("7 - Propriedade e Empréstimo");
    println!("8 - Concorrência");
    println!("9 - Manipulação de Arquivos");
    println!("10 - Programação Orientada a Objetos");
    println!("11 - Programação Funcional");
    println!("12 - Macros");
    println!("13 - Testes");
    println!("14 - Ferramentas e Ecossistema");
    println!("15 - Sair");
    println!("Digite o número correspondente ao tópico desejado: ");
    io::stdin().read_line(&mut op).expect("Failed to read line");
    let op = op.trim().parse::<i32>().expect("Failed to parse number");
        match op {
            1 => basico::variaveis_e_tipos(),
            2 => basico::operadores(),
            3 => basico::controle_de_fluxo(),
            4 => basico::funcoes(),
            5 => ownership::propriedade_e_emprestimo(),
            6 => mods::modulos_e_pacotes(),
            7 => paradigma::programacao_funcional(),
            8 => paradigma::programacao_orientada_a_objetos(),
            12 => macros::macros(),
            /*5 => estruturas_de_dados(),
            8 => concorrencia(),
            9 => manipulacao_de_arquivos(),
            ,
            

            13 => testes(),
            14 => ferramentas_e_ecossistema(),*/
            15 => println!("Saindo..."),
            _ => println!("Opção inválida"),
        }
    /*let mut num1 = String::new();

    println!("Digite um número:");
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1 = num1.trim().parse::<i32>().expect("Failed to parse number");

    let mut op1 = String::new();
    println!("qual operação deseja realizar? (1 - Soma, 2 - Subtração, 3 - Multiplicação, 4 - Divisão)");
    io::stdin().read_line(&mut op1).expect("Failed to read line");
    let op1 = op1.trim().parse::<i32>().expect("Failed to parse number");

    let mut num2 = String::new();
    println!("Digite um número:");
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2 = num2.trim().parse::<i32>().expect("Failed to parse number");

    let resultado = calcular(num1, num2, op1);
    println!("Resultado: {}", resultado);


}

fn calcular(num1: i32, num2: i32, op: i32) -> i32 {
    match op {
        1 => num1 + num2,
        2 => num1 - num2,
        3 => num1 * num2,
        4 => num1 / num2,
        _ => 0,
    }*/
}