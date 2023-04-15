use inquire::Select;

#[derive(Debug, Clone)]
enum PokemonType {
    Grass,
    Fire,
    Water,
    Electric,
}

struct Pokemon {
    name: String,
    pokemon_type: PokemonType,
}

macro_rules! pokemon {
    ($name:expr, $pokemon_type:expr) => {
        Pokemon {
            name: $name,
            pokemon_type: $pokemon_type,
        }
    };
}

// const POKEMON: Vec<Pokemon> = vec![
//     pokemon!("Pikachu", PokemonType::Electric)
// ];

fn main() {
    let pokemon: Vec<Pokemon> = vec![
        pokemon!("Pikachu".to_string(), PokemonType::Electric),
        pokemon!("Charmander".to_string(), PokemonType::Fire),
        pokemon!("Bulbasaur".to_string(), PokemonType::Grass),
        pokemon!("Squirtle".to_string(), PokemonType::Water),
    ];

    let options = pokemon
        .iter()
        .map(|pokemon| format!("{} ({:?})", pokemon.name.clone(), pokemon.pokemon_type.clone()))
        .collect::<Vec<String>>();

    let answer = Select::new("Select a pokemon", options).prompt();
    // _ = Confirm::new("message").prompt();

    match answer {
        Ok(name) => println!("{}", name),
        Err(_) => println!("An error happened when asking you to select a pokemon"),
    }
}
