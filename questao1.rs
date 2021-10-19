use num::complex::Complex;
use rand::{distributions::Uniform, Rng};
use std::time::{Duration, Instant};

#[derive(Debug)]
enum Algoritmo {
    Bubble,
    Insertion,
    Merge,
    Quick,
    Heap,
}

pub fn run() {
    soma_assina("Gonçalo Candeias Amaro"); // Devolve Insertion
    sort_complex();
    //reverse_sort();
    //erro_relativo(10);
    //erro_relativo(20);
}

fn soma_assina(nome: &str) {
    let mut soma = 0;
    for x in nome.chars() {
        let v = match x.to_digit(16) {
            Some(e) => e,
            None => 0,
        };
        if x.is_ascii_whitespace() {
            soma += 1;
        } else if x.is_ascii_lowercase() {
            soma += 2;
        }
        soma += v;
    }
    const N: u32 = 5;
    let diva = soma % N;
    let escolha = match diva {
        1 => Algoritmo::Bubble,
        2 => Algoritmo::Heap,
        3 => Algoritmo::Insertion,
        4 => Algoritmo::Merge,
        0 => Algoritmo::Quick,
        _ => Algoritmo::Bubble,
    };

    println!("algoritmo a implementar -> {:?} Sort", escolha);
}

fn get_vector(size: usize) -> Vec<Complex<f64>> {
    let mut rng = rand::thread_rng();
    let distr = Uniform::from(0.0..=99.9);
    let mut vect: Vec<Complex<f64>> = vec![];

    for _ in 0..size {
        vect.push(Complex::new(rng.sample(distr), rng.sample(distr)));
    }

    return vect;
}

fn sort_complex() {
    let table = get_vector(10);
    let mut normatable: Vec<f64> = vec![];

    for i in 0..table.len() {
        normatable.push(table[i].norm_sqr());
    }

    ignorant_printer(&normatable);

    for j in 2..normatable.len() {
        let key = normatable[j].clone();
        let mut i = j - 1;
        while i > 0 && normatable[i] > key {
            normatable[i + 1] = normatable[i].clone();
            i = i - 1;
        }
        normatable[i + 1] = key;
    }

    ignorant_printer(&normatable);
}

fn reverse_sort() {
    let table = get_vector(10);
    let mut realtable: Vec<f64> = vec![];

    for i in 0..table.len() {
        realtable.push(table[i].re);
    }

    ignorant_printer(&realtable);

    for j in 2..realtable.len() {
        let key = realtable[j].clone();
        let mut i = j - 1;
        while i > 0 && realtable[i] < key {
            realtable[i + 1] = realtable[i].clone();
            i = i - 1;
        }
        realtable[i + 1] = key;
    }

    ignorant_printer(&realtable);
}

fn benchmark(m: usize) -> Vec<u128> {
    let mut measure: Instant;
    let mut measured: Duration;
    let mut vec: Vec<u128> = vec![];
    for _ in 0..m {
        measure = Instant::now();
        sort_complex();
        measured = measure.elapsed();
        println!("{} ns\n", measured.as_nanos());
        vec.push(measured.as_nanos());
    }
    return vec;
}

fn erro_relativo(m: usize) {
    let a1 = benchmark(m);
    let mean = get_mean(a1.clone());
    let var = get_variance(a1.clone(), mean);
    let stdev = get_stdev(a1.clone(), mean);

    println!(
        "Arr len(): {}\nMean: {}\nVariance: {}\nStdDeviation: {}\n",
        a1.len(),
        mean,
        var,
        stdev
    );
}

fn get_stdev(table: Vec<u128>, mean: f64) -> f64 {
    return get_variance(table, mean).sqrt();
}

fn get_mean(table: Vec<u128>) -> f64 {
    let mut sum: f64 = 0.0;
    let size: f64 = table.len() as f64;

    for i in table {
        sum += i as f64;
    }

    return (sum / size) as f64;
}

fn get_variance(table: Vec<u128>, mean: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let size: f64 = table.len() as f64;

    for i in table {
        sum += ((i as f64) - mean) * ((i as f64) - mean);
    }

    return (sum / size) as f64;
}

fn ignorant_printer(a: &Vec<f64>) {
    // Ignoramos o primeiro item na ordenação como pedido, logo não é preciso mostrar
    print!(" iteração 1 {}", a[1]);
    for i in 2..a.len() {
        print!(", iteração {} {}",i, a[i]);
    }
    println!("\n");
}
