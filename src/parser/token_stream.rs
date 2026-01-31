use std::fmt::Debug;
use std::rc::Rc;

use crate::{
    lexer::{LocatedToken, Token, TokenMatch},
    parser::{ParseNode, SyntaxError, SyntaxErrorType, TokenSpan},
};

pub struct TokenStream {
    tokens: Rc<Vec<LocatedToken>>,
    index: usize,
    pub errors: Vec<SyntaxError>,
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

    pub fn expect(&mut self, predicate: &impl TokenMatch) -> Result<(), SyntaxError> {
        if self.accept(predicate) {
            Ok(())
        } else {
            Err(self.make_error(SyntaxErrorType::Unimplemented))
        }
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.index].token
    }

    pub fn next(&mut self) {
        self.index += 1;
    }

    pub fn is_done(&self) -> bool {
        self.index == self.tokens.len()
    }

    pub fn located<P: Debug, E>(
        &mut self,
        parse: impl Fn(&mut TokenStream) -> Result<P, E>,
    ) -> Result<ParseNode<P>, E> {
        let start_index = self.index;
        let value = parse(self)?;
        Ok(ParseNode {
            value,
            span: self.span(start_index),
        })
    }

    pub fn maybe_located<P: Debug, E>(
        &mut self,
        parse: impl Fn(&mut TokenStream) -> Result<Option<P>, E>,
    ) -> Result<Option<ParseNode<P>>, E> {
        let start_index = self.index;
        let result = parse(self)?;
        Ok(result.map(|value| ParseNode {
            value,
            span: self.span(start_index),
        }))
    }

    fn span(&self, start_index: usize) -> TokenSpan {
        TokenSpan {
            start_index,
            end_index: self.index - 1,
        }
    }

    pub fn current_span(&self) -> TokenSpan {
        TokenSpan {
            start_index: self.index,
            end_index: self.index,
        }
    }

    pub fn push_error(&mut self, kind: SyntaxErrorType) {
        self.errors.push(self.make_error(kind));
    }

    pub fn make_error(&self, kind: SyntaxErrorType) -> SyntaxError {
        SyntaxError {
            span: self.current_span(),
            kind,
        }
    }
}
