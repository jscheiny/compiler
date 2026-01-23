use std::fmt::Debug;
use std::rc::Rc;

use crate::{
    lexer::{IdentifierToken, LocatedToken, Token},
    parser::{LocatedNode, NodeSpanTracker, ParserPredicate, ProgramParseNode, grammar::program},
};

pub struct TokenTraverser {
    tokens: Rc<Vec<LocatedToken>>,
    index: usize,
}

impl TokenTraverser {
    fn new(tokens: Rc<Vec<LocatedToken>>) -> Self {
        TokenTraverser { tokens, index: 0 }
    }

    pub fn accept(&mut self, predicate: &impl ParserPredicate) -> bool {
        if predicate.is_match(self.peek()) {
            self.next();
            true
        } else {
            false
        }
    }

    pub fn expect(&mut self, predicate: &impl ParserPredicate) -> Result<(), ()> {
        if self.accept(predicate) {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn identifier(&mut self) -> Result<LocatedNode<String>, ()> {
        self.located(identifier)
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

    pub fn start_span(&self) -> NodeSpanTracker {
        NodeSpanTracker::new(self.index)
    }

    pub fn located<P: Debug>(
        &mut self,
        parse: impl Fn(&mut TokenTraverser) -> Result<P, ()>,
    ) -> Result<LocatedNode<P>, ()> {
        let token_start_index = self.index;
        let value = parse(self)?;
        Ok(LocatedNode {
            value,
            token_start_index,
            token_end_index: self.index,
        })
    }

    pub fn maybe_located<P: Debug>(
        &mut self,
        parse: impl Fn(&mut TokenTraverser) -> Result<Option<P>, ()>,
    ) -> Result<Option<LocatedNode<P>>, ()> {
        let token_start_index = self.index;
        let result = parse(self)?;
        Ok(result.map(|value| LocatedNode {
            value,
            token_start_index,
            token_end_index: self.index,
        }))
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

fn identifier(tokens: &mut TokenTraverser) -> Result<String, ()> {
    if let Token::Identifier(IdentifierToken(identifier)) = tokens.peek() {
        let identifier = identifier.clone();
        tokens.next();
        Ok(identifier)
    } else {
        Err(())
    }
}

pub fn parse(tokens: Rc<Vec<LocatedToken>>) -> Result<ProgramParseNode, ()> {
    let mut traverser = TokenTraverser::new(tokens);
    program(&mut traverser)
}
