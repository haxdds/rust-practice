// Structs - used to create custom data types

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct MyColor(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct
    fn new(first: &str, last: &str) -> Person {
        Person{
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

pub fn run(){

    // Tradition Struct
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.green = 169;

    println!("Color: {:?}", (c.red, c.blue, c.green));

    // Tuple Struct
    let mut c2 = MyColor(100, 120, 0);

    println!("MyColor {} {} {}", c.red, c.green, c.blue);

    let mut p = Person::new("John", "Doe");

    println!("Person is {}", p.full_name());

    p.set_last_name("Smith");


    println!("Person changed their name to {}", p.full_name());


    let x = p.to_tuple();
    // p isx moved and deallocated
    
}