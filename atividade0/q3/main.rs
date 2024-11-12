//  1. Solicite ao usuário que insira três números inteiros.
//  2. Crie uma tupla com esses números.
//  3. Chame a função analisar_tupla e exiba os resultados da tupla retornada.
use std::io;

fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada.");
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|n| n.parse().expect("Erro ao converter número."))
        .collect();
    if numbers.len() == 3 {
        let values = (numbers[0], numbers[1], numbers[2]);
        let result = analisar_tupla(values);
        println!("soma: {}\nmaior: {}\nmenor: {}", result.0, result.1, result.2);
    } else {
        println!("Digite os 3 numeros")
    }

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
