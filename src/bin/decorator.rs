use design_patterns_bench::pokemon::{PokemonActions, PokemonType};

struct AttackAnalyser {
    pokemon: Box<dyn PokemonActions>,
}

impl AttackAnalyser {
    fn new(pokemon: Box<dyn PokemonActions>) -> Self {
        AttackAnalyser { pokemon }
    }
}

impl PokemonActions for AttackAnalyser {
    fn fight(&self) {
        println!("{:?} Enemy defeated!", self.pokemon.fight())
    }

    fn run(&self) {
        println!("{:?} Escape was successful!", self.pokemon.run())
    }

    fn heal(&self) {
        println!("{:?} Pokemon is already healthy!", self.pokemon.heal())
    }
}

#[allow(dead_code)]
fn main() {
    #[path = "factory.rs"]
    mod factory;
    use factory::PokemonFactory;

    let charmander = PokemonFactory::new_pokemon(PokemonType::Fire);
    let charmander_with_decorator = AttackAnalyser::new(Box::new(charmander));

    charmander_with_decorator.fight();
    charmander_with_decorator.run();
    charmander_with_decorator.heal();
}
