use design_patterns_bench::pokemon::{Category, Pokemon, PokemonGender, PokemonType};

pub struct PokemonFactory;

impl<'a> PokemonFactory {
    pub fn new_pokemon(pokemon_type: PokemonType) -> Pokemon<'a> {
        match pokemon_type {
            PokemonType::Fire => Pokemon::new(
                String::from("Charmander"),
                PokemonType::Fire,
                Category::Lizard,
                2.00,
                18.7,
                PokemonGender::Male,
                vec!["Blaze"],
            ),
            PokemonType::Water => Pokemon::new(
                String::from("Squirtle"),
                PokemonType::Water,
                Category::TinyTurtle,
                1.80,
                19.8,
                PokemonGender::Male,
                vec!["Torrent"],
            ),

            PokemonType::Grass => Pokemon::new(
                String::from("Bulbasaur"),
                PokemonType::Grass,
                Category::Seed,
                2.04,
                15.2,
                PokemonGender::Female,
                vec!["Overgrow"],
            ),
        }
    }
}

#[allow(dead_code)]
fn main() {
    use design_patterns_bench::pokemon::PokemonActions;

    let charmander = PokemonFactory::new_pokemon(PokemonType::Fire);
    let squirtle = PokemonFactory::new_pokemon(PokemonType::Water);
    let bulbasaur = PokemonFactory::new_pokemon(PokemonType::Grass);

    charmander.fight();
    squirtle.fight();
    bulbasaur.fight();

    charmander.heal();
    charmander.run();
}
