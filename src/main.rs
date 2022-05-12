fn minimum_path(target: usize, shortcuts: &[usize]) -> usize {
    let mut min_path = 0;
    let mut current_node = 0;
    loop {
        if current_node == target {
            return min_path;
        }

        let shortcut = shortcuts.get(current_node).unwrap() - 1;
        if (shortcut <= target) && (shortcut != current_node) {
            current_node = shortcut;
            min_path += 1;
            continue;
        }

        current_node += 1;
        min_path += 1;
    }
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

    println!("{:?}", minimum_paths(n, &shortcuts));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let n = 3;
        let shortcuts = vec![2, 2, 3];

        assert_eq!(vec![0, 1, 2], minimum_paths(n, &shortcuts));
    }

    #[test]
    fn example2() {
        let n = 5;
        let shortcuts = vec![1, 2, 3, 4, 5];

        assert_eq!(vec![0, 1, 2, 3, 4], minimum_paths(n, &shortcuts));
    }

    #[test]
    fn example3() {
        let n = 7;
        let shortcuts = vec![4, 4, 4, 4, 7, 7, 7];

        assert_eq!(vec![0, 1, 2, 1, 2, 3, 3], minimum_paths(n, &shortcuts));
    }
}
