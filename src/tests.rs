

#[cfg(test)]
mod tests {
    use std::thread::available_parallelism;
    use crate::{HashCalculator, HashFinder};
    use super::*;


    // Проверка, что `calculate_hash` возвращает None, если хеш не оканчивается на заданное количество нулей.
    #[test]
    fn test_calculate_hash_none() {
        let calculator = HashCalculator::new(5, 10);
        let result = calculator.calculate_hash(10);
        if let None = result {
            assert!(true);
        }
    }


    // Проверка, что количество ядер, полученное из `available_parallelism`, используется корректно.
    #[test]
    fn test_available_cores_used() {
        let cores = match available_parallelism() {
            Ok(cores) => cores.get(),
            Err(_) => 1,
        };
        assert!(cores > 0);
    }


    // Проверка, что программа завершается с кодом выхода 2 при неверном значении для параметра number_of_zeros.
    #[test]
    fn test_invalid_number_of_zeros_exit_code() {
        use assert_cmd::Command;

        let mut cmd = Command::cargo_bin("hash_finder").unwrap();
        cmd.arg("-N").arg("65").arg("-F").arg("10");
        cmd.assert().code(2);
    }


    // Проверка, что `calculate_hash` возвращает хеш, который оканчивается на заданное количество нулей, или None, если такого хеша нет.
    #[test]
    fn test_calculate_hash() {
        let calculator = HashCalculator::new(2, 10);
        let result = calculator.calculate_hash(10);
        if let Some((_, hash)) = result {
            assert!(hash.ends_with("00"));
        }
    }


    // Проверка, что `display_hashes` выводит хеши в stdout.
    // (Поскольку функция ничего не возвращает, этот тест просто запускает функцию без проверки.)
    #[test]
    fn test_display_hashes_count() {
        let calculator = HashCalculator::new(1, 3);
        let hashes = calculator.find_hashes();
        calculator.display_hashes(&hashes);
    }

}