use std::collections::HashMap;
use regex::Regex; // Required for move parsing
use super::board::Board;
use crate::types::board::internalstate::GameState;


// Struct to hold the parsed PGN game data.
#[derive(Debug)]
pub struct PgnGame {
    pub tags: HashMap<String, String>,
    pub movetext: String,
    pub parsed_moves: Vec<String>, // New field for parsed moves
    pub result: GameState,
}

impl PgnGame {
    /// Creates a new PgnGame instance.
    pub fn new() -> Self {
        PgnGame {
            tags: HashMap::new(),
            movetext: String::new(),
            parsed_moves: Vec::new(),
            result: GameState::InProgress,
        }
    }

    /// Parses a PGN string and returns a PgnGame struct.
    pub fn parse(pgn_string: &str) -> Result<PgnGame, String> {
        let mut game = PgnGame::new();
        let mut lines = pgn_string.lines().peekable();
        let mut raw_result_str: Option<String> = None; // To store the raw result string for move parsing

        // Parse tags
        while let Some(line) = lines.next() {
            // Check if the line is a tag pair
            if line.starts_with('[') && line.ends_with(']') {
                // Remove brackets and split by the first space to separate tag name and value
                let content = &line[1..line.len() - 1];
                let parts: Vec<&str> = content.splitn(2, ' ').collect();

                if parts.len() == 2 {
                    let tag_name = parts[0].to_string();
                    // Remove quotes from the tag value
                    let tag_value = parts[1].trim_matches('"').to_string();
                    if tag_name == "Result" {
                        raw_result_str = Some(tag_value.clone());
                    }
                    game.tags.insert(tag_name, tag_value);
                } else {
                    return Err(format!("Malformed tag pair: {}", line));
                }
            } else if !line.trim().is_empty() {
                // If it's not a tag line and not empty, it's the start of movetext
                let mut full_movetext_lines = String::new();
                full_movetext_lines.push_str(line.trim());
                full_movetext_lines.push('\n');

                while let Some(movetext_line) = lines.next() {
                    full_movetext_lines.push_str(movetext_line.trim());
                    full_movetext_lines.push('\n');
                }

                game.movetext = full_movetext_lines.trim().to_string();
                break; // Tags are done, move to movetext parsing
            }
        }

        // Extract result from tags if available
        if let Some(result_str) = game.tags.get("Result") {
            game.result = result_str.as_str().into();
            raw_result_str = Some(result_str.clone());
        } else {
            // Attempt to extract result from the end of movetext if not in tags
            if let Some(last_word) = game.movetext.split_whitespace().last() {
                game.result = last_word.into();
                raw_result_str = Some(last_word.to_string());
            }
        }

        // Parse moves from movetext
        if let Some(result_str) = raw_result_str {
            game.parsed_moves = PgnGame::parse_movetext_into_moves(&game.movetext, &result_str);
        } else {
            // If no result found in tags or movetext, try to parse moves assuming no result string to filter
            game.parsed_moves = PgnGame::parse_movetext_into_moves(&game.movetext, "");
        }


        Ok(game)
    }

    /// Helper function to parse movetext into individual moves.
    fn parse_movetext_into_moves(movetext: &str, game_result_str: &str) -> Vec<String> {
        let mut moves = Vec::new();

        // Regex to remove comments {}
        let re_comments = Regex::new(r"\{[^}]*\}").expect("Invalid regex for comments");
        let cleaned_movetext = re_comments.replace_all(movetext, "").to_string();

        // Regex to remove variations () - basic, might not handle nested variations perfectly
        let re_variations = Regex::new(r"\([^)]*\)").expect("Invalid regex for variations");
        let cleaned_movetext = re_variations.replace_all(&cleaned_movetext, "").to_string();

        let parts: Vec<&str> = cleaned_movetext.split_whitespace().collect();

        for part in parts {
            // Skip move numbers (e.g., "1.", "2.", "1...")
            // A move number is typically one or more digits followed by a dot.
            if part.ends_with('.') && part.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                continue;
            }
            // Also skip '...' which sometimes appears for black's move number if it's explicitly shown.
            if part == "..." {
                continue;
            }

            // Skip the final game result if it's still present in the movetext
            if part == game_result_str {
                continue;
            }

            // Otherwise, it's considered a move (or a NAG, or other PGN annotation, which we keep for now)
            moves.push(part.to_string());
        }
        moves
    }
}

impl Board{
    pub fn from_pgn(pgn_game: PgnGame) -> Result<Self, String> {
        let game = pgn_game;
        let mut board = Board::start();
        for move_str in game.parsed_moves {
            let _mv_str = move_str.clone().replace("+", "").replace("#", "");
            let moves = board.generate_all_moves();
            let algebraic = moves.generate_algebraic_notation(&board);
            if algebraic.contains(&_mv_str) {
                let index = algebraic.iter().position(|s| s == &_mv_str).unwrap();
                let mv = moves.moves[index];
                board.apply_move(&mv);
            }else{
                println!("{}", move_str);
                println!("{:?}", algebraic);
                panic!()
            }
        }
        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_game() {
        let pgn_example = r#"[Event "Casual Game"]
[Site "Landshut, Bavaria, Germany"]
[Date "2025.06.28"]
[Round "?"]
[White "PlayerA"]
[Black "PlayerB"]
[Result "1-0"]

1. e4 e5 2. Nf3 Nc6 3. Bc4 Bc5 4. b4 Bb6 5. a4 a6 6. Nc3 Nf6 7. Nd5 Nxd5 8. exd5 Na5 9. Bd3 O-O 10. O-O d6 11. b5 axb5 12. axb5 c6 13. dxc6 bxc6 14. bxc6 Nxc6 15. Rxa8 Bc7 16. c3 d5 17. Ba3 e4 18. Bxf8 Kxf8 19. Bb5 Qd6 20. Rxc8+ Ke7 21. Bxc6 exf3 22. Qe1+ Kf6 23. g3 g5 24. Qe8 Kf5 25. Qxf7+ Ke4 26. Re1+ Kd3 27. Re3+ Kc2 28. Qf5+ Kd1 29. Ba4+ Kxd2 30. Rd3+ Ke1 31. Re3+ Kd2 32. Rxf3 d4 33. cxd4 Bb6 34. Qxg5 1-0"#;

        let game = PgnGame::parse(pgn_example).unwrap();

        assert_eq!(game.tags["Event"], "Casual Game");
        assert_eq!(game.tags["Site"], "Landshut, Bavaria, Germany");
        assert_eq!(game.tags["White"], "PlayerA");
        assert_eq!(game.tags["Black"], "PlayerB");
        assert_eq!(game.result, GameState::WhiteWin);

        let expected_movetext_start = "1. e4 e5 2. Nf3 Nc6 3. Bc4 Bc5 4. b4 Bb6 5. a4 a6 6. Nc3 Nf6 7. Nd5 Nxd5 8. exd5 Na5 9. Bd3 O-O 10. O-O d6 11. b5 axb5 12. axb5 c6 13. dxc6 bxc6 14. bxc6 Nxc6 15. Rxa8 Bc7 16. c3 d5 17. Ba3 e4 18. Bxf8 Kxf8 19. Bb5 Qd6 20. Rxc8+ Ke7 21. Bxc6 exf3 22. Qe1+ Kf6 23. g3 g5 24. Qe8 Kf5 25. Qxf7+ Ke4 26. Re1+ Kd3 27. Re3+ Kc2 28. Qf5+ Kd1 29. Ba4+ Kxd2 30. Rd3+ Ke1 31. Re3+ Kd2 32. Rxf3 d4 33. cxd4 Bb6 34. Qxg5 1-0";
        assert_eq!(game.movetext, expected_movetext_start);

        let expected_moves = vec![
            "e4", "e5", "Nf3", "Nc6", "Bc4", "Bc5", "b4", "Bb6", "a4", "a6",
            "Nc3", "Nf6", "Nd5", "Nxd5", "exd5", "Na5", "Bd3", "O-O", "O-O", "d6",
            "b5", "axb5", "axb5", "c6", "dxc6", "bxc6", "bxc6", "Nxc6", "Rxa8", "Bc7",
            "c3", "d5", "Ba3", "e4", "Bxf8", "Kxf8", "Bb5", "Qd6", "Rxc8+", "Ke7",
            "Bxc6", "exf3", "Qe1+", "Kf6", "g3", "g5", "Qe8", "Kf5", "Qxf7+", "Ke4",
            "Re1+", "Kd3", "Re3+", "Kc2", "Qf5+", "Kd1", "Ba4+", "Kxd2", "Rd3+", "Ke1",
            "Re3+", "Kd2", "Rxf3", "d4", "cxd4", "Bb6", "Qxg5"
        ];
        assert_eq!(game.parsed_moves, expected_moves);
        let board = Board::from_pgn(game).unwrap();
    }

    #[test]
    fn test_parse_draw_with_comments_and_variations() {
        let pgn_draw_example = r#"[Event "Friendly Match"]
[Site "Online"]
[Date "2025.06.28"]
[Round "1"]
[White "Alice"]
[Black "Bob"]
[Result "1/2-1/2"]

1. d4 {White's opening move} Nf6 (1... c5 {Sicilian Defense} 2. e4) 2. c4 e6 3. Nf3 d5 4. Nc3 Be7 5. Bg5 O-O 6. e3 Nbd7 7. Bd3 dxc4 8. Bxc4 c5 9. O-O a6 1/2-1/2"#;

        let game = PgnGame::parse(pgn_draw_example).unwrap();

        assert_eq!(game.tags["Event"], "Friendly Match");
        assert_eq!(game.tags["White"], "Alice");
        assert_eq!(game.tags["Black"], "Bob");
        assert_eq!(game.result, GameState::Draw);

        let expected_moves = vec![
            "d4", "Nf6", "c4", "e6", "Nf3", "d5", "Nc3", "Be7", "Bg5", "O-O",
            "e3", "Nbd7", "Bd3", "dxc4", "Bxc4", "c5", "O-O", "a6"
        ];
        assert_eq!(game.parsed_moves, expected_moves);
    }

    #[test]
    fn test_parse_no_result_tag() {
        let pgn_no_result_tag_example = r#"[Event "No Result Tag"]
[Site "Test"]
[Date "2025.01.01"]
[White "PlayerX"]
[Black "PlayerY"]

1. e4 e5 2. Nf3 Nc6 3. Bb5 1-0"#;

        let game = PgnGame::parse(pgn_no_result_tag_example).unwrap();

        assert_eq!(game.tags["Event"], "No Result Tag");
        assert_eq!(game.tags["White"], "PlayerX");
        assert_eq!(game.tags["Black"], "PlayerY");
        assert_eq!(game.result, GameState::WhiteWin);

        let expected_moves = vec![
            "e4", "e5", "Nf3", "Nc6", "Bb5"
        ];
        assert_eq!(game.parsed_moves, expected_moves);
    }

    #[test]
    fn test_empty_pgn() {
        let pgn_empty = "";
        let game = PgnGame::parse(pgn_empty).unwrap();
        assert!(game.tags.is_empty());
        assert!(game.movetext.is_empty());
        assert!(game.parsed_moves.is_empty());
        assert_eq!(game.result, GameState::InProgress);
    }

    #[test]
    fn test_only_tags() {
        let pgn_only_tags = r#"[Event "Only Tags"]
[White "Test"]
[Result "*"]"#;
        let game = PgnGame::parse(pgn_only_tags).unwrap();
        assert_eq!(game.tags["Event"], "Only Tags");
        assert_eq!(game.result, GameState::InProgress);
        assert!(game.movetext.is_empty());
        assert!(game.parsed_moves.is_empty());
    }
}
