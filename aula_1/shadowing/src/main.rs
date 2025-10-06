fn runComShadowing(){
    let num = 5;
    let num = num + 3;   // shadowing: novo num = 8
    let num = num * 2;   // shadowing: novo num = 16

    println!("Valor final com shadowing: {}", num);
}

fn runComMut(){
    let mut num = 5;
    num = num + 3;   // mut: mesmo num = 8
    num = num * 2;   // mut: mesmo num = 16

    println!("Valor final com mut: {}", num);
}

fn main() {
    // Shadowing permite redeclarar a variável com o mesmo nome, criando uma nova variável.
    runComShadowing();
    // Mutabilidade permite alterar o valor da mesma variável.
    runComMut();
}
    