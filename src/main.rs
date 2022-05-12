fn minimum_path(target: usize, shortcuts: &[usize]) -> usize {
    0
}

fn minimum_paths(n: usize, shortcuts: &[usize]) -> Vec<usize> {
    let mut minimum_paths = Vec::new();
    for i in 0..n {
        let min = minimum_path(i, &shortcuts);
        minimum_paths.push(min);
    }
    minimum_paths
}

fn main() {
    let n = 3;
    let shortcuts = vec![2, 2, 3];

    println!("mike's minimum paths: {:?}", minimum_paths(n, &shortcuts));
}
