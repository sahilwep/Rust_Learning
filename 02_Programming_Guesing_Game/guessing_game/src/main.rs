fn main(){
    // Defining a Person struct
    struct Person {
        name: String,
        age: u8,
        height: u8
    }

    // instantiate Person struct : 
    let person1 = Person{
        name: String::from("Prince"),
        age: 23,
        height: 190
    };

    // destructure person struct into name, age, and height variables
    let Person {name, age, height} = person1;

    // accessing the values of person1 struct
    println!("Person name = {}", name);
    println!("Person age = {}", age);
    println!("Person height = {}", height);
}