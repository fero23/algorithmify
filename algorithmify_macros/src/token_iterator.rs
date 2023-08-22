use proc_macro::TokenTree;

use crate::token_container::TokenContainer;

#[derive(Debug)]
pub(crate) struct TokenIterator<'a> {
    tokens: TokenContainer<'a>,
    pub index: usize,
}

impl From<Vec<TokenTree>> for TokenIterator<'static> {
    fn from(value: Vec<TokenTree>) -> Self {
        TokenIterator {
            tokens: TokenContainer::Vec(value),
            index: 0,
        }
    }
}

impl<'a> From<&'a [TokenTree]> for TokenIterator<'a> {
    fn from(value: &'a [TokenTree]) -> Self {
        TokenIterator {
            tokens: TokenContainer::Slice(value),
            index: 0,
        }
    }
}

impl<'a> TokenIterator<'a> {
    pub(crate) fn rewind_to(&mut self, index: usize) {
        self.index = index;
    }

    pub(crate) fn peek(&self) -> Option<&TokenTree> {
        if self.index >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.index])
        }
    }

    pub(crate) fn next(&mut self) -> Option<&TokenTree> {
        if self.index >= self.tokens.len() {
            None
        } else {
            let next = Some(&self.tokens[self.index]);
            self.index += 1;
            next
        }
    }

    pub(crate) fn try_get_next_token(&mut self, token: &str) -> Option<()> {
        if self.index < self.tokens.len() && self.tokens[self.index].to_string() == token {
            self.index += 1;
            Some(())
        } else {
            None
        }
    }

    pub(crate) fn next_nth(&mut self, count: usize) -> Option<&[TokenTree]> {
        if self.index < self.tokens.len() {
            let end_index = if self.index + count < self.tokens.len() {
                self.index + count
            } else {
                self.tokens.len()
            };
            let slice = &self.tokens[self.index..end_index];
            self.index = end_index;
            Some(slice)
        } else {
            None
        }
    }

    pub(crate) fn next_nth_string(&mut self, count: usize) -> Option<String> {
        if self.index + count >= self.tokens.len() {
            None
        } else {
            let result = (self.index..self.index + count).fold(String::new(), |acc, index| {
                acc + &self.tokens[index].to_string()
            });
            self.index += count;
            Some(result)
        }
    }

    pub(crate) fn get_until_delimiter(&mut self, token: &str) -> Option<&[TokenTree]> {
        if let Some((index, _)) = self.tokens[self.index..]
            .iter()
            .enumerate()
            .find(|(_, t)| t.to_string() == token)
        {
            let slice = &self.tokens[self.index..self.index + index];
            self.index += slice.len() + 1;
            Some(slice)
        } else {
            None
        }
    }
}
