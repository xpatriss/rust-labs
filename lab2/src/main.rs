#[derive(Debug, Clone, Default)]
// Copy nie, bo String trzyma dane na stercie, a na stosie ma tylko wskaźnik.
// Copy kopiuje stos bit po bicie, więc skopiowałoby wskaźnik, nie tekst.
struct NumberWithUnit {
    unit: String,
    value: f64,
}

impl NumberWithUnit {
    fn unitless(value: f64) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }

    fn with_unit(value: f64, unit: String)-> Self {
        Self {
            value,
            unit
        }
    }

    fn with_unit_from(other: Self, value: f64)-> Self {
        Self {
            value,
            ..other
        }
    }

    fn add(self, other: Self)-> Self {
        let value : f64 = self.value + other.value;
        let unit : String = self.unit.clone();

       if self.unit != other.unit {
           panic!("Cannot add values if units are not equal")
       }
       else
       {
           Self {
               value,
               unit
           }
       }
    }
    fn mul(self, other: Self)-> Self{
        let value = self.value * other.value;
        let mut unit : String = self.unit.clone();

        if self.unit.is_empty() {
            unit = other.unit.clone();
        }
        else if !other.unit.is_empty() {
            unit = self.unit + "*" + &other.unit;
        }

        Self {
            value,
            unit
        }
    }
    fn div(self, other: Self)-> Self{
        let value = self.value / other.value;
        let mut unit : String = self.unit.clone();

        if self.unit.is_empty() && !other.unit.is_empty() {
            unit = "1/".to_string() + &other.unit;
        }
        else if !other.unit.is_empty() && self.unit != other.unit {
            unit = self.unit + "/" + &other.unit;
        }
        else if other.unit == self.unit {
            unit = "".to_string();
        }

        Self {
            value,
            unit
        }
    }

    fn add_in_place(&mut self, other: &Self) {
        if self.unit != other.unit {
            panic!("Cannot add values if units are not equal")
        }

        self.value += other.value;
    }
    fn mul_in_place(&mut self, other: &Self) {
        self.value *= other.value;

        if self.unit.is_empty() {
            self.unit = other.unit.clone();
        } else if !other.unit.is_empty(){
            self.unit += "*";
            self.unit += other.unit.as_str();
        }
    }
    fn div_in_place(&mut self, other: &Self) {
        self.value /= other.value;

        if self.unit.is_empty() && !other.unit.is_empty() {
            self.unit = "1/".to_string() + &other.unit;
        } else if !other.unit.is_empty() && self.unit != other.unit {
            self.unit += "/";
            self.unit += other.unit.as_str();
        }
        else if other.unit == self.unit {
            self.unit = "".to_string();
        }
    }
}

// struktua krotkowa
struct DoubleString(String, String);

impl DoubleString {
    fn from_strs(str_1: &str, str_2: &str) -> Self {
        Self(
            str_1.to_string(),
            str_2.to_string()
        )
    }

    #[allow(clippy::ptr_arg)]
    fn from_strings(str_1: &String, str_2: &String) -> Self {
        Self(
            str_1.clone(),
            str_2.clone()
        )
    }

    fn show(&self) {
        println!("({}, {})", self.0, self.1);
    }
}


fn main() {
    // stwórz wartości wszystkimi napisanymi funkcjami skojarzonymi, wyświetl je
    let struct1 = NumberWithUnit::unitless(10.0);
    println!("Unitless: {:?}", struct1);
    let struct2 = NumberWithUnit::with_unit(20.0, String::from("hello"));
    println!("With_unit: {:?}", struct2);
    let struct3 = NumberWithUnit::with_unit_from(struct2.clone(), 30.0);
    println!("With_unit_from(struct2) {:?}", struct3);

    // policz prędkość z odległości i czasu.
    let s1 = NumberWithUnit::with_unit(20.0, "m".to_string());
    let s2 = NumberWithUnit::with_unit(10.0, "m".to_string());
    let mut s_c = s1.clone();
    s_c.add_in_place(&s2);
    let s_c_add = s1.clone().add(s2.clone());
    println!("s1: {:?}", s1);
    println!("s2: {:?}", s2);
    println!("s_c: {:?}", s_c);
    println!("s_c_add: {:?}", s_c_add);

    let t = NumberWithUnit::with_unit(10.0, "s".to_string());
    println!("t: {:?}", t);

    let v = s_c.clone().div(t.clone());
    println!("v: {:?}", v);

    s_c.div_in_place(&t);
    println!("v (div_in_place): {:?}", s_c);

    let s_check = v.clone().mul(t.clone());
    println!("s_check (v * t): {:?}", s_check);

    // punkt 10: mul_vals (slice) i mul_vals_vec (Vec) na jednym wektorze
    let vals = vec![s1.clone(), s2.clone(), t.clone()];

    println!("mul_vals 1: {:?}", mul_vals(&vals));
    println!("mul_vals 2: {:?}", mul_vals(&vals));

    println!("mul_vals_vec 1: {:?}", mul_vals_vec(vals.clone()));
    println!("mul_vals_vec 2: {:?}", mul_vals_vec(vals));

    let string: String = "hello".to_string();
    let str_slice: &str = "world";

    let db1 = DoubleString::from_strs(&string, str_slice);
    let str_as_string = str_slice.to_string();
    let db2 = DoubleString::from_strings(&string, &str_as_string);
    // from_strings(&string, str_slice) się nie kompiluje — str_slice to &str, nie &String

    db1.show();
    db2.show();
}

// Napisz funkcję mul_vals, która przyjmuje slice NumberWithUnit i
// zwraca NumberWithUnit z wymnożonymi wszystkimi elementami slice.

fn mul_vals(s: &[NumberWithUnit]) -> NumberWithUnit {
    let mut res: NumberWithUnit = NumberWithUnit::unitless(1.0);
    for elem in s {
        res.mul_in_place(elem);
    }
    res
}

// Napisz funckję mul_vals_vec analogiczną do powyższej, ale przyjmującą Vec (nie &Vec!)
fn mul_vals_vec(v: Vec<NumberWithUnit>) -> NumberWithUnit {
    let mut res: NumberWithUnit = NumberWithUnit::unitless(1.0);
    for elem in v {
        res.mul_in_place(&elem);
    }
    res
}

