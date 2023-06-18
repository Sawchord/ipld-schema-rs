mod traits;
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use nom::{error::Error as NomError, Parser};
use std::error::Error as StdError;

/// Intermediate result type. Similar to [`nom::IResult`], but defined over [`InstrumentedStr`].
pub type IResult<'a, T> = Result<(InstrumentedStr<'a>, T), NomError<InstrumentedStr<'a>>>;

/// Final result type. If the parsing was not successful, we have an [`ErrorDiagnose`] which we want to pass
/// up in the chain.
pub type ParseResult<'a, T, E> = Result<(InstrumentedStr<'a>, T), nom::Err<ErrorDiagnose<'a, E>>>;

pub fn diagnose<'a, P, S, Po, So, E>(
    mut parser: P,
    mut span_parser: S,
    error: E,
) -> impl FnOnce(InstrumentedStr<'a>) -> ParseResult<'a, Po, E>
where
    P: Parser<InstrumentedStr<'a>, Po, NomError<InstrumentedStr<'a>>>,
    S: Parser<InstrumentedStr<'a>, So, NomError<InstrumentedStr<'a>>>,
    E: StdError + Clone,
{
    move |input: InstrumentedStr<'a>| match parser.parse(input.clone()) {
        Ok(output) => Ok(output),
        Err(nom::Err::Incomplete(incomplete)) => Err(nom::Err::Incomplete(incomplete)),
        Err(nom::Err::Error(_)) | Err(nom::Err::Failure(_)) => {
            let end = span_parser
                .parse(input.clone())
                // TODO: Do something smarter than panicking
                .expect("Span parser returned an error, this is a bug")
                .0;

            Err(nom::Err::Failure(ErrorDiagnose {
                src: input.src,
                file: input.file,
                span_start: input.span_start,
                span_end: end.span_start,
                error,
            }))
        }
    }
}

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

    /// Finalize the input processing
    ///
    /// This function checks that there is no more
    pub fn finalize<T>(self, error: T) -> Result<(), ErrorDiagnose<'a, T>>
    where
        T: StdError,
    {
        if self.span_start == self.span_end {
            Ok(())
        } else {
            Err(ErrorDiagnose {
                src: self.src,
                file: self.file,
                span_start: self.span_start,
                span_end: self.span_end,
                error,
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorDiagnose<'a, T>
where
    T: StdError,
{
    src: &'a str,
    file: Option<&'a str>,
    span_start: usize,
    span_end: usize,
    error: T,
}

impl<'a, T> ErrorDiagnose<'a, T>
where
    T: StdError,
{
    pub fn display(&self) {
        let mut files = SimpleFiles::new();
        let file = files.add(self.file.unwrap_or(""), self.src);

        let diagnostic = Diagnostic::error()
            .with_message(self.error.to_string())
            .with_labels(vec![Label::primary(file, self.span_start..self.span_end)]);

        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();
        term::emit(&mut writer.lock(), &config, &files, &diagnostic).unwrap();
    }
}
