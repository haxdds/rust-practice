// conditionals - used to check the condition of something and act on it

pub fn run() {
    let age: u8 = 18;
    let has_id: bool = true;
    let knows_person_of_age = true;

    if age >= 21 && has_id || knows_person_of_age{
        println!("You can drink!");
    } else if age < 21 && has_id {
        println!("Nope no no no!");
    } else {
        println!("lemme see dat id");
    }

    // shorthand if
    let is_of_age = if age >= 21 { "yes" } else { "no" };

    println!("is of age: {}", is_of_age);
}