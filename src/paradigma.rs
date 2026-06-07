pub fn programacao_orientada_a_objetos(){
    println!("Fundamentos de Rust - Programação Orientada a Objetos");
    println!("Rust suporta programação orientada a objetos através de structs e traits");
    println!("Structs: permitem criar tipos de dados personalizados com campos e métodos");
    println!("Traits: permitem definir comportamentos compartilhados entre diferentes tipos");
    println!("Encapsulamento: Rust não possui classes, mas é possível encapsular dados e comportamentos usando structs e traits");
    println!("Polimorfismo: traits permitem que diferentes tipos implementem o mesmo comportamento, permitindo polimorfismo em Rust");
    println!("Herança: Rust não suporta herança tradicional, mas é possível compartilhar código entre structs usando traits e composição");
    println!("Exemplo de struct e trait:");
    
    struct Pessoa {
        nome: String,
        idade: u32,
    }

    impl Pessoa {
        fn apresentar(&self) {
            println!("Olá, meu nome é {} e tenho {} anos.", self.nome, self.idade);
        }
    }

    let pessoa1 = Pessoa {
        nome: String::from("Alice"),
        idade: 30,
    };

    pessoa1.apresentar();
    println!("Traits podem ser usados para definir comportamentos compartilhados entre diferentes tipos:");
    trait Saudacao {
        fn saudacao(&self);
    }
    impl Saudacao for Pessoa {
        fn saudacao(&self) {
            println!("Bem-vindo, {}!", self.nome);
        }
    }
    pessoa1.saudacao();
}

pub fn programacao_funcional(){
    println!("Fundamentos de Rust - Programação Funcional");
    println!("Rust suporta programação funcional através de funções de ordem superior, closures e iteradores");
    println!("Funções de ordem superior: permitem passar funções como argumentos ou retorná-las como resultado");
    println!("Closures: são funções anônimas que podem capturar variáveis do ambiente onde foram definidas");
    println!("Iteradores: permitem processar coleções de dados de forma eficiente e expressiva");
    println!("Exemplo de função de ordem superior, closure e iterador:");
    
    fn aplicar_operacao<F>(a: i32, b: i32, operacao: F) -> i32
    where
        F: Fn(i32, i32) -> i32,
    {
        operacao(a, b)
    }

    let soma = |x, y| x + y; // Closure para soma
    let resultado = aplicar_operacao(5, 3, soma);
    println!("Resultado da soma: {}", resultado);

    let numeros = [1, 2, 3, 4, 5];
    let quadrados: Vec<i32> = numeros.iter().map(|x| x * x).collect(); // Usando iterador para calcular quadrados
    println!("Quadrados dos números: {:?}", quadrados);
}