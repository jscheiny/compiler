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

    pub fn identifier(&mut self) -> Option<LocatedNode<String>> {
        if let Token::Identifier(IdentifierToken(identifier)) = self.peek() {
            let identifier = identifier.clone();
            let span = self.start_span();
            self.next();
            Some(span.close(self, identifier))
        } else {
            None
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

    pub fn start_span(&self) -> NodeSpanTracker {
        NodeSpanTracker::new(self.index)
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

pub fn parse(tokens: Rc<Vec<LocatedToken>>) -> Result<ProgramParseNode, ()> {
    let mut traverser = TokenTraverser::new(tokens);
    program(&mut traverser)
}
