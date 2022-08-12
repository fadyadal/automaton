use ::worlds::Worlds;
use clap::Parser;
use spaces::Spaces;

#[cfg(feature = "sdl2")]
pub mod gui;
#[cfg(feature = "cursive")]
pub mod terminal;
#[cfg(feature = "wasm")]
pub mod web;

mod worlds;

/// Simulate basic cellular automaton-based worlds
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Width of the canvas to draw in
    #[clap(short = 'W', long, value_parser, default_value_t = 1200)]
    width: usize,

    /// Height of the canvas to draw in
    #[clap(short = 'H', long, value_parser, default_value_t = 700)]
    height: usize,

    /// The size of each cell represented in the canvas
    #[clap(short, long, value_parser, default_value_t = 20)]
    cell_size: usize,

    /// Time to wait between each tick
    #[clap(short, long, value_parser, default_value_t = 100)]
    update_millis: usize,

    /// The type of [Worlds] to simulate
    #[clap(short, long)]
    world: Worlds,

    /// the type of [Space] to use as canvas
    #[clap(short, long)]
    frontend: Spaces,
}

fn main() -> Result<(), String> {
    use cellular_automaton::common::Dimensions;

    let args = Args::parse();

    match args.frontend {
        #[cfg(feature = "cursive")]
        Spaces::Cursive => terminal::run(
            args.world,
            Dimensions(args.width, args.height),
            args.update_millis,
        ),
        #[cfg(feature = "sdl2")]
        Spaces::Sdl2 => gui::run(
            args.world,
            Dimensions(args.width, args.height),
            args.cell_size,
            args.update_millis,
        ),
    }
}
