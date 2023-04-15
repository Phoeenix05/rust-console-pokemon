use inquire::Select;

fn main() {
    // let name = Text::new("What is yout name?").prompt();

    // match name {
    //     Ok(name) => println!("Hello, {}", name),
    //     Err(_) => println!("An error happened when asking for your name, try again later."),
    // }
    
    let options = vec!["Pikachu", "Charmander", "Bulbasaur", "Squirtle"];
    
    let answer = Select::new("Select a pokemon", options).prompt();
    
    match answer {
        Ok(name) => println!("{}", name),
        Err(_) => println!("An error happened when asking you to select a pokemon"),
    }
}
