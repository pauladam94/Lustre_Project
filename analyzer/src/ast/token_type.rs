use lsp_types::SemanticTokenType;

pub enum TokenType {
    Type,
    Parameter,
    Variable,
    Function,
    Keyword,
    Modifier,
    Comment,
    String,
    Number,
}

impl TokenType {
    pub fn to_vec() -> Vec<SemanticTokenType> {
        vec![
            SemanticTokenType::TYPE,
            SemanticTokenType::PARAMETER,
            SemanticTokenType::VARIABLE,
            SemanticTokenType::FUNCTION,
            SemanticTokenType::KEYWORD,
            SemanticTokenType::MODIFIER,
            SemanticTokenType::COMMENT,
            SemanticTokenType::STRING,
            SemanticTokenType::NUMBER,
        ]
    }
}

#[test]
fn test() {
    let v = TokenType::to_vec();
    assert_eq!(v[TokenType::Type as usize], SemanticTokenType::TYPE);
    assert_eq!(
        v[TokenType::Parameter as usize],
        SemanticTokenType::PARAMETER
    );
    assert_eq!(v[TokenType::Variable as usize], SemanticTokenType::VARIABLE);
    assert_eq!(v[TokenType::Function as usize], SemanticTokenType::FUNCTION);
    assert_eq!(v[TokenType::Keyword as usize], SemanticTokenType::KEYWORD);
    assert_eq!(v[TokenType::Modifier as usize], SemanticTokenType::MODIFIER);
    assert_eq!(v[TokenType::Comment as usize], SemanticTokenType::COMMENT);
    assert_eq!(v[TokenType::String as usize], SemanticTokenType::STRING);
    assert_eq!(v[TokenType::Number as usize], SemanticTokenType::NUMBER);
}
