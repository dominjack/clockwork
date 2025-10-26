#[derive(Copy, Clone, Eq, PartialEq, Debug, serde::Serialize)]
pub enum Color{
    White,
    Black,
    None
}

impl Color{
    pub const COUNT: usize = 2;


    pub fn to_string(&self) -> char{
        match self{
            Color::White => 'w',
            Color::Black => 'b',
            Color::None => 'n'
        }
    }

    pub fn invert(&self) -> Self{
        match self{
            Color::White => Color::Black,
            Color::Black => Color::White,
            Color::None => Color::None
        }
    }
}

impl TryFrom<&str> for Color{
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error>{
        match value.to_lowercase().as_str(){
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => Err(())
        }
    }
}

impl Default for Color{
    fn default() -> Self{
        Color::None
    }
}


