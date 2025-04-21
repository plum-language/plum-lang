use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\r\f]+")]
#[logos(skip r"//.*")]
pub enum Token {
    #[regex("[абвгдђежзијклљмнњопрстћуфхцчџшАБВГДЂЕЖЗИЈКЛЉМНЊОПРСТЋУФХЦЧЏШ_][абвгдђежзијклљмнњопрстћуфхцчџшАБВГДЂЕЖЗИЈКЛЉМНЊОПРСТЋУФХЦЧЏШ0-9_-]*", plum_identifier, priority = 5)]
    #[regex(
        "[a-zđžšćčA-ZĐŽŠŽĆ_][a-zđžšćčA-ZĐŽŠŽĆ0-9_]*",
        plum_identifier,
        priority = 3
    )]
    Identifier(String),
    #[regex(r#""[^"]*""#, plum_string, priority = 1)]
    String(String),
    #[regex(r#"[0-9]+"#, plum_integer, priority = 1)]
    Integer(i64),
    #[regex(r#"[0-9]+(\.[0-9]+)"#, plum_float, priority = 1)]
    Float(f64),

    #[token("(")]
    LtParen,
    #[token(")")]
    RtParen,
    #[token("[")]
    LtBracket,
    #[token("]")]
    RtBracket,
    #[token("{")]
    LtBrace,
    #[token("}")]
    RtBrace,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token("/")]
    Divide,
    #[token("*")]
    Multiply,
    #[token("=")]
    Assignment,
    #[token(">")]
    BiggerThan,
    #[token("<")]
    LessThan,
    #[token("!")]
    Negate,

    #[token("==")]
    Equals,
    #[token(">=")]
    GreaterOrEqualThan,
    #[token("<=")]
    LesserOrEqualThan,
    #[token(":")]
    ScopeResolution,

    #[token("ако")]
    #[token("ako")]
    If,

    #[token("иначе")]
    #[token("inače")]
    #[token("inace")]
    Else,

    #[token("тури")]
    #[token("turi")]
    Let,

    #[token("посо")]
    #[token("poso")]
    Function,

    #[token("бекни")]
    #[token("bekni")]
    Print,

    #[token("углоби")]
    #[token("uglobi")]
    Import,

    #[token("врни")]
    #[token("vrni")]
    Return,

    #[token("сорта")]
    #[token("sorta")]
    Class,

    #[token("или")]
    #[token("ili")]
    Or,

    #[token("и")]
    #[token("i")]
    And,

    #[token("терај")]
    #[token("teraj")]
    For,

    #[token("док")]
    #[token("dok")]
    While,

    #[token("ја")]
    #[token("ja")]
    This,

    #[token("татко")]
    #[token("tatko")]
    Super,

    #[token("газда")]
    #[token("gazda")]
    Main,

    #[token("dabome", |_| true)]
    #[token("jok", |_| false)]
    #[token("дабоме", |_| true)]
    #[token("јок", |_| false)]
    Bool(bool),

    #[token("бунар")]
    #[token("bunar")]
    Null,
}

fn plum_integer(lexer: &mut logos::Lexer<Token>) -> i64 {
    let slice = lexer.slice();
    slice.parse::<i64>().expect(
        format!(
            "Error parsing int {}:{}",
            lexer.span().start,
            lexer.span().end
        )
        .as_str(),
    )
}

fn plum_float(lexer: &mut logos::Lexer<Token>) -> f64 {
    let slice = lexer.slice();
    slice.parse::<f64>().expect(
        format!(
            "Error parsing float {}:{}",
            lexer.span().start,
            lexer.span().end
        )
        .as_str(),
    )
}

fn plum_string(lexer: &mut logos::Lexer<Token>) -> String {
    let slice = lexer.slice();
    slice[1..slice.len() - 1].to_string()
}

fn plum_identifier(lexer: &mut logos::Lexer<Token>) -> String {
    let slice = lexer.slice();
    slice.to_string()
}

#[cfg(test)]
mod tests {
    use crate::lexer::Token;
    use logos::Logos;

    #[test]
    fn parser_test_lat() {
        let src = r#"
        uglobi "zadruga/sorte".

        sorta Doručak {
            turi jaja = bunar.

            (jaja) {
                ja:jaja = jaja.
            }

            poso isprži komada {
                ako (komada == 0 ili komada == bunar) {
                    bekni "Isprženo #{jaja} komada".
                } inače {
                    bekni "Isprženo #{komada} komada".
                }
            }
        }

        poso gazda {
            turi obrok = Doručak(5).
            obrok:isprži(5).

            vrni 0.
        }
        "#;

        let mut lex = Token::lexer(src).peekable();

        assert_eq!(lex.next(), Some(Ok(Token::Import)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::String("zadruga/sorte".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::Class)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Identifier("Doručak".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::LtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Let)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier("jaja".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Assignment)));
        assert_eq!(lex.next(), Some(Ok(Token::Null)));
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::LtParen)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier("jaja".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::RtParen)));
        assert_eq!(lex.next(), Some(Ok(Token::LtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::This)));
        assert_eq!(lex.next(), Some(Ok(Token::ScopeResolution)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier("jaja".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Assignment)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier("jaja".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::RtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Function)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Identifier("isprži".to_string())))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Identifier("komada".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::LtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::If)));
        assert_eq!(lex.next(), Some(Ok(Token::LtParen)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Identifier("komada".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::Equals)));
        assert_eq!(lex.next(), Some(Ok(Token::Integer(0))));
        assert_eq!(lex.next(), Some(Ok(Token::Or)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Identifier("komada".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::Equals)));
        assert_eq!(lex.next(), Some(Ok(Token::Null)));
        assert_eq!(lex.next(), Some(Ok(Token::RtParen)));
        assert_eq!(lex.next(), Some(Ok(Token::LtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Print)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::String("Isprženo #{jaja} komada".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::RtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Else)));
        assert_eq!(lex.next(), Some(Ok(Token::LtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Print)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::String("Isprženo #{komada} komada".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::RtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::RtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::RtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Function)));
        assert_eq!(lex.next(), Some(Ok(Token::Main)));
        assert_eq!(lex.next(), Some(Ok(Token::LtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Let)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier("obrok".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Assignment)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Identifier("Doručak".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::LtParen)));
        assert_eq!(lex.next(), Some(Ok(Token::Integer(5))));
        assert_eq!(lex.next(), Some(Ok(Token::RtParen)));
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier("obrok".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::ScopeResolution)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Identifier("isprži".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::LtParen)));
        assert_eq!(lex.next(), Some(Ok(Token::Integer(5))));
        assert_eq!(lex.next(), Some(Ok(Token::RtParen)));
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::Return)));
        assert_eq!(lex.next(), Some(Ok(Token::Integer(0))));
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::RtBrace)));
    }

    #[test]
    fn parser_test_cyr() {
        let src = r#"
        углоби "zadruga/sorte".

        сорта Доручак {
            тури јаја = бунар.

            (јаја) {
                ја:јаја = јаја.
            }

            посо испржи комада {
                ако (комада == 0 или комада == бунар) {
                    бекни "Испржено #{јаја} комада".
                } иначе {
                    бекни "Испржено #{комада} комада".
                }
            }
        }

        посо газда {
            тури оброк = Доручак(5).
            оброк:испржи(5).

            врни 0.
        }
        "#;

        let mut lex = Token::lexer(src);

        assert_eq!(lex.next(), Some(Ok(Token::Import)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::String("zadruga/sorte".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::Class)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Identifier("Доручак".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::LtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Let)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier("јаја".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Assignment)));
        assert_eq!(lex.next(), Some(Ok(Token::Null)));
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::LtParen)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier("јаја".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::RtParen)));
        assert_eq!(lex.next(), Some(Ok(Token::LtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::This)));
        assert_eq!(lex.next(), Some(Ok(Token::ScopeResolution)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier("јаја".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Assignment)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier("јаја".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::RtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Function)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Identifier("испржи".to_string())))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Identifier("комада".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::LtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::If)));
        assert_eq!(lex.next(), Some(Ok(Token::LtParen)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Identifier("комада".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::Equals)));
        assert_eq!(lex.next(), Some(Ok(Token::Integer(0))));
        assert_eq!(lex.next(), Some(Ok(Token::Or)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Identifier("комада".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::Equals)));
        assert_eq!(lex.next(), Some(Ok(Token::Null)));
        assert_eq!(lex.next(), Some(Ok(Token::RtParen)));
        assert_eq!(lex.next(), Some(Ok(Token::LtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Print)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::String("Испржено #{јаја} комада".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::RtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Else)));
        assert_eq!(lex.next(), Some(Ok(Token::LtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Print)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::String("Испржено #{комада} комада".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::RtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::RtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::RtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Function)));
        assert_eq!(lex.next(), Some(Ok(Token::Main)));
        assert_eq!(lex.next(), Some(Ok(Token::LtBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Let)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier("оброк".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Assignment)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Identifier("Доручак".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::LtParen)));
        assert_eq!(lex.next(), Some(Ok(Token::Integer(5))));
        assert_eq!(lex.next(), Some(Ok(Token::RtParen)));
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier("оброк".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::ScopeResolution)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Identifier("испржи".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::LtParen)));
        assert_eq!(lex.next(), Some(Ok(Token::Integer(5))));
        assert_eq!(lex.next(), Some(Ok(Token::RtParen)));
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::Return)));
        assert_eq!(lex.next(), Some(Ok(Token::Integer(0))));
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::RtBrace)));
    }
}

