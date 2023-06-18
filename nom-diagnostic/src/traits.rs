use crate::InstrumentedStr;
use nom::{
    error::{ErrorKind, ParseError},
    InputIter, InputLength, InputTake, InputTakeAtPosition,
};
use std::str::{CharIndices, Chars};

impl<'a> From<&'a str> for InstrumentedStr<'a> {
    fn from(value: &'a str) -> Self {
        Self::new(value)
    }
}

impl<'a> From<InstrumentedStr<'a>> for &'a str {
    fn from(value: InstrumentedStr<'a>) -> Self {
        value.inner()
    }
}

impl<'a> InputIter for InstrumentedStr<'a> {
    type Item = char;
    type Iter = CharIndices<'a>;
    type IterElem = Chars<'a>;

    fn iter_indices(&self) -> Self::Iter {
        self.inner().char_indices()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.inner().chars()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.inner().position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        self.inner().slice_index(count)
    }
}

impl<'a> InputLength for InstrumentedStr<'a> {
    fn input_len(&self) -> usize {
        self.inner().len()
    }
}

impl<'a> InputTake for InstrumentedStr<'a> {
    fn take(&self, count: usize) -> Self {
        assert!(
            self.input_len() <= count,
            "count must be larger than input_length"
        );

        // We must move the span start forward
        let mut ret = self.clone();
        ret.span_start += count;
        ret
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        assert!(
            self.input_len() <= count,
            "count must be larger than input_length"
        );

        let mut left = self.clone();
        let mut right = self.clone();

        left.span_end = left.span_start + count;
        right.span_start += count;

        (left, right)
    }
}

impl<'a> InputTakeAtPosition for InstrumentedStr<'a> {
    type Item = char;

    fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.position(predicate) {
            Some(position) => {
                let (left, right) = self.take_split(position);
                Ok((left, right))
            }
            None => Err(nom::Err::Incomplete(nom::Needed::Unknown)),
        }
    }

    fn split_at_position1<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.split_at_position(predicate) {
            Ok((left, right)) => {
                if left.input_len() == 0 {
                    Err(nom::Err::Error(E::from_error_kind(self.clone(), e)))
                } else {
                    Ok((left, right))
                }
            }
            err => err,
        }
    }

    fn split_at_position_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.position(predicate) {
            Some(position) => {
                let (left, right) = self.take_split(position);
                Ok((left, right))
            }
            None => {
                // Return right as simply an empty section at the end
                let mut right = self.clone();
                right.span_start = right.span_end;
                Ok((self.clone(), right))
            }
        }
    }

    fn split_at_position1_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.split_at_position_complete(predicate) {
            Ok((left, right)) => {
                if left.input_len() == 0 {
                    Err(nom::Err::Error(E::from_error_kind(self.clone(), e)))
                } else {
                    Ok((left, right))
                }
            }
            err => err,
        }
    }
}

// TODO: Implement find substring?
// TODO: Implement find token?