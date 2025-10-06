fn main() {
    // 2. Declaração das variáveis com tipos explícitos

    let mut idade: i32 = 25; // 5. idade é mutável

    let peso: f64 = 70.5;
    let ativo: bool = true;

    println!("Idade: {}", idade);
    println!("Peso: {}", peso);
    println!("Ativo: {}", ativo);

    idade = 30; // 5. Modifica o valor de idade
    println!("Nova Idade: {}", idade);
}
