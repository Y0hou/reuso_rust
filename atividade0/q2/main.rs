// QUESTÃO 2- Escreva uma função chamada analisar_tupla que recebe uma tupla
// (i32, i32, i32) como argumento e retorna outra tupla com três elementos:
// 1. Asomados três números.
// 2. Omaior número entre os três.
// 3. Omenor número entre os três.
fn main() {
    let values = (10, 6, 5);
    let result = analisar_tupla(values);

    println!("somar: {}\nmaior: {}\nmenor: {}", result.0, result.1, result.2);
}

fn analisar_tupla(tupla: (i32, i32, i32)) -> (i32, i32, i32) {
    let (a, b, c) = tupla;

    let somar = a + b + c;
    let mut maior = a;
    let mut menor = a;

    for &elem in &[a, b, c] {
        if elem > maior {
            maior = elem;
        }
        if elem < menor {
            menor = elem;
        }
    }

    (somar, maior, menor)
}
