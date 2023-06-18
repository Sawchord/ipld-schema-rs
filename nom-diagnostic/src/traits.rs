use crate::{ErrorDiagnose, InstrumentedStr};
use nom::{
    error::{ErrorKind, ParseError},
    Compare, InputIter, InputLength, InputTake, InputTakeAtPosition, Offset, Slice,
};
use std::{
    error::Error as StdError,
    ops::{Range, RangeFrom, RangeFull, RangeTo},
    str::{CharIndices, Chars},
};

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
            self.input_len() > count,
            "count must be larger than input_length"
        );

        // We must move the span start forward
        let mut ret = self.clone();
        ret.span_end = ret.span_start + count;
        ret
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        assert!(
            self.input_len() > count,
            "count must be larger than input_length"
        );

        let mut prefix = self.clone();
        let mut suffix = self.clone();

        prefix.span_end = prefix.span_start + count;
        suffix.span_start += count;

        (suffix, prefix)
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
                right.span_start = self.span_end;
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

impl<'a> Offset for InstrumentedStr<'a> {
    fn offset(&self, second: &Self) -> usize {
        if self.src != second.src {
            usize::MAX
        } else {
            self.span_start.saturating_sub(second.span_start)
        }
    }
}

impl<'a, S> Compare<S> for InstrumentedStr<'a>
where
    S: Into<&'a str>,
{
    fn compare(&self, t: S) -> nom::CompareResult {
        self.inner().compare(t.into())
    }

    fn compare_no_case(&self, t: S) -> nom::CompareResult {
        self.inner().compare_no_case(t.into())
    }
}

impl<'a> Slice<Range<usize>> for InstrumentedStr<'a> {
    fn slice(&self, range: Range<usize>) -> Self {
        self.slice(..range.end).slice(range.start..)
    }
}

impl<'a> Slice<RangeTo<usize>> for InstrumentedStr<'a> {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        let mut ret = self.clone();
        ret.span_end = std::cmp::min(ret.span_start + range.end, ret.span_end);
        ret
    }
}

impl<'a> Slice<RangeFrom<usize>> for InstrumentedStr<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        let mut ret = self.clone();
        ret.span_start = std::cmp::min(ret.span_start + range.start, ret.span_end);
        ret
    }
}

impl<'a> Slice<RangeFull> for InstrumentedStr<'a> {
    fn slice(&self, _: RangeFull) -> Self {
        self.clone()
    }
}

// TODO: Implement find substring?
// TODO: Implement find token?

impl<'a, T> ParseError<InstrumentedStr<'a>> for ErrorDiagnose<'a, T>
where
    T: StdError + Default,
{
    fn from_error_kind(input: InstrumentedStr<'a>, _: ErrorKind) -> Self {
        ErrorDiagnose {
            src: input.src,
            file: input.file,
            span_start: input.span_start,
            span_end: input.span_end,
            error: T::default(),
        }
    }

    fn append(_: InstrumentedStr<'a>, _: ErrorKind, other: Self) -> Self {
        other
    }
}
