use time::Instant;

pub fn run() {
    println!(
        "At 10 iters\nIterative took {} ns\nRecursive took {} ns\n\nAt 40 iters\nIterative took {} ns\nRecursive took {} ns\n",
        benchmark(fibo_iter, 10).whole_nanoseconds(),
        benchmark_special(fibo_rec, 10).whole_nanoseconds(),
        benchmark(fibo_iter, 40).whole_nanoseconds(),
        benchmark_special(fibo_rec, 40).whole_nanoseconds()
    )
}

fn benchmark(func: fn(usize) -> Vec<i64>, arg: usize) -> time::Duration {
    let measure = Instant::now();
    func(arg);
    return measure.elapsed();
}

fn benchmark_special(func: fn(usize, Option<Vec<i64>>) -> Vec<i64>, arg: usize) -> time::Duration {
    let measure = Instant::now();
    func(arg, None);
    return measure.elapsed();
}

fn fibo_iter(n: usize) -> Vec<i64> {
    let mut fibo_arr: Vec<i64> = vec![0,1];
    for i in 2..=n {
        fibo_arr.push(fibo_arr[i - 2] + fibo_arr[i - 1]);
    }
    return fibo_arr;
}

fn fibo_rec(n: usize, previous: Option<Vec<i64>>) -> Vec<i64> {
    let mut fibo_arr = previous.unwrap_or(vec![0, 1]);
    fibo_arr.push(fibo_arr[fibo_arr.len() - 2] + fibo_arr[fibo_arr.len() - 1]);
    
    if fibo_arr.len() >= n {
        return fibo_arr;
    }

    return fibo_rec(n, Some(fibo_arr));
}