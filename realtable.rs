use rand::{distributions::Uniform, prelude::ThreadRng, Rng};
use table::Table;

pub fn run() {
    let mut a1: Table = Table::new();

    let mut rng: ThreadRng = rand::thread_rng();
    let distr: Uniform<i64> = Uniform::from(0..=99);

    for i in 0..1000 {
        a1.insert(i, rng.sample(distr) as f64);
    }

    /*
    for j in 0..a1.len() {
        println!("{:?}", a1.get::<_, f64>(j).unwrap());
    }
    */

    let mean = get_mean(a1.clone());
    let var = get_variance(a1.clone(), mean);
    let stdev = get_stdev(a1.clone(), mean);

    println!(
        "Table, column1 len(): {}\nMean: {}\nVariance: {}\nStdDeviation: {}\n",
        a1.len(),
        mean,
        var,
        stdev
    );
}

fn get_mean(table: Table) -> f64 {
    let mut sum: f64 = 0.0;
    let size: f64 = table.len() as f64;

    for i in 0..table.len() {
        sum += table.get::<_, f64>(i).unwrap();
    }

    return (sum / size) as f64;
}

fn get_variance(table: Table, mean: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let size: f64 = table.len() as f64;

    for i in 0..table.len() {
        sum += (table.get::<_, f64>(i).unwrap() - mean) * (table.get::<_, f64>(i).unwrap() - mean);
    }

    return (sum / size) as f64;
}

fn get_stdev(table: Table, mean: f64) -> f64 {
    return get_variance(table, mean).sqrt();
}
