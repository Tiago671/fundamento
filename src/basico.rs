
pub fn variaveis_e_tipos(){
    println!("Fundamentos de Rust - Variáveis e Tipos de Dados");
    println!("Interger ==> i8, i16, i32, i64, i128, isize");
    println!("Unsigned Integer ==> u8, u16, u32, u64, u128, usize");
    println!("Floating Point ==> f32, f64");
    println!("Boolean ==> bool");
    println!("Character ==> char");
    println!("String ==> String, &str");
    println!("Exemplos de variáveis e tipos de dados:");
    let mut x: i32 = 5; // Variável mutável do tipo inteiro
    println!("Valor i32 de x mutável: {}", x);
    x = 10; // Modificando o valor de x
    println!("Valor i32 de x após modificação: {}", x);

    let y: f64 = 3.145; // Variável imutável do tipo float
    println!("Valor f64 de y: {}", y);

    let z: bool = true; // Variável do tipo booleano
    println!("Valor bool de z: {}", z);

    let nome: &str = "Rust"; // Variável do tipo string
    println!("Valor &str de nome: {}", nome);

    let mut nome_completo: String = String::from("João"); // Variável mutável do tipo String
    println!("Valor String de nome_completo: {}", nome_completo);
    nome_completo.push_str(" Silva"); // Modificando o valor de nome_completo
    println!("Valor String de nome_completo após modificação: {}", nome_completo);

}

pub fn operadores(){
    println!("Fundamentos de Rust - Operadores");
    println!("Operadores Aritméticos: +, -, *, /, %");
    println!("Operadores de Atribuição: =, +=, -=, *=, /=, %=");
    println!("Operadores de Comparação: ==, !=, >, <, >=, <=");
    println!("Operadores Lógicos: &&, ||, !");
    println!("Exemplos de operadores:");
    let a = 10;
    let b = 5;

    // Operadores Aritméticos
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
    println!("a % b = {}", a % b);

    // Operadores de Atribuição
    let mut c = 10;
    c += 5; // c = c + 5
    println!("c += 5 => c = {}", c);
    c -= 3; // c = c - 3
    println!("c -= 3 => c = {}", c);
    c *= 2; // c = c * 2
    println!("c *= 2 => c = {}", c);
    c /= 4; // c = c / 4
    println!("c /= 4 => c = {}", c);
    c %= 3; // c = c % 3
    println!("c %= 3 => c = {}", c);

    // Operadores de Comparação
    println!("a == b => {}", a == b);
    println!("a != b => {}", a != b);
    println!("a > b => {}", a > b);
    println!("a < b => {}", a < b);
    println!("a >= b => {}", a >= b);
    println!("a <= b => {}", a <= b);
    
    // Operadores Lógicos
    let x = true;
    let y = false;
    println!("x && y => {}", x && y);
    println!("x || y => {}", x || y);
    println!("!x => {}", !x);
    println!("!y => {}", !y);

}

pub fn controle_de_fluxo(){
    println!("Fundamentos de Rust - Controle de Fluxo");
    println!("Estruturas de controle de fluxo: if, else, else if, match, loop, while, for");
    println!("Exemplos de controle de fluxo:");
    
    let numero = 10;

    // Estrutura if
    if numero > 0 {
        println!("O número é positivo.");
    } else if numero < 0 {
        println!("O número é negativo.");
    } else {
        println!("O número é zero.");
    }

    // Estrutura match
    match numero {
        1 => println!("Número é um."),
        2 => println!("Número é dois."),
        3 => println!("Número é três."),
        _ => println!("Número é diferente de um, dois e três."),
    }

    // Estrutura loop
    let mut contador = 0;
    loop {
        contador += 1;
        if contador > 5 {
            break; // Sai do loop quando contador for maior que 5
        }
        println!("Contador: {}", contador);
    }

    // Estrutura while
    let mut contador_while = 0;
    while contador_while < 5 {
        contador_while += 1;
        println!("Contador while: {}", contador_while);
    }

    // Estrutura for
    for i in 0..5 { // Itera de 0 a 4
        println!("Contador for: {}", i);
    }
}

pub fn funcoes(){
    println!("Fundamentos de Rust - Funções");
    println!("Definição de funções, parâmetros, retorno, funções anônimas (closures)");
    println!("Exemplos de funções:");
    
    // Função simples
    fn saudacao() {
        println!("Olá, seja bem-vindo ao Rust!");
    }
    saudacao();

    // Função com parâmetros
    fn soma(a: i32, b: i32) -> i32 {
        a + b
    }
    let resultado = soma(5, 3);
    println!("Resultado da soma: {}", resultado);

    // Função anônima (closure)
    let multiplicacao = |x: i32, y: i32| -> i32 {
        x * y
    };
    let resultado_multiplicacao = multiplicacao(4, 6);
    println!("Resultado da multiplicação: {}", resultado_multiplicacao);
}