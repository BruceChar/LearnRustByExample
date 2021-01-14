#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("Bruce"),
            age: 18
        }
    }
}

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        let v: Vec<_> = s.split(",").collect();
        if v.len() < 2 || s.len() == 0 {
            return Person::default();
        }

        match v[1].parse::<u8>() {
            Ok(age) => {
                if v[0].len() > 0 {
                    Person {
                        name: v[0].to_string(),
                        age
                    }
                } else {
                    Person::default()
                }
            }
            Err(e) => Person::default()
        }
    }
}

fn main() {
    let str = "hes,";
    let p: Person = str.into();
    let p1 = Person::from("wrtf,1");
    println!("{:?} {:?}", p, p1);
}