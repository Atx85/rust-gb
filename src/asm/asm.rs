
pub struct Asm {
  source: String,
}

impl Asm {
  pub fn from_string(&mut self, source: String) -> Self {
    Self {
      source
    }
  }
}
#[derive(PartialEq, Debug, Clone, Copy)]
enum TokenType {
  Unknown,
  Identifier,
  Comma,
  HexLiteral,
  Address,
}

#[derive(PartialEq, Debug)]
struct Token {
  token_type: TokenType,
  start: usize,
  length: usize,
  string: String
}
impl Token {
  pub fn new(token_type: TokenType, start: usize, length: usize, string: String) -> Self {
    Self { token_type, start, length, string}
  }
}
#[derive(PartialEq, Debug)]
enum LexState {
 Idle, InProgress 
}
#[derive(PartialEq, Debug)]
struct Lexer {
  pos: usize,
}
impl Lexer {
  fn new () -> Self {
    Self { pos: 0 }
  }
  fn is_end(self, source: &str) -> bool {
    self.pos >= source.len()
  }
  fn peek(&mut self, source: &str) -> Option<char> {
    source.chars().nth(self.pos)
  }
  fn chop(&mut self, source: &str) -> Option<char> {
    let ch = self.peek(source);    
    match ch {
      Some(c) => self.pos += c.len_utf8(),
      _ => {},
    }
    ch
  }
  fn tokenize(&mut self, src: &str)->Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    self.pos = 0;
    let mut token_start = 0;
    let mut token = String::new();
    let mut state = LexState::Idle;
    let mut token_type: Option<TokenType> = None; 
    while let Some(ch) = self.peek(src) {
      if state == LexState::Idle {
        if ch.is_whitespace() { self.chop(src); continue; }
        if ch == ',' {
          state = LexState::InProgress;
          token_type = Some(TokenType::Comma);
          token_start = self.pos;
        } else if ch == '$' {
          state = LexState::InProgress;
          token_type = Some(TokenType::Address);
          token_start = self.pos;
        }
        else if ch.is_ascii_alphabetic() {
          state = LexState::InProgress;
          token_type = Some(TokenType::Identifier);
          token_start = self.pos;
        }
      }
      if state == LexState::InProgress {
        if let Some(ref token_kind) = token_type {

          if *token_kind == TokenType::Comma {
            // token.push(ch);
            self.chop(src);
            tokens.push(Token::new(
                *token_kind, 
                token_start, 
                1, 
                ",".to_string()
                ));
            token.clear();
            token_type = None;
            state = LexState::Idle;
            continue;
          }

          if *token_kind == TokenType::Identifier {
            if let Some(peeked) = self.peek(src) {
               while let Some(addr_ch) = self.peek(src) {
                 if !addr_ch.is_alphanumeric() { 
                   //self.chop(src);
                    break; 
                 } else {
                   token.push(addr_ch);
                   self.chop(src);
                 }
               }
              tokens.push(Token::new(
                  *token_kind, 
                  token_start, 
                  token.chars().count(), 
                  token.clone()));
              token_type = None;
              token_start = self.pos;
              state = LexState::Idle;
              token = String::new();
              continue;
            }
          }

          if *token_kind == TokenType::Address {
            self.chop(src);

            while let Some(addr_ch) = self.peek(src) {
              token.push(addr_ch);
              self.chop(src);
              if !addr_ch.is_alphanumeric() { break; }
            }
            tokens.push(Token::new(
                *token_kind, 
                token_start, 
                token.chars().count(), 
                token.clone()));
            token_type = None;
            state = LexState::Idle;
            continue;
          }
        }
      }
    }    
    tokens
  }
 }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_lex_one_line_code() {
    let source = "LD A, $FF";
    let mut lex = Lexer::new();
    let tokens = lex.tokenize(source);
    assert_eq!(
      tokens,
      vec![
        Token::new(TokenType::Identifier, 0, 2, "LD".to_string()),
        Token::new(TokenType::Identifier, 3, 1, "A".to_string()),
        Token::new(TokenType::Comma, 4, 1, ",".to_string()),
        Token::new(TokenType::Address, 6, 2, "FF".to_string()),
      ]);
  }
}
