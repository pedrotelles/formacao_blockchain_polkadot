const PI: f64 = 3.1415;
const GRAVIDADE: f32 = 9.8;

fn main() {
    let raio = 10.0;
    let area = PI * raio * raio;
    println!("Área do círculo com raio {}: {}", raio, area);
    // GRAVIDADE = 3.0; // Isso causará um erro de compilação E0070
}
