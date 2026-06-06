pub fn modulos_e_pacotes() {
    println!("Fundamentos de Rust - Módulos e Pacotes");
    println!("Organização de código em módulos e pacotes em Rust");
    println!("Módulos: permitem organizar o código em unidades lógicas, usando a palavra-chave 'mod'");
    println!("Pacotes: são coleções de módulos, organizados em diretórios, usando a palavra-chave 'crate'");
    println!("Exemplos de módulos e pacotes:");
    println!("Exemplo de pacote em Rust:");
    println!("  Cargo.toml define o pacote e o nome do crate");
    println!("  src/main.rs ou src/lib.rs é a raiz do crate");
    println!("  Um pacote pode conter vários módulos como `fundamento::math`");
    
    // Criando um módulo chamado 'math'
    mod math {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        
        pub fn subtract(a: i32, b: i32) -> i32 {
            a - b
        }
    }

    // Usando as funções do módulo 'math'
    let sum = math::add(5, 3);
    let difference = math::subtract(5, 3);
    println!("Soma: {}", sum);
    println!("Subtração: {}", difference);

    println!("Exemplo de uso de um crate externo (rand):");
    println!("Adicione `rand = \"0.8\"` no Cargo.toml para usar o crate rand");
    println!("use rand::Rng; para importar o módulo Rng do crate rand");
    println!("Gerando um número aleatório entre 1 e 10:");
    // Exemplo de uso do crate rand (certifique-se de adicionar rand = "0.8" no Cargo.toml)
    /*
    use rand::Rng;  
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(1..=10);
    println!("Número aleatório gerado: {}", random_number);
    */  
    println!("Módulos podem ser aninhados, permitindo uma hierarquia de organização de código, e pacotes podem conter múltiplos módulos para estruturar projetos complexos.");
    println!("A palavra-chave `pub` é usada para tornar funções, structs ou módulos públicos, permitindo que sejam acessados de outros módulos ou pacotes.");
    println!("Além disso, o sistema de módulos e pacotes em Rust é integrado com o sistema de gerenciamento de dependências do Cargo, permitindo que os desenvolvedores facilmente adicionem e gerenciem dependências externas em seus projetos.");
   
    

    
}