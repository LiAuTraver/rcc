use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Display, EnumString, PartialEq, Eq)]
pub enum Operator {
  // one-character operators
  #[strum(serialize = "+")]
  Plus,
  #[strum(serialize = "-")]
  Minus,
  #[strum(serialize = "*")]
  Star,
  #[strum(serialize = "/")]
  Slash,
  #[strum(serialize = "%")]
  Percent,
  #[strum(serialize = ",")]
  Comma,
  #[strum(serialize = ";")]
  Semicolon,
  #[strum(serialize = "(")]
  LeftParen,
  #[strum(serialize = ")")]
  RightParen,
  #[strum(serialize = "{")]
  LeftBrace,
  #[strum(serialize = "}")]
  RightBrace,
  #[strum(serialize = "[")]
  LeftBracket,
  #[strum(serialize = "]")]
  RightBracket,
  #[strum(serialize = "=")]
  Assign,
  #[strum(serialize = "!")]
  Not,
  #[strum(serialize = "<")]
  Less,
  #[strum(serialize = ">")]
  Greater,
  #[strum(serialize = "&")]
  Ampersand,
  #[strum(serialize = "|")]
  Pipe,
  #[strum(serialize = "^")]
  Caret,
  #[strum(serialize = "~")]
  Tilde,
  #[strum(serialize = ".")]
  Dot,
  #[strum(serialize = "?")]
  Question,
  #[strum(serialize = ":")]
  Colon,
  // multi-character operators
  #[strum(serialize = "++")]
  PlusPlus,
  #[strum(serialize = "--")]
  MinusMinus,
  #[strum(serialize = "+=")]
  PlusAssign,
  #[strum(serialize = "-=")]
  MinusAssign,
  #[strum(serialize = "*=")]
  StarAssign,
  #[strum(serialize = "/=")]
  SlashAssign,
  #[strum(serialize = "%=")]
  PercentAssign,
  #[strum(serialize = "==")]
  EqualEqual,
  #[strum(serialize = "!=")]
  NotEqual,
  #[strum(serialize = "<=")]
  LessEqual,
  #[strum(serialize = ">=")]
  GreaterEqual,
  #[strum(serialize = "&&")]
  And,
  #[strum(serialize = "||")]
  Or,
  #[strum(serialize = "<<")]
  LeftShift,
  #[strum(serialize = ">>")]
  RightShift,
  #[strum(serialize = "&=")]
  AmpersandAssign,
  #[strum(serialize = "|=")]
  PipeAssign,
  #[strum(serialize = "^=")]
  CaretAssign,
  #[strum(serialize = "<<=")]
  LeftShiftAssign,
  #[strum(serialize = ">>=")]
  RightShiftAssign,
  #[strum(serialize = "->")]
  Arrow,

  // preprocessor
  #[strum(serialize = "#")]
  Hash,
  #[strum(serialize = "##")]
  HashHash,

  #[strum(serialize = "\033")] // just a garbage value
  EOF,
}

impl Operator {
  pub fn unary(&self) -> bool {
    matches!(
      self,
      Operator::Plus
        | Operator::Minus
        | Operator::Star
        | Operator::Not
        | Operator::Tilde
        | Operator::Ampersand
        | Operator::PlusPlus
        | Operator::MinusMinus
    )
  }
  pub fn binary(&self) -> bool {
    matches!(
      self,
      Operator::Plus
        | Operator::Minus
        | Operator::Star
        | Operator::Slash
        | Operator::Percent
        | Operator::EqualEqual
        | Operator::NotEqual
        | Operator::Less
        | Operator::LessEqual
        | Operator::Greater
        | Operator::GreaterEqual
        | Operator::And
        | Operator::Or
        | Operator::LeftShift
        | Operator::RightShift
        | Operator::Ampersand
        | Operator::Pipe
        | Operator::Caret
    )
  }
  // left-.
  pub fn precedence(&self) -> u8 {
    match self {
      Operator::Star => 0x80,
      Operator::Slash => 0x80,
      Operator::Percent => 0x80,
      Operator::Plus => 0x40,
      Operator::Minus => 0x40,
      Operator::LeftShift => 0x20,
      Operator::RightShift => 0x20,
      Operator::Less => 0x10,
      Operator::LessEqual => 0x10,
      Operator::Greater => 0x10,
      Operator::GreaterEqual => 0x10,
      Operator::EqualEqual => 0x08,
      Operator::NotEqual => 0x08,
      Operator::Ampersand => 0x08,
      Operator::Caret => 0x04,
      Operator::Pipe => 0x02,
      _ => panic!("not a binary op or it is a rel op"),
    }
  }
}
