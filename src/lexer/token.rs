use super::Span;
use thiserror::Error;

pub trait Parse {
    fn parse(tokens: &mut Vec<Token>, data: &[(usize, char)]) -> Result<()>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    variant: TokenVariant,
    span: Span,
}

impl Parse for Token {
    fn parse(tokens: &mut Vec<Token>, data: &[(usize, char)]) -> Result<()> {
        for parser in [
            Delimiter::parse,
            Operator::parse,
            Keyword::parse,
            Literal::parse,
            Identifier::parse,
        ] {
            if parser(tokens, data).is_ok() {
                return Ok(());
            }
        }

        Unknown::parse(tokens, data)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenVariant {
    Delimiter(Delimiter),
    Operator(Operator),
    Keyword(Keyword),
    Literal(Literal),
    Identifier(String),
    Unknown(String),
}

// ============================
//  Order to parse variants in
// ============================
// 1) delimiter  (if span less than 2)
// 2) operator   (if span less than 3)
// 3) keyword    (match against list)
// 4) literal    (match against constraints)
// 5) identifier (match against constraints)

#[derive(Debug, PartialEq, Eq)]
pub enum Delimiter {
    Parenthesis(DelimiterDirection),
    Bracket(DelimiterDirection),
    CurlyBrace(DelimiterDirection),
    // Carrot(DelimiterDirection),
    Comma,
    SemiColon,
    SingleColon,
    // DoubleColon,
}

impl Parse for Delimiter {
    fn parse(tokens: &mut Vec<Token>, data: &[(usize, char)]) -> Result<()> {
        let maybe_word = data.iter().map(|(_, char)| char).collect::<String>();
        let variant = match maybe_word.as_str() {
            "(" => Delimiter::Parenthesis(DelimiterDirection::Open),
            ")" => Delimiter::Parenthesis(DelimiterDirection::Close),
            "[" => Delimiter::Bracket(DelimiterDirection::Open),
            "]" => Delimiter::Bracket(DelimiterDirection::Close),
            "{" => Delimiter::CurlyBrace(DelimiterDirection::Open),
            "}" => Delimiter::CurlyBrace(DelimiterDirection::Close),
            "," => Delimiter::Comma,
            ";" => Delimiter::SemiColon,
            ":" => Delimiter::SingleColon,
            _ => return Err(Error::Delimiter),
        };

        let span = Span {
            start: data[0].0,
            end: data.last().unwrap().0,
        };

        tokens.push(Token {
            variant: TokenVariant::Delimiter(variant),
            span,
        });

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum DelimiterDirection {
    Open,
    Close,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Assignement,
    Negation,
    Comparison(ComparisonOperator),
    ArithmeticOrLogical(ArithmeticOrLogicalOperator),
    LazyBoolean(LazyBooleanOperator),
}

impl Parse for Operator {
    fn parse(tokens: &mut Vec<Token>, data: &[(usize, char)]) -> Result<()> {
        let maybe_word = data.iter().map(|(_, char)| char).collect::<String>();
        let variant = match maybe_word.as_str() {
            "=" => Operator::Assignement,
            "!" => Operator::Negation,
            "==" => Operator::Comparison(ComparisonOperator::Equals),
            "<" => Operator::Comparison(ComparisonOperator::LessThan),
            ">" => Operator::Comparison(ComparisonOperator::GreaterThan),
            "<=" => Operator::Comparison(ComparisonOperator::LessThanOrEqual),
            ">=" => Operator::Comparison(ComparisonOperator::GreaterThanOrEqual),
            "!=" => Operator::Comparison(ComparisonOperator::NotEqaul),
            "+" => Operator::ArithmeticOrLogical(ArithmeticOrLogicalOperator::Addition),
            "-" => Operator::ArithmeticOrLogical(ArithmeticOrLogicalOperator::Substraction),
            "*" => Operator::ArithmeticOrLogical(ArithmeticOrLogicalOperator::Multiplication),
            "/" => Operator::ArithmeticOrLogical(ArithmeticOrLogicalOperator::Division),
            "%" => Operator::ArithmeticOrLogical(ArithmeticOrLogicalOperator::Remainder),
            "+=" => Operator::ArithmeticOrLogical(ArithmeticOrLogicalOperator::AdditionAssignement),
            "-=" => {
                Operator::ArithmeticOrLogical(ArithmeticOrLogicalOperator::SubtractionAssignement)
            }
            "*=" => Operator::ArithmeticOrLogical(
                ArithmeticOrLogicalOperator::MultiplicationAssignement,
            ),
            "/=" => Operator::ArithmeticOrLogical(ArithmeticOrLogicalOperator::DivisionAssignement),
            "%=" => {
                Operator::ArithmeticOrLogical(ArithmeticOrLogicalOperator::RemainderAssignement)
            }
            "&" => Operator::ArithmeticOrLogical(ArithmeticOrLogicalOperator::And),
            "|" => Operator::ArithmeticOrLogical(ArithmeticOrLogicalOperator::Or),
            "^" => Operator::ArithmeticOrLogical(ArithmeticOrLogicalOperator::Xor),
            "<<" => Operator::ArithmeticOrLogical(ArithmeticOrLogicalOperator::LeftShift),
            ">>" => Operator::ArithmeticOrLogical(ArithmeticOrLogicalOperator::RightShift),
            "&&" => Operator::LazyBoolean(LazyBooleanOperator::And),
            "||" => Operator::LazyBoolean(LazyBooleanOperator::Or),
            _ => return Err(Error::Operator),
        };

        // SAFETY: we know data contains characters, as we would have already returned otherwise
        let span = Span {
            start: data[0].0,
            end: data.last().unwrap().0,
        };

        tokens.push(Token {
            variant: TokenVariant::Operator(variant),
            span,
        });

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ComparisonOperator {
    Equals,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    NotEqaul,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArithmeticOrLogicalOperator {
    Addition,
    Substraction,
    Multiplication,
    Division,
    Remainder,
    AdditionAssignement,
    SubtractionAssignement,
    MultiplicationAssignement,
    DivisionAssignement,
    RemainderAssignement,
    And,
    Or,
    Xor,
    LeftShift,
    RightShift,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LazyBooleanOperator {
    And,
    Or,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Keyword {
    Auto,
    Bool,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    False,
    Float,
    For,
    If,
    Inline,
    Int,
    Loop,
    Long,
    Register,
    Restrict,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    True,
    TypeDef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,
}

impl Parse for Keyword {
    fn parse(tokens: &mut Vec<Token>, data: &[(usize, char)]) -> Result<()> {
        let maybe_word = data.iter().map(|(_, char)| char).collect::<String>();
        let variant = match maybe_word.as_str() {
            "auto" => Keyword::Auto,
            "bool" => Keyword::Bool,
            "break" => Keyword::Break,
            "case" => Keyword::Case,
            "char" => Keyword::Char,
            "const" => Keyword::Const,
            "continue" => Keyword::Continue,
            "default" => Keyword::Default,
            "do" => Keyword::Do,
            "double" => Keyword::Double,
            "else" => Keyword::Else,
            "enum" => Keyword::Enum,
            "extern" => Keyword::Extern,
            "false" => Keyword::False,
            "float" => Keyword::Float,
            "for" => Keyword::For,
            "if" => Keyword::If,
            "inline" => Keyword::Inline,
            "int" => Keyword::Int,
            "loop" => Keyword::Loop,
            "long" => Keyword::Long,
            "register" => Keyword::Register,
            "restrict" => Keyword::Restrict,
            "return" => Keyword::Return,
            "short" => Keyword::Short,
            "signed" => Keyword::Signed,
            "sizeof" => Keyword::Sizeof,
            "static" => Keyword::Static,
            "struct" => Keyword::Struct,
            "switch" => Keyword::Switch,
            "true" => Keyword::True,
            "typedef" => Keyword::TypeDef,
            "union" => Keyword::Union,
            "unsigned" => Keyword::Unsigned,
            "void" => Keyword::Void,
            "volatile" => Keyword::Volatile,
            "while" => Keyword::While,
            _ => return Err(Error::Keyword),
        };

        // SAFETY: we know data contains characters, as we would have already returned otherwise
        let span = Span {
            start: data[0].0,
            end: data.last().unwrap().0,
        };

        tokens.push(Token {
            variant: TokenVariant::Keyword(variant),
            span,
        });

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Literal {
    variant: LiteralVariant,
    contents: String,
}

impl Parse for Literal {
    fn parse(tokens: &mut Vec<Token>, data: &[(usize, char)]) -> Result<()> {
        let maybe_word = data.iter().map(|(_, char)| char).collect::<String>();
        // SAFETY: we know data contains characters, as we would have already returned otherwise
        let span = Span {
            start: data[0].0,
            end: data.last().unwrap().0,
        };

        let variant = LiteralVariant::try_from(maybe_word.as_str())?;

        tokens.push(Token {
            variant: TokenVariant::Literal(Literal {
                variant,
                contents: maybe_word,
            }),
            span,
        });

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LiteralVariant {
    String,
    Number,
}

impl TryFrom<&str> for LiteralVariant {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let chars = value.chars().collect::<Vec<_>>();
        let len = chars.len();

        match chars[0] {
            '"' if len >= 2 => {
                if chars[len - 1] != '"' && (len >= 3 && chars[len - 2] == '\\') {
                    return Err(Error::Literal);
                }

                Ok(Self::String)
            }
            // TODO: add support for hex and binary codes
            // '0' if len >= 3 => {
            //     chars[1] == 'x'

            //     todo!()
            // }
            _ => {
                let mut decimal_points = 0;
                for char in chars {
                    if char == '.' {
                        if decimal_points == 1 {
                            return Err(Error::Literal);
                        }

                        decimal_points += 1;
                        continue;
                    }

                    if !char.is_numeric() {
                        return Err(Error::Literal);
                    }
                }

                Ok(Self::Number)
            }
        }
    }
}

pub struct Identifier;

impl Parse for Identifier {
    fn parse(tokens: &mut Vec<Token>, data: &[(usize, char)]) -> Result<()> {
        let maybe_word = data.iter().map(|(_, char)| char).collect::<String>();
        // SAFETY: we know data contains characters, as we would have already returned otherwise
        let span = Span {
            start: data[0].0,
            end: data.last().unwrap().0,
        };

        let chars = maybe_word.chars();
        let first = chars.clone().collect::<Vec<char>>()[0];
        let first_allowed = first.is_alphabetic() || first == '_';
        let rest_allowed = chars.fold(true, |acc, char| {
            acc && (char.is_alphanumeric() || char == '_')
        });

        if !(first_allowed && rest_allowed) {
            return Err(Error::Identifier);
        }

        tokens.push(Token {
            variant: TokenVariant::Identifier(maybe_word),
            span,
        });

        Ok(())
    }
}

pub struct Unknown;

impl Parse for Unknown {
    fn parse(tokens: &mut Vec<Token>, data: &[(usize, char)]) -> Result<()> {
        let maybe_word = data.iter().map(|(_, char)| char).collect::<String>();
        // SAFETY: we know data contains characters, as we would have already returned otherwise
        let span = Span {
            start: data[0].0,
            end: data.last().unwrap().0,
        };

        tokens.push(Token {
            variant: TokenVariant::Unknown(maybe_word),
            span,
        });

        Ok(())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("")]
    Delimiter,
    #[error("")]
    Operator,
    #[error("")]
    Keyword,
    #[error("")]
    Literal,
    #[error("")]
    Identifier,
}
