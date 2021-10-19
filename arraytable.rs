use rand::{distributions::Uniform, Rng};

pub fn run() {
    let a1 = get_vector(1000);
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

fn get_vector(size: i64) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let distr = Uniform::from(0..=99);
    let mut vect: Vec<i64> = vec![];

    for _ in 0..size {
        vect.push(rng.sample(distr));
    }

    return vect;
}

fn get_mean(table: Vec<i64>) -> f64 {
    let mut sum: f64 = 0.0;
    let size: f64 = table.len() as f64;

    for i in table {
        sum += i as f64;
    }

    return (sum / size) as f64;
}

fn get_variance(table: Vec<i64>, mean: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let size: f64 = table.len() as f64;

    for i in table {
        sum += ((i as f64) - mean) * ((i as f64) - mean);
    }

    return (sum / size) as f64;
}

fn get_stdev(table: Vec<i64>, mean: f64) -> f64 {
    return get_variance(table, mean).sqrt();
}
