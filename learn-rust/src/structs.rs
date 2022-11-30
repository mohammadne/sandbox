// traditional struct
struct Color1 {
    red: u8,
    green: u8,
    blue: u8,
}

// tuple struct
struct Color2(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut color1 = Color1{
        red: 255,
        green: 0,
        blue: 0,
    };
    color1.blue = 100;
    println!("color: {}, {}, {}", color1.red, color1.green, color1.blue);

    let mut color2 = Color2(255, 0, 0);
    println!("color: {}, {}, {}", color2.0, color2.1, color2.2);

    let mut person = Person::new("mohammad", "nasr");
    println!("person: {} {}", person.first_name, person.last_name);
    person.set_last_name("esfahani");
    println!("person full_name: {}", person.full_name());
    println!("person tuple: {:?}", person.to_tuple());
}