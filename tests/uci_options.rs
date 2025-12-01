use clockwork::engine::Engine;
use clockwork::api::uci::uci::{parse_uci, UciCommand};
use chess_core::types::color::Color;

#[test]
fn test_uci_set_hash() {
    let mut engine = Engine::new();
    
    // Default is 64MB
    assert_eq!(engine.config.tt_size_mb, 64);

    let cmd_str = "setoption name Hash value 128";
    let cmd = parse_uci(cmd_str, Color::White).expect("Failed to parse setoption");
    
    if let UciCommand::SetOption { name, value } = &cmd {
        assert_eq!(name, "Hash");
        assert_eq!(value.as_deref(), Some("128"));
    } else {
        panic!("Parsed command is not SetOption");
    }

    engine.execute(cmd);

    assert_eq!(engine.config.tt_size_mb, 128);
    
    // Check if TT was actually resized (table length should be > 0)
    let tt = engine.cache.lock().unwrap();
    assert!(tt.table.len() > 0);
}

#[test]
fn test_uci_set_threads() {
    let mut engine = Engine::new();
    
    // Default is 1
    assert_eq!(engine.config.threads, 1);

    let cmd_str = "setoption name Threads value 4";
    let cmd = parse_uci(cmd_str, Color::White).expect("Failed to parse setoption");

    if let UciCommand::SetOption { name, value } = &cmd {
        assert_eq!(name, "Threads");
        assert_eq!(value.as_deref(), Some("4"));
    } else {
        panic!("Parsed command is not SetOption");
    }

    engine.execute(cmd);

    assert_eq!(engine.config.threads, 4);
}
