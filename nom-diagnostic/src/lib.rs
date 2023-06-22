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

/// Intermediate result type. Similar to [`nom::IResult`], but defined over [`InStr`].
pub type IResult<'a, T> = Result<(InStr<'a>, T), nom::Err<NomError<InStr<'a>>>>;

/// Final result type. If the parsing was not successful, we have an [`ErrorDiagnose`] which we want to pass
/// up in the chain.
pub type ParseResult<'a, T, E> = Result<(InStr<'a>, T), nom::Err<ErrorDiagnose<'a, E>>>;

pub fn diagnose<'a, P, S, Po, So, E>(
    mut parser: P,
    mut span_parser: S,
    error: E,
) -> impl FnOnce(InStr<'a>) -> ParseResult<'a, Po, E>
where
    P: Parser<InStr<'a>, Po, NomError<InStr<'a>>>,
    S: Parser<InStr<'a>, So, NomError<InStr<'a>>>,
    E: StdError + Default + Clone,
{
    move |input: InStr<'a>| match parser.parse(input.clone()) {
        Ok(output) => Ok(output),
        Err(nom::Err::Incomplete(incomplete)) => Err(nom::Err::Incomplete(incomplete)),
        Err(nom::Err::Error(err)) | Err(nom::Err::Failure(err)) => {
            dbg!(err);

            let end = span_parser
                .parse(input.clone())
                // TODO: Do something smarter than panicking
                .expect("Span parser returned an error, this is a bug")
                .0;

            Err(nom::Err::Failure(ErrorDiagnose {
                src: input.src,
                file: input.file,
                errors: vec![ErrorSpan {
                    start: input.span_start,
                    end: end.span_start,
                    error,
                    hint: None,
                }],
            }))
        }
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

    /// Finalize the input processing
    ///
    /// This function checks that there is no more
    pub fn finalize<T>(self, error: T) -> Result<(), ErrorDiagnose<'a, T>>
    where
        T: StdError + Default,
    {
        if self.span_start == self.span_end {
            Ok(())
        } else {
            Err(ErrorDiagnose {
                src: self.src,
                file: self.file,
                errors: vec![ErrorSpan {
                    start: self.span_start,
                    end: self.span_end,
                    error,
                    hint: None,
                }],
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorDiagnose<'a, T>
where
    T: StdError + Default,
{
    src: &'a str,
    file: Option<&'a str>,
    errors: Vec<ErrorSpan<'a, T>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorSpan<'a, T>
where
    T: StdError + Default,
{
    start: usize,
    end: usize,
    error: T,
    hint: Option<&'a str>,
}
impl<'a, T> ErrorDiagnose<'a, T>
where
    T: StdError + Default + Clone,
{
    pub fn display(&self) {
        let mut files = SimpleFiles::new();
        let file = files.add(self.file.unwrap_or(""), self.src);

        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();

        for error in self.errors.iter() {
            let label = Label::primary(file, error.start..error.end);
            let label = if let Some(hint) = error.hint {
                label.with_message(hint)
            } else {
                label
            };

            let diagnostic = Diagnostic::error()
                .with_message(error.error.to_string())
                .with_labels(vec![label]);

            term::emit(&mut writer.lock(), &config, &files, &diagnostic).unwrap();
        }
    }
}
