// QUESTÃO 1- Você está desenvolvendo uma aplicação em Rust que precisa realizar
//  análises em dados numéricos armazenados em um array. Para isso, você deve implementar
//  funções que manipulam esses dados. A tarefa consiste em criar duas funções específicas:
//  Função media_positivos: que recebe como entrada um array de inteiros (tamanho fixo
//  de 10 elementos) e retorna a média APENAS dos números positivos presentes nesse array.
//  Se não houver números positivos, a função deve retornar None.
//  Obs: https://doc.rust-lang.org/std/option/index.html
//  Função produto_pares: que recebe como entrada um array de inteiros (tamanho fixo de
//  10 elementos) e retorna o produto de todos os números pares presentes nesse array. Se
//  não houver números pares, a função deve retornar 1.
fn main() {
    let numbers = [2, -3, 7, 0, 8, -1, 5, -4, 6, 10];

    match positive_average(&numbers) {
        Some(average) => println!("Média dos números positivos: {}", average),
        None => println!("Não há números positivos."),
    }

    let product = even_product(&numbers);
    println!("Produto dos números pares: {}", product);
}

fn positive_average(numbers: &[i32]) -> Option<f64> {
    let mut sum = 0;
    let mut count = 0;

    for &number in numbers {
        if number > 0 {
            sum += number;
            count += 1;
        }
    }

    if count > 0 {
        Some(sum as f64 / count as f64)
    } else {
        None
    }
}

fn even_product(numbers: &[i32]) -> i32 {
    let mut product = 1;
    for &number in numbers {
        if number % 2 == 0 && number > 0 {
            product *= number;
        }
    }
    println!("{}", product);
    if product > 1 {
       product
    }else {
        1
    }
}
