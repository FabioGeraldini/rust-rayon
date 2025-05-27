use rayon::prelude::*;

fn main() {
    //Cria um vetor com números de 1 até 1.000.000
    let numbers: Vec<i32> = (1..=1_000_000).collect();

    // Usa Rayon para calcular a soma dos quadrados em paralelo
    let sum_of_squares: i64 = numbers
        .par_iter()
        .map(|&x| (x as i64) * (x as i64))
        .sum();
    println!("Soma dos quadrados: {}", sum_of_squares);

    // Filtra os números pares em paralelo e coleta em um vetor
    let even_numbers: Vec<i32> = numbers
        .par_iter()
        .filter(|&&x| x % 2 == 0)
        .cloned() // Converte de &i32 para i32
        .collect();
    println!("Quantidade números pares: {}", even_numbers.len());
} 
