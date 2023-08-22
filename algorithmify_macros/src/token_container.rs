use std::ops::{Index, Range, RangeFrom, RangeInclusive};

use proc_macro::TokenTree;

#[derive(Debug)]
pub(crate) enum TokenContainer<'a> {
    Slice(&'a [TokenTree]),
    Vec(Vec<TokenTree>),
}

impl<'a> TokenContainer<'a> {
    pub(crate) fn len(&self) -> usize {
        match self {
            Self::Slice(slice) => slice.len(),
            Self::Vec(vec) => vec.len(),
        }
    }
}

impl<'a> Index<usize> for TokenContainer<'a> {
    type Output = TokenTree;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Self::Slice(slice) => &slice[index],
            Self::Vec(vec) => &vec[index],
        }
    }
}

impl<'a> Index<Range<usize>> for TokenContainer<'a> {
    type Output = [TokenTree];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        match self {
            Self::Slice(slice) => &slice[index],
            Self::Vec(vec) => &vec[index],
        }
    }
}

impl<'a> Index<RangeFrom<usize>> for TokenContainer<'a> {
    type Output = [TokenTree];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        match self {
            Self::Slice(slice) => &slice[index],
            Self::Vec(vec) => &vec[index],
        }
    }
}

impl<'a> Index<RangeInclusive<usize>> for TokenContainer<'a> {
    type Output = [TokenTree];

    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        match self {
            Self::Slice(slice) => &slice[index],
            Self::Vec(vec) => &vec[index],
        }
    }
}
