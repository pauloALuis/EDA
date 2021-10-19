use rand::{distributions::Uniform, Rng};

pub fn run() {
    // Fuck your tables
    let a1 = get_vector(10).clone();
    let a4 = get_vector(40).clone();

    println!("{:?}\n{:?}\n", a1, a4);
    println!("{:?}\n{:?}\n", insertion_sort(&a1), insertion_sort(&a4));
    println!("{:?}\n{:?}\n", bubble_sort(&a1), bubble_sort(&a4));
    println!("{:?}\n{:?}\n", selection_sort(&a1), selection_sort(&a4));
    println!("{:?}\n{:?}\n", merge_sort(&a1), merge_sort(&a4));
    println!("{:?}\n{:?}\n", quick_sort(&a1), quick_sort(&a4));
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

fn bubble_sort(arg: &Vec<i64>) -> Vec<i64> {
    let mut table = arg.clone();
    let mut n: i64 = table.len() as i64;

    while n >= 1 {
        let mut newn: i64 = 0;
        for i in 1..table.len() {
            if table[i - 1] > table[i] {
                table.swap(i - 1, i);
                newn = i as i64;
            }
        }
        n = newn;
    }

    return table;
}

fn insertion_sort(arg: &Vec<i64>) -> Vec<i64> {
    let mut table = arg.clone();

    for i in 1..table.len() {
        let mut j = i;
        while j > 0 && table[j - 1] > table[j] {
            table.swap(j - 1, j);
            j -= 1;
        }
    }

    return table;
}

fn selection_sort(arg: &Vec<i64>) -> Vec<i64> {
    let mut table = arg.clone();

    for i in 0..table.len() {
        let mut min = i;

        for j in (i)..table.len() {
            if table[j] < table[min] {
                min = j;
            }
        }

        table.swap(min, i);
    }

    return table;
}

fn merge_sort(arg: &Vec<i64>) -> Vec<i64> {
    fn merge(arg: &Vec<i64>, left: &Vec<i64>, right: &Vec<i64>) -> Vec<i64> {
        if arg.len() == left.len() + right.len() {
            let mut table = arg.clone();
            let mut i = 0;
            let mut j = 0;
            let mut k = 0;

            while i < left.len() && j < right.len() {
                if left[i] < right[j] {
                    table[k] = left[i];
                    k += 1;
                    i += 1;
                } else {
                    table[k] = right[j];
                    k += 1;
                    j += 1;
                }
            }
            if i < left.len() {
                table[k..].copy_from_slice(&left[i..]);
            }
            if j < right.len() {
                table[k..].copy_from_slice(&right[j..]);
            }

            return table;
        } else {
            return arg.clone();
        }
    }

    let mut table = arg.clone();

    if table.len() <= 1 {
        return table;
    }

    let mut left = vec![];
    let mut right = vec![];

    for i in 0..table.len() {
        if i < table.len() / 2 {
            left.push(table[i]);
        } else {
            right.push(table[i]);
        }
    }

    left = merge_sort(&left);
    right = merge_sort(&right);

    table = merge(&table, &left, &right);

    return table;
}

fn quick_sort(arg: &Vec<i64>) -> Vec<i64> {
    fn partition(start: usize, end: usize, arg: &Vec<i64>) -> usize {
        let mut table = arg.clone();
        let pivot = table[end];
        let mut index = start;
        for i in start..end {
            if table[i] < pivot {
                table.swap(i, index);
                index += 1;
            }
        }
        table.swap(index, end);
        return index;
    }
    fn q_sort(start: usize, end: usize, arg: &Vec<i64>) -> Vec<i64> {
        let table = arg.clone();
        if start < end {
            let p = partition(start, end, &table);
            q_sort(start, p - 1, &table);
            q_sort(p + 1, end, &table);
        }
        return table;
    }
    return q_sort(1, arg.len() - 1, &arg);
}

fn heap_sort(arg: &Vec<i64>) -> Vec<i64> {
    let mut table = arg.clone();
    return table;
}
