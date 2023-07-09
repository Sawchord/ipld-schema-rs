mod traits;
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use nom::{
    error::{Error as NomError, ParseError},
    InputTakeAtPosition, Parser,
};
use std::{
    collections::BTreeMap,
    error::Error as StdError,
    ops::{Deref, DerefMut},
};

/// Intermediate result type. Similar to [`nom::IResult`], but defined over [`InStr`].
pub type IResult<'a, T> = Result<(InStr<'a>, T), nom::Err<NomError<InStr<'a>>>>;

/// Final result type. If the parsing was not successful, we have an [`ErrorDiagnose`] which we want to pass
/// up in the chain.
pub type ParseResult<'a, T, E> = Result<(InStr<'a>, T), nom::Err<ErrorDiagnose<'a, E>>>;

pub fn diagnose<'a, P, S, Po, E>(
    mut parser: P,
    span_parser: S,
) -> impl FnOnce(InStr<'a>) -> ParseResult<'a, Po, E>
where
    P: Parser<InStr<'a>, Po, NomError<InStr<'a>>>,
    S: Fn(NomError<InStr<'a>>) -> Vec<Span<'a, E>>,
    E: StdError + Default + Clone,
{
    move |input: InStr<'a>| match parser.parse(input.clone()) {
        Ok(output) => Ok(output),
        Err(nom::Err::Incomplete(incomplete)) => Err(nom::Err::Incomplete(incomplete)),
        Err(nom::Err::Error(err)) => {
            let errors = span_parser(err);
            Err(nom::Err::Error(ErrorDiagnose { errors }))
        }
        Err(nom::Err::Failure(err)) => {
            let errors = span_parser(err);
            Err(nom::Err::Failure(ErrorDiagnose { errors }))
        }
    }
}

pub fn map_diagnose<'a, P, Po, M, Mo, E>(
    mut parser: P,
    map: M,
) -> impl FnOnce(InStr<'a>) -> ParseResult<'a, Mo, E>
where
    P: Parser<InStr<'a>, Po, ErrorDiagnose<'a, E>>,
    M: Fn(Po) -> Result<Mo, ErrorDiagnose<'a, E>>,
    E: StdError + Default,
{
    move |input: InStr<'a>| match parser.parse(input) {
        Ok((input, parse_output)) => match map(parse_output) {
            Ok(value) => Ok((input, value)),
            Err(err) => Err(nom::Err::Error(err)),
        },
        Err(err) => Err(err),
    }
}

#[derive(Debug, Clone)]
pub struct InStr<'a> {
    src: &'a str,
    file: Option<&'a str>,
    span_start: usize,
    span_end: usize,
}

impl<'a> InStr<'a> {
    /// Create a new [`InStr`] from a [`&str`]
    pub fn new(input: &'a str) -> Self {
        Self {
            src: input,
            file: None,
            span_start: 0,
            span_end: input.len(),
        }
    }

    /// Create a new [`InStr`] but provide a filename as well
    pub fn new_with_filename(input: &'a str, filename: &'a str) -> Self {
        Self {
            src: input,
            file: Some(filename),
            span_start: 0,
            span_end: input.len(),
        }
    }

    /// Access the [`&str`] that this [`InStr`] is pointing to
    pub fn inner(&self) -> &'a str {
        &self.src[self.span_start..self.span_end]
    }

    // TODO: Rename to error_span
    pub fn error_span<P, E>(&self, predicate: P, inner: E) -> Span<'a, E>
    where
        P: Fn(char) -> bool,
        E: StdError + Default,
    {
        let span: InStr<'a> = self
            .split_at_position_complete::<_, ()>(predicate)
            .map(|(_, prefix)| prefix)
            .unwrap_or_else(|_| self.clone());

        Span {
            src: span.src,
            file: span.file,
            start: span.span_start,
            end: span.span_end,
            inner,
            hint: None,
        }
    }

    pub fn map<F, T>(&self, f: F) -> Span<'a, T>
    where
        F: Fn(&str) -> T,
    {
        Span {
            src: self.src,
            file: self.file,
            start: self.span_start,
            end: self.span_end,
            inner: f(self.inner()),
            hint: None,
        }
    }

    /// Finalize the input processing
    ///
    /// This function checks that there is no more
    pub fn finalize<T>(self, error: T, hint: &'a str) -> Result<(), ErrorDiagnose<'a, T>>
    where
        T: StdError + Default,
    {
        if self.span_start == self.span_end {
            Ok(())
        } else {
            Err(ErrorDiagnose {
                errors: vec![Span {
                    src: self.src,
                    file: self.file,
                    start: self.span_start,
                    end: self.span_end,
                    inner: error,
                    hint: Some(hint),
                }],
            })
        }
    }
}

impl<'a> ToString for InStr<'a> {
    fn to_string(&self) -> String {
        self.inner().to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorDiagnose<'a, T>
where
    T: StdError + Default,
{
    errors: Vec<Span<'a, T>>,
}

impl<'a, E> ErrorDiagnose<'a, E>
where
    E: StdError + Default + Clone,
{
    pub fn compat<T>(result: IResult<'a, T>) -> ParseResult<'a, T, E> {
        match result {
            Ok(val) => Ok(val),
            Err(nom::Err::Incomplete(needed)) => Err(nom::Err::Incomplete(needed)),
            Err(nom::Err::Error(err)) => Err(nom::Err::Error(ErrorDiagnose::from_error_kind(
                err.input, err.code,
            ))),
            Err(nom::Err::Failure(err)) => Err(nom::Err::Failure(ErrorDiagnose::from_error_kind(
                err.input, err.code,
            ))),
        }
    }

    pub fn display(&self) {
        let mut files = SimpleFiles::new();
        let mut files_map = BTreeMap::new();

        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();
        let mut diagnostic = Diagnostic::error();

        for error in self.errors.iter() {
            let file = files_map
                .entry((error.file.unwrap_or(""), error.src))
                .or_insert_with(|| files.add(error.file.unwrap_or(""), error.src));

            let label = Label::primary(*file, error.start..error.end);
            let label = if let Some(hint) = error.hint {
                label.with_message(hint)
            } else {
                label
            };

            diagnostic = diagnostic
                .with_message(error.inner.to_string())
                .with_labels(vec![label]);
        }

        term::emit(&mut writer.lock(), &config, &files, &diagnostic).unwrap();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span<'a, T> {
    src: &'a str,
    file: Option<&'a str>,
    start: usize,
    end: usize,
    inner: T,
    hint: Option<&'a str>,
}

impl<'a, T> Span<'a, T> {
    pub fn with_hint(mut self, hint: &'a str) -> Self {
        self.hint = Some(hint);
        self
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn map<F, K>(self, f: F) -> Span<'a, K>
    where
        F: Fn(T) -> K,
    {
        Span {
            src: self.src,
            file: self.file,
            start: self.start,
            end: self.end,
            inner: f(self.inner),
            hint: self.hint,
        }
    }
}

// TODO: Figure out how to make this work
// impl<'a, T> From<Span<'a, T>> for T {
//     fn from(value: Span<'a, T>) -> Self {
//         todo!()
//     }
// }

impl<'a, T> Deref for Span<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> DerefMut for Span<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
