use crate::{ErrorDiagnose, InStr, Span};
use nom::{
    error::{ErrorKind, ParseError},
    Compare, InputIter, InputLength, InputTake, InputTakeAtPosition, Offset, Slice,
};
use std::{
    error::Error as StdError,
    ops::{Range, RangeFrom, RangeFull, RangeTo},
    str::{CharIndices, Chars},
};

impl<'a> From<&'a str> for InStr<'a> {
    fn from(value: &'a str) -> Self {
        Self::new(value)
    }
}

impl<'a> From<InStr<'a>> for &'a str {
    fn from(value: InStr<'a>) -> Self {
        value.inner()
    }
}

impl<'a> InputIter for InStr<'a> {
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

impl<'a> InputLength for InStr<'a> {
    fn input_len(&self) -> usize {
        self.inner().len()
    }
}

impl<'a> InputTake for InStr<'a> {
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
            self.input_len() >= count,
            "count must be larger than input_length"
        );

        let mut prefix = self.clone();
        let mut suffix = self.clone();

        prefix.span_end = prefix.span_start + count;
        suffix.span_start += count;

        (suffix, prefix)
    }
}

impl<'a> InputTakeAtPosition for InStr<'a> {
    type Item = char;

    fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.position(predicate) {
            Some(position) => {
                let (suffix, prefix) = self.take_split(position);
                Ok((suffix, prefix))
            }
            None => Err(nom::Err::Incomplete(nom::Needed::new(1))),
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
            Ok((suffix, prefix)) => {
                if prefix.input_len() == 0 {
                    Err(nom::Err::Error(E::from_error_kind(self.clone(), e)))
                } else {
                    Ok((suffix, prefix))
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
                let (suffix, prefix) = self.take_split(position);
                Ok((suffix, prefix))
            }
            None => {
                // Suffix is simply an empty section while prefix is the whole rest of the output
                let mut suffix = self.clone();
                suffix.span_start = self.span_end;
                Ok((suffix, self.clone()))
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
            Ok((suffix, prefix)) => {
                if prefix.input_len() == 0 {
                    Err(nom::Err::Error(E::from_error_kind(self.clone(), e)))
                } else {
                    Ok((suffix, prefix))
                }
            }
            err => err,
        }
    }
}

impl<'a> Offset for InStr<'a> {
    fn offset(&self, second: &Self) -> usize {
        if self.src != second.src {
            usize::MAX
        } else {
            self.span_start.saturating_sub(second.span_start)
        }
    }
}

impl<'a, 'b> Compare<&'b str> for InStr<'a> {
    fn compare(&self, t: &'b str) -> nom::CompareResult {
        self.inner().compare(t)
    }

    fn compare_no_case(&self, t: &'b str) -> nom::CompareResult {
        self.inner().compare_no_case(t)
    }
}

impl<'a, 'b> Compare<&'b [u8]> for InStr<'a> {
    fn compare(&self, t: &'b [u8]) -> nom::CompareResult {
        self.inner().compare(t)
    }

    fn compare_no_case(&self, t: &'b [u8]) -> nom::CompareResult {
        self.inner().compare_no_case(t)
    }
}

impl<'a> Slice<Range<usize>> for InStr<'a> {
    fn slice(&self, range: Range<usize>) -> Self {
        self.slice(..range.end).slice(range.start..)
    }
}

impl<'a> Slice<RangeTo<usize>> for InStr<'a> {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        let mut ret = self.clone();
        ret.span_end = std::cmp::min(ret.span_start + range.end, ret.span_end);
        ret
    }
}

impl<'a> Slice<RangeFrom<usize>> for InStr<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        let mut ret = self.clone();
        ret.span_start = std::cmp::min(ret.span_start + range.start, ret.span_end);
        ret
    }
}

impl<'a> Slice<RangeFull> for InStr<'a> {
    fn slice(&self, _: RangeFull) -> Self {
        self.clone()
    }
}

// TODO: Implement find substring?
// TODO: Implement find token?

impl<'a, T> ParseError<InStr<'a>> for ErrorDiagnose<'a, T>
where
    T: StdError + Default,
{
    fn from_error_kind(input: InStr<'a>, _: ErrorKind) -> Self {
        ErrorDiagnose {
            errors: vec![Span {
                src: input.src,
                file: input.file,
                start: input.span_start,
                end: input.span_end,
                inner: T::default(),
                hint: None,
            }],
        }
    }

    fn append(input: InStr<'a>, _: ErrorKind, mut other: Self) -> Self {
        other.errors.push(Span {
            src: input.src,
            file: input.file,
            start: input.span_start,
            end: input.span_end,
            inner: T::default(),
            hint: None,
        });

        other
    }
}
