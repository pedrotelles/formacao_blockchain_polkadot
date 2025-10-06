fn main() {
    let x = 10;
    {
        let x = 20;
        println!("Dentro do escopo interno: x = {}", x);
    }
    println!("Fora do escopo interno: x = {}", x)

    // - Dentro do bloco {}, uma nova variável x é declarada. Ela oculta temporariamente (sombra) a variável x original.
    // - Essa nova x só existe dentro do bloco. Quando o bloco termina, ela é descartada.
    // - Fora do bloco, a variável x original volta a ser visível e seu valor permanece 10.

}
