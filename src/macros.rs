
pub fn macros(){
    println!("Fundamentos de Rust - Macros");
    println!("Macros são uma forma de metaprogramação em Rust, permitindo gerar código em tempo de compilação");
    println!("Tipos de macros: declarativas (macro_rules!), procedurais (proc_macro) e derive macros");
    println!("Macros declarativas: definidas usando macro_rules!, permitem criar regras de substituição para gerar código");
    println!("Macros procedurais: definidas usando proc_macro, permitem criar macros mais complexas que podem manipular o código de entrada de forma mais flexível");
    println!("Derive macros: permitem gerar implementações de traits para structs e enums de forma automática usando #[derive]");
    println!("Macros podem ser usadas para criar código repetitivo, implementar padrões de design ou criar DSLs (Domain-Specific Languages)");
    println!("Exemplo de macro simples:");
    
    macro_rules! saudacao {
        ($nome:expr) => {
            println!("Olá, {}!", $nome);
        };
    }

    saudacao!("Alice");
    saudacao!("Bob");

    #[derive(Debug, Clone)]
    struct MinhaStruct {
        campo1: i32,
        campo2: String,
    }
    println!("Exemplo de derive macro para gerar implementação de Debug e Clone:");
    let minha_struct = MinhaStruct {
        campo1: 42,
        campo2: String::from("Olá"),
    };
    println!("{:?}", minha_struct);
    println!("{:?}", minha_struct.clone());
}