# Design Patterns Pokedex (Rust)

*This is a work-in-progress, many many design changes still to be worked on as well as more patterns to be included.*

Classic software design patterns case study, modeled through a Pokémon-inspired Pokedex.  
Each module demonstrates how a different pattern can be implemented using idiomatic Rust features such as traits, ownership, and type safety.

---

## Overview

This project explores the following design patterns:

| Pattern      | File                              | Concept Demonstrated |
|---------------|------------------------------------|-----------------------|
| **Factory**   | [`bin/factory.rs`](src/bin/factory.rs) | Centralized object creation logic (PokemonFactory) |
| **Decorator** | [`bin/decorator.rs`](src/bin/decorator.rs) | Adds behavior dynamically to existing objects (AttackAnalyser) |
| **Singleton** | [`bin/singleton.rs`](src/bin/singleton.rs) | Single shared instance across the app (PokeDex) |
| **Strategy**  | [`bin/strategy.rs`](src/bin/strategy.rs) | Dynamic behavior selection (Damage calculation) |

---

## Project Structure

```
pokerust/
├── src/
├── └── bin/
    │   ├── factory.rs      # Factory Pattern example
    │   ├── decorator.rs    # Decorator Pattern example
    │   ├── singleton.rs    # Singleton Pattern example
    │   └── strategy.rs     # Strategy Pattern example
    │   └── lib.rs          # Core module with Pokemon definitions and behavior
└── Cargo.toml
```

---

## Running Examples

Each design pattern can be executed separately:

### Factory
```bash
cargo run --bin factory
```

### Decorator
```bash
cargo run --bin decorator
```

### Singleton
```bash
cargo run --bin singleton
```

### Strategy
```bash
cargo run --bin strategy
```

---

## Core Concepts

- Traits as Interfaces: All Pokémon share common behavior via the `PokemonActions` trait.  
- Ownership and Lifetimes: Each Pokémon carries its own data safely across examples.  
- Generics and Dynamic Dispatch: Patterns like Strategy and Decorator leverage trait objects (`dyn Trait`) for runtime polymorphism.  
- Safe Concurrency: The Singleton uses `OnceLock` for safe, lazily-initialized global state.

---

## Example – Factory Pattern

```rust
let charmander = PokemonFactory::new_pokemon(PokemonType::Fire);
charmander.fight();
```

Output:
```
Pokemon attacked with Blaze!
```

---

## Example – Strategy Pattern

```rust
let fire_strategy = Box::new(FireDamage);
let context = PokemonStrategyContext::new(fire_strategy);
context.process_strategy(squirtle);
```
