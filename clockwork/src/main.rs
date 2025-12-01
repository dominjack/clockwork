use clockwork::{
    api::uci::uci::{UciCommand, parse_uci},
    engine::Engine,
};

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
