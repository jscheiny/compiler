use std::rc::Rc;

use crate::{
    lexer::{LocatedToken, Token, TokenMatch},
    parser::{
        IdentifierNode, IdentifierType, LocatedSyntaxError, ParseNode, ParseResult,
        SyntaxError, TokenSpan, identifier,
    },
};

pub struct TokenStream {
    tokens: Rc<Vec<LocatedToken>>,
    index: usize,
    pub errors: Vec<LocatedSyntaxError>,
}

impl TokenStream {
    pub fn from(tokens: Rc<Vec<LocatedToken>>) -> Self {
        TokenStream {
            tokens,
            index: 0,
            errors: vec![],
        }
    }

    pub fn accept(&mut self, predicate: &impl TokenMatch) -> bool {
        if predicate.matches(self.peek()) {
            self.next();
            true
        } else {
            false
        }
    }

    pub fn expect(&mut self, predicate: &impl TokenMatch, error: SyntaxError) -> ParseResult<()> {
        if self.accept(predicate) {
            Ok(())
        } else {
            Err(self.make_error(error))
        }
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.index].token
    }

    pub fn next(&mut self) {
        self.index += 1;
    }

    pub fn is_done(&self) -> bool {
        // The last token is an EOF token
        self.index >= self.tokens.len() - 1
    }

    pub fn identifier(
        &mut self,
        id_type: IdentifierType,
    ) -> ParseResult<ParseNode<IdentifierNode>> {
        let start_index = self.index;
        let value = identifier(self, id_type)?;
        Ok(self.close(value, start_index))
    }

    pub fn located<P, E>(
        &mut self,
        parse: impl Fn(&mut TokenStream) -> Result<P, E>,
    ) -> Result<ParseNode<P>, E> {
        let start_index = self.index;
        let value = parse(self)?;
        Ok(self.close(value, start_index))
    }

    pub fn located_with<P, Arg, E>(
        &mut self,
        parse: impl Fn(&mut TokenStream, Arg) -> Result<P, E>,
        arg: Arg,
    ) -> Result<ParseNode<P>, E> {
        let start_index = self.index;
        let value = parse(self, arg)?;
        Ok(self.close(value, start_index))
    }

    pub fn index(&self) -> usize {
        self.index
    }

    fn close<P>(&self, value: P, start_index: usize) -> ParseNode<P> {
        ParseNode {
            value,
            span: TokenSpan {
                start_index,
                end_index: self.index - 1,
            },
        }
    }

    pub fn current_span(&self) -> TokenSpan {
        TokenSpan {
            start_index: self.index,
            end_index: self.index,
        }
    }

    pub fn push_error(&mut self, kind: SyntaxError) {
        self.errors.push(self.make_error(kind));
    }

    pub fn make_error(&self, kind: SyntaxError) -> LocatedSyntaxError {
        LocatedSyntaxError {
            span: self.current_span(),
            error: kind,
        }
    }
}
