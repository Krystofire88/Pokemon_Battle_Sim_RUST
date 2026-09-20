# Pokemon Battle Simulator

This is a Rust rewrite of my [older C# version](https://github.com/Krystofire88/Pokemon_Battle_Sim).

## Info

Currently, the scope is aimed at the same level as the C# implementation is at, as of September 2026. This means it will have all Pokemon and a limited number of fully implemented moves. It will not have abilities or items. Unlike the C# implementation, I don't plan to make Trainer Battles alongside of 1v1 battles, I will first fully implement 1v1 before touching Trainer battles. Therefore, moves such as U-Turn won't be fully implemented until Trainer battles.

The main idea of this project, besides learning Rust, is a fast, complex and accurate simulation to run battles between AI pokemon or trainers, akin to videos like https://www.youtube.com/watch?v=mM7EhT5UdAE or https://www.youtube.com/watch?v=BukW57lsoIY, the former which inspired the original C# project.

## How To Run

You will need Rust 2024 edition or newer.

Clone the GitHub repo, then using Cargo build and run the program. If you run it as a release build it will not display telemetry data during the battle, such as per round information, it will only print starting and ending states. 

```sh
  git clone https://github.com/Krystofire88/Pokemon_Battle_Sim_RUST
  cd Pokemon_Battle_Sim_RUST
  cargo run -r # Omit release build (-r) if you wish to see in-battle telemetry
```