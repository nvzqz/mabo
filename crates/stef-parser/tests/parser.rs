use std::{
    fmt::{self, Display},
    fs,
};

use insta::{assert_snapshot, glob};
use miette::{Diagnostic, MietteHandler, MietteHandlerOpts, ReportHandler};
use stef_parser::Schema;

#[test]
fn parse_schema() {
    glob!("inputs/*.stef", |path| {
        let input = fs::read_to_string(path).unwrap();
        let value = Schema::parse(input.as_str()).unwrap();

        assert_snapshot!(
            "parse",
            format!("{value:#?}"),
            stringify!(Schema::parse(input.as_str()).unwrap())
        );
        assert_snapshot!(
            "print",
            value.to_string(),
            stringify!(Schema::parse(input.as_str()).unwrap())
        );
    });
}

#[test]
fn parse_invalid_schema() {
    struct Wrapper<'a>(&'a MietteHandler, &'a dyn Diagnostic);

    impl<'a> Display for Wrapper<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.debug(self.1, f)
        }
    }

    let handler = MietteHandlerOpts::new()
        .color(false)
        .terminal_links(false)
        .width(120)
        .force_graphical(true)
        .build();

    glob!("inputs/invalid/*.stef", |path| {
        let input = fs::read_to_string(path).unwrap();
        let value = Schema::parse(input.as_str()).unwrap_err();

        insta::assert_snapshot!(
            "error",
            Wrapper(&handler, &*value).to_string(),
            stringify!(Schema::parse(input.as_str()).unwrap_err())
        );
    });
}
