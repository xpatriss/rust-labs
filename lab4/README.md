# Lab 4

Trzy niezależne kawałki:

- `divisors` — dzielniki liczby dodatniej w `BTreeSet` z `NonZero<u32>`, z krótkim pomiarem czasu,
- `assert_sorted` — panika, gdy slice `u32` nie jest posortowany rosnąco,
- serwer TCP na `127.0.0.1:7878` — klient przysyła długość ścieżki (4 bajty, big-endian) i samą ścieżkę, serwer odsyła nazwy plików z tego katalogu.

Ćwiczone rzeczy: kolekcje, `NonZero`, obsługa błędów I/O, odczyt i zapis strumienia w kawałkach, testy (także sieciowe, na porcie losowym).

`cargo run` startuje serwer i nie kończy się sam. Testy nie zależą od tego procesu:

```
cargo test
```
