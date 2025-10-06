fn main() {
    let dados: String = String::from("Segurança");
    imprimir_dados(&dados);
}

// - Em Rust, quando você passa uma String para uma função por valor, a posse da variável é movida.
// - Isso significa que a variável original (dados) não pode mais ser usada após a chamada da função.
// - O compilador impede o uso para evitar acesso inválido à memória — uma das formas como Rust garante segurança

// fn imprimir_dados(dados: String) {
//     println!("Dados: {}", dados);
// }

// - Ao usar &dados, estamos emprestando a variável para a função.
// - A função pode ler os dados, mas não pode modificá-los nem tomar posse.
// - Isso evita duplicação de memória e garante que o valor original continue válido.   
fn imprimir_dados(dados: &String) {
    println!("Dados: {}", dados);
}
