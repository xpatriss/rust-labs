use rand::prelude::*;
use std::fs::File;
use std::io::Write;

fn main() {

    let res: bool = loop {
        let mut buf = String::new();

        println!("Podaj liczbę:");

        // wczytaj liczbę od użytkownika
        std::io::stdin().read_line(&mut buf).expect("Błąd odczytu");

        // przekonwertuj stringa na u64
        let result = buf.trim().parse::<u64>();

        // przypisz wartość do x lub przerwij pętle w przypadku błędu
        let mut x: u64 = match result {
            Ok(n) => n,
            Err(_)=> break true,
        };

        // przerwij pętlę jeśli liczba to 0
        if x == 0 {
            break false;
        }

        // dodaj do x losową liczbę z przedziału [0, 5] i wyświetl nową wartość
        let random = rand::rng().random_range(0..=5);
        x += random;
        println!("Nowa wartość x = {}", x);

        // zapisz do zmiennej tablicę 10-elementową, zawierającą kolejne potęgi x
        let power_tab = power(x);

        // Dla każdej liczby w tablicy sprawdź, czy spełniona jest dla niej hipoteza Collatza
        // i zapisz tę informację do osobnej, 10-elementowej tablicy.
        let collatz_tab : [bool; 10] = collatz(power_tab);

        // Tablicę z poprzedniego punktu zapisz w dowolnym formacie do pliku o nazwie ”xyz.txt”
        let mut file = File::create("xyz.txt").expect("File creation");

        for elem in &collatz_tab {
            let buf  = elem.to_string() + "\n";
            file.write_all(buf.as_bytes()).expect("File write");
        }
    };

    // Jeśli pętla się zakończy, to wartość z niej zwróconą zapisz do zmiennej i wyświetl
    // komunikat, czy zakończyła się z woli użytkownika (false) czy z powodu błędu (true).
    match res {
        true => {
            println!("Zakończono pętle z powodu błedu.");
        },
        false => {
            println!("Zakończono pętle zgodnie z wolą użytkownika.");
        }
    }

    // Dodatkowa funkcjonalność: krotka i zagnieżdżona pętla
    println!("\nGuessing game: podaj liczbę od 0 do 30, jeśli jest w tajemniczej macierzy - wygrywasz!");

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Błąd odczytu");

    let result = buf.trim().parse::<u64>();

    let x: u64 = match result {
        Ok(n) => n,
        Err(_)=> panic!(),
    };
    let (res, i, j) = guess(x);

    match res {
        true => println!("Wygrałeś! Liczba jest na pozycji ({},{})", i, j),
        false => println!("Przegrywasz :(")
    }

}

// funkcja wypełniająca tablicę kolejnymi potęgami x (zaczynając od x^0)
fn power(x: u64) -> [u64; 10] {
    let mut result: [u64; 10] = [0; 10];
    result[0] = 1;
    for i in 1..10 {
        result[i] = x * result[i - 1];
    }
    result
}

fn collatz(tab: [u64; 10]) -> [bool; 10] {
    let mut result = [false; 10];

    for i in 0..10 {
        let mut c0 = tab[i];

        if c0 == 1 {
            result[i] = true;
        }
        else {
            for _ in 0..100 {
                if c0.is_multiple_of(2){
                     c0 /= 2;
                }
                else {
                    c0 = 3 * c0 + 1;
                }
                if c0 == 1 {
                    result[i] = true;
                    break;
                }
            }
        }
    }
    result
}

fn guess(x: u64) -> (bool, usize, usize) {
    let mut matrix = [[0u64; 5]; 5];
    for row in &mut matrix {
        for item in row {
            *item = rand::rng().random_range(0..=30);
        }
    }
    let mut res: (bool, usize, usize) = (false, 0, 0);
    'find: for (row, items) in matrix.iter().enumerate() {
        for (col, item) in items.iter().enumerate() {
            if *item == x {
                res = (true, row, col);
                break 'find;
            }
        }
    }
    println!("Tajemnicza macierz: {:?}", matrix);
    res
}

