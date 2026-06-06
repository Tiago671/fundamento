pub fn propriedade_e_emprestimo(){
    println!("Fundamentos de Rust - Propriedade e Empréstimo");
    println!("Conceitos de propriedade, empréstimo e referências em Rust");
    println!("Regras de propriedade: cada valor tem um proprietário, apenas um proprietário por vez, quando o proprietário sai de escopo, o valor é descartado");
    println!("Empréstimo: permite que uma função use um valor sem tomar posse dele, usando referências (&)");
    println!("Referências mutáveis: permitem modificar um valor emprestado, mas apenas uma referência mutável por vez");
    println!("Exemplos de propriedade e empréstimo:");
    
    let s1 = String::from("Olá"); // s1 é o proprietário da string
    let s2 = &s1; // s2 é uma referência a s1 (empréstimo)
    println!("s1: {}, s2: {}", s1, s2); // Ambos podem ser usados

    let mut s3 = String::from("Mundo"); // s3 é o proprietário da string
    let  s4 = &mut s3; // s4 é uma referência mutável a s3 (empréstimo)
    println!("s4: {}", s4); // Ambos podem ser usados
    *s4 = String::from("Rust"); // Modificando o valor através da referência mutável
    println!("s3 após modificação: {}", s3); // O valor de s3 foi modificado

}