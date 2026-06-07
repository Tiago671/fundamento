pub fn concorrencia(){
    println!("Fundamentos de Rust - Concorrência");
    println!("Concorrência é a capacidade de um programa executar múltiplas tarefas ao mesmo tempo");
    println!("Rust oferece suporte à concorrência através de threads, async/await e canais de comunicação");
    println!("Threads: permitem executar código em paralelo usando std::thread::spawn");
    println!("Async/await: permitem escrever código assíncrono de forma mais fácil usando async fn e .await");
    println!("Canais: permitem comunicação segura entre threads usando std::sync::mpsc (multi-producer, single-consumer)");
    println!("Rust garante segurança de memória e ausência de condições de corrida através do sistema de propriedade e empréstimo, mesmo em código concorrente");
    println!("Exemplo de uso de threads:");
    
    use std::thread;
    use std::time::Duration;

    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 1..5 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(300));
    }

    handle.join().unwrap(); // Aguarda a thread terminar
}