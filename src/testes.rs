pub  fn testes(){
    println!("Fundamentos de Rust - Testes");
    println!("Testes são uma parte essencial do desenvolvimento de software, garantindo que o código funcione corretamente e seja confiável");
    println!("Rust possui um sistema de testes integrado, permitindo escrever testes unitários, testes de integração e testes de documentação");
    println!("Testes unitários: testam funções ou módulos isolados usando a anotação #[test]");
    println!("Testes de integração: testam a interação entre múltiplos módulos ou componentes usando arquivos separados na pasta tests/");
    println!("Testes de documentação: testam exemplos incluídos na documentação usando a anotação /// e #[doc = \"\"]");
    println!("Rust também suporta testes paralelos, permitindo executar múltiplos testes ao mesmo tempo para acelerar o processo de teste");
    println!("Exemplo de teste unitário:");
    
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn teste_soma() {
            assert_eq!(2 + 2, 4);
        }

        #[test]
        fn teste_subtracao() {
            assert_eq!(5 - 3, 2);
        }
    }
    println!("Exemplo de teste de integração:");
    println!("Crie um arquivo tests/integration_test.rs com o seguinte conteúdo:");
    println!("
    use fundamento::math;
    let resultado = math::add(2, 3);
    assert_eq!(resultado, 5);
    ");
    println!("Exemplo de teste de documentação:");
    println!("Inclua o seguinte exemplo na documentação de uma função:");
    println!("
    /// Soma dois números
    /// 
    /// # Exemplo
    /// 
    /// ```
    /// let resultado = somar(2, 3);
    /// assert_eq!(resultado, 5);
    /// ```
    fn somar(a: i32, b: i32) -> i32 
        a + b
    
    ");
    println!("Para rodar os testes, use o comando `cargo test` no terminal. O Cargo irá compilar o código de teste e executar todos os testes definidos, mostrando os resultados no console.");
    println!("Testes são fundamentais para garantir a qualidade do código e facilitar a manutenção, permitindo identificar e corrigir bugs de forma eficiente. Rust incentiva a escrita de testes desde o início do desenvolvimento, promovendo uma cultura de testes robusta e confiável.");

}