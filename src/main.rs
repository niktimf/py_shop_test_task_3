

/// Задание 3. Разработать консольное приложение для подбора хеша
/// Задача
/// Требуется разработать консольное приложение, которое будет перебирать целые числа начиная с 1,
/// для каждого из чисел рассчитывать хеш sha256, и выводить в консоль хеш и исходное число,
/// если дайджест хеша (символьное представление хеша) оканчивается N-символами нуля.
/// N задается пользователем при запуске приложения. Параметр F определяет сколько значений хеша следует найти команде.
///
/// Замечания к реализации
/// Требуется разработать консольное приложение Rust.
/// Код должен сопровождаться Cargo.toml, чтобы приложение было легко собрать и запустить для проверки.
/// Архитектура приложения должна быть ориентирована на максимальную утилизацию вычислительных мощностей (concurrency, parallelism).
/// При оценке будут учитываться архитектура, декомпозиция, документация, тесты.



mod tests;

use sha2::{Sha256, Digest};
use rayon::prelude::*;

use std::{thread};
use std::thread::available_parallelism;

use clap::{Arg, Command, value_parser};


/// Структура, представляющая калькулятор хэшей.
///
/// Он использует SHA256 для генерации хэшей и ищет хэши,
/// которые заканчиваются на определенное количество нулей.
pub struct HashCalculator {
    number_of_zeros: usize,
    count_of_hashes: usize,
}

impl HashCalculator {
    /// Создает новый экземпляр калькулятора хэшей.
    ///
    /// # Аргументы
    ///
    /// * `number_of_zeros` - Количество нулей, на которое должен заканчиваться хэш.
    /// * `count_of_hashes` - Количество хешей, которые нужно найти.
    pub fn new(number_of_zeros: usize, count_of_hashes: usize) -> Self {
        HashCalculator { number_of_zeros, count_of_hashes }
    }

    /// Рассчитывает хэш для заданного числа.
    ///
    /// Если хэш заканчивается на нужное количество нулей, возвращает `Some`, иначе `None`.
    pub fn calculate_hash(&self, num: u64) -> Option<(u64, String)> {
        let mut hasher = Sha256::new();
        hasher.update(num.to_string());
        let result = hasher.finalize();
        let hex = format!("{:x}", result);
        if hex.ends_with(&"0".repeat(self.number_of_zeros)) {
            Some((num, hex))
        } else {
            None
        }
    }
}

/// Трейт для поиска и отображения хешей.
trait HashFinder {
    /// Ищет хэши, которые соответствуют условиям.
    fn find_hashes(&self) -> Vec<(u64, String)>;
    /// Отображает найденные хэши.
    fn display_hashes(&self, hashes: &[(u64, String)]);
}

impl HashFinder for HashCalculator {
    // ... [реализация трейта]
    fn find_hashes(&self) -> Vec<(u64, String)> {
        let threads = match available_parallelism() {
            Ok(cores) => cores.get(),
            Err(e) => {
                println!("Не удалось получить количество ядер ЦП: {}", e);
                std::process::exit(1);
            }
        };

        let count_of_hashes = self.count_of_hashes;
        let number_of_zeros = self.number_of_zeros;


        let handles: Vec<_> = (0..threads).map(|i| {
            thread::spawn(move || {
                (i as u64..)
                    .step_by(threads)
                    .filter_map(|num| HashCalculator::new(number_of_zeros, count_of_hashes).calculate_hash(num))
                    .take(count_of_hashes)
                    .collect::<Vec<_>>()
            })
        }).collect();

        let mut all_hashes: Vec<_> = handles.into_par_iter()
            .filter_map(|handle| handle.join().ok())
            .flatten()
            .collect();


        all_hashes.sort_by_key(|k| k.0);
        all_hashes
    }

    fn display_hashes(&self, hashes: &[(u64, String)]) {
        hashes
            .par_iter()
            .take(self.count_of_hashes)
            .for_each(|(num, hash)| {
                println!("{}, \"{}\"", num, hash);
            });
    }
}

/// Главная точка входа в программу.
///
/// Эта функция считывает аргументы командной строки, чтобы определить количество нулей,
/// на которое должен заканчиваться хэш, и количество хешей, которые нужно найти.
/// Затем она инициализирует `HashCalculator`, находит и отображает соответствующие хэши.
fn main() {

    let matches = Command::new("Hash Finder")
        //.author("niktimf@gmail.com")
        .about("Находит хэши, которые заканчиваются на определенное количество нулей и количество этих хешей")
        .arg(
            Arg::new("number_of_zeros")
                .short('N')
                .long("number-of-zeros")
                .value_parser(value_parser!(u64).range(1..=64))
                .value_name("NUMBER")
                .help("Количество нулей в конце хеша")
                .required(true),
        )
        .arg(
            Arg::new("count_of_hashes")
                .short('F')
                .long("count-of-hashes")
                .value_parser(value_parser!(u64).range(1..))
                .value_name("NUMBER")
                .help("Количество хешей, которые нужно найти")
                .required(true),
        )
        .try_get_matches()
        .unwrap_or_else(|e| e.exit());


    let number_of_zeros = if let Some(value) = matches.get_one::<u64>("number_of_zeros") {
        *value
    } else {
        println!("Неккоректное значение для N");
        std::process::exit(2);
    };

    let count_of_hashes = if let Some(value) = matches.get_one::<u64>("count_of_hashes") {
        *value
    } else {
        println!("Неккоректное значение для F");
        std::process::exit(3);
    };


    let calculator = HashCalculator::new(number_of_zeros as usize, count_of_hashes as usize);
    let hashes = calculator.find_hashes();
    calculator.display_hashes(&hashes);
}


