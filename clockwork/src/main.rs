use clockwork::{
    api::uci::uci::{UciCommand, parse_uci},
    engine::Engine,
};

/// The main entry point for the clockwork chess engine.
///
/// This function initializes the `Engine` and then enters a loop to read and parse UCI commands
/// from standard input. It executes the commands until the `Quit` command is received.
fn main() {
    let mut engine = Engine::new();

    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();

        if let Ok(command) = parse_uci(&buffer, engine.board.state.color) {
            if command == UciCommand::Quit {
                break;
            }

            engine.execute(command);
        }
    }
}
