use design_patterns_bench::pokemon::{Pokemon, PokemonType};
use std::sync::OnceLock;

#[derive(Clone, Debug)]
struct PokeDex<'a> {
    data: OnceLock<Pokemon<'a>>,
    next: OnceLock<Box<PokeDex<'a>>>,
}

impl<'a> PokeDex<'a> {
    /// Never instantiate a PokeDex with PokeDex::new(),
    /// use PokeDex::instance() instead.
    unsafe fn new() -> Self {
        Self {
            data: OnceLock::new(),
            next: OnceLock::new(),
        }
    }

    fn instance() -> &'static PokeDex<'a> {
        static INSTANCE: OnceLock<PokeDex> = OnceLock::new();
        unsafe { INSTANCE.get_or_init(|| PokeDex::new()) }
    }

    fn push(&self, pokemon: Pokemon<'a>) {
        if let Err(pokemon) = self.data.set(pokemon) {
            unsafe {
                let next = self.next.get_or_init(|| Box::new(PokeDex::new()));
                next.push(pokemon);
            }
        }
    }

    fn get_pokemons(&self) -> Vec<Pokemon<'a>> {
        let mut pokemons = Vec::new();
        let mut current = Some(self);

        while let Some(pokedex) = current {
            if let Some(pokemon) = pokedex.data.get() {
                pokemons.push(pokemon.clone());
            }
            current = pokedex.next.get().map(|next| next.as_ref());
        }
        pokemons
    }
}

#[allow(dead_code)]
fn main() {
    #[path = "factory.rs"]
    mod factory;
    use factory::PokemonFactory;

    {
        let charmander = PokemonFactory::new_pokemon(PokemonType::Fire);
        let pokedex = PokeDex::instance();
        pokedex.push(charmander.clone());

        let pokemons_in_pokedex = pokedex.get_pokemons();
        println!("{:?}", pokemons_in_pokedex);
    }
    {
        let squirtle = PokemonFactory::new_pokemon(PokemonType::Water);
        let pokedex_2 = PokeDex::instance();
        pokedex_2.push(squirtle.clone());

        let pokemons_in_pokedex_2 = pokedex_2.get_pokemons();
        println!("{:?}", pokemons_in_pokedex_2);
    }

    // Example error of instatiation without unsafe
    // Code will panic
    //let pokedex_3 = PokeDex::new();
    //let pokemons_in_pokedex_3 = pokedex_3.get_pokemons();
    //println!("{pokemons_in_pokedex_3:?}");
}
