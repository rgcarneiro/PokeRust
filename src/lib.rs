pub mod pokemon {
    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub enum PokemonType {
        Water,
        Fire,
        Grass,
    }

    #[derive(Clone, Debug)]
    pub enum PokemonGender {
        Male,
        Female,
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub enum Category {
        Lizard,
        Seed,
        TinyTurtle,
    }

    pub trait PokemonActions {
        fn fight(&self);
        fn heal(&self);
        fn run(&self);
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub struct Pokemon<'a> {
        pub name: String,
        pub element: PokemonType,
        category: Category,
        height: f32,
        weight: f32,
        gender: PokemonGender,
        abilities: Vec<&'a str>,
    }

    impl<'a> Pokemon<'a> {
        pub fn new(
            name: String,
            element: PokemonType,
            category: Category,
            height: f32,
            weight: f32,
            gender: PokemonGender,
            abilities: Vec<&'a str>,
        ) -> Self {
            Self {
                name,
                element,
                category,
                height,
                weight,
                gender,
                abilities,
            }
        }
    }

    impl<'a> PokemonActions for Pokemon<'a> {
        fn fight(&self) {
            let ability = self.abilities.clone();
            println!("Pokemon attacked with {}!", ability[0])
        }

        fn heal(&self) {
            println!("Pokemon, heal!")
        }

        fn run(&self) {
            println!("Pokemon, run!")
        }
    }
}
