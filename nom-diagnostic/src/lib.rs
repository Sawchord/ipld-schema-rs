#![allow(dead_code)]

mod traits;
use nom::error::Error as NomError;
use std::error::Error as StdError;

/// Intermediate result type. Similar to [`nom::IResult`], but defined over [`InstrumentedStr`].
pub type IResult<'a, T> = Result<(InstrumentedStr<'a>, T), NomError<InstrumentedStr<'a>>>;

/// Final result type. If the parsing was not successful, we have an [`ErrorDiagnose`] which we want to pass
/// up in the chain.
pub type ParseResult<'a, T, E> = Result<(InstrumentedStr<'a>, T), nom::Err<ErrorDiagnose<E>>>;

// TODO: Diagnose function

#[derive(Debug, Clone)]
pub struct InstrumentedStr<'a> {
    src: &'a str,
    file: Option<&'a str>,
    span_start: usize,
    span_end: usize,
}

impl<'a> InstrumentedStr<'a> {
    /// Create a new [`InstrumentedStr`] from a [`&str`]
    pub fn new(input: &'a str) -> Self {
        Self {
            src: input,
            file: None,
            span_start: 0,
            span_end: input.len(),
        }
    }

    /// Create a new [`InstrumentedStr`] but provide a filename as well
    pub fn new_with_filename(input: &'a str, filename: &'a str) -> Self {
        Self {
            src: input,
            file: Some(filename),
            span_start: 0,
            span_end: input.len(),
        }
    }

    /// Access the [`&str`] that this [`InstrumentedStr`] is pointing to
    pub fn inner(&self) -> &'a str {
        &self.src[self.span_start..self.span_end]
    }

    // Finalize the input processing
    //pub fn finalize(self)
}

pub struct ErrorDiagnose<T>
where
    T: StdError,
{
    span_start: usize,
    span_end: usize,
    error: T,
}

// TODO: Implement Error Display
