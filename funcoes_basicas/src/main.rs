// fn soma(a: i32, b: i32) -> i32 {
//     a + b
// }
fn soma(a: i32, b: i32) -> i64 {
    // Quando mudamos o tipo de retorno para i64, precisamos garantir que os parâmetros também sejam convertidos para esse tipo.
    // Isso é necessário porque Rust não faz conversão automática entre tipos numéricos — isso evita erros sutis e garante segurança.
    (a + b) as i64
}
fn main() {
    // Caso com i32
    // let resultado = soma(5, 3);
    // println!("Resultado da soma: {}", resultado);
    let resultado = soma(5, 3);
    println!("Resultado da soma: {}", resultado);
}
