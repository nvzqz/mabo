use std::ops::Range;

use lsp_types::{self as lsp, Diagnostic, Url};
use stef_compiler::validate;
use stef_parser::{
    error::{
        ParseAliasCause, ParseAttributeCause, ParseCommentError, ParseConstCause,
        ParseDefinitionError, ParseEnumCause, ParseFieldsCause, ParseFieldsError,
        ParseGenericsError, ParseIdError, ParseImportCause, ParseLiteralCause, ParseModuleCause,
        ParseSchemaCause, ParseSchemaError, ParseStructCause, ParseTypeCause, ParseTypeError,
    },
    Schema,
};

use crate::utf16;

pub fn compile(file: Url, schema: &str) -> std::result::Result<Schema<'_>, Diagnostic> {
    let parsed = stef_parser::Schema::parse(schema, None)
        .map_err(|e| parse_schema_diagnostic(schema, &e))?;

    stef_compiler::validate_schema(&parsed)
        .map_err(|e| validate_schema_diagnostic(file, schema, e))?;

    Ok(parsed)
}

fn parse_schema_diagnostic(schema: &str, e: &ParseSchemaError) -> Diagnostic {
    match &e.cause {
        ParseSchemaCause::Parser(_) => {
            Diagnostic::new_simple(get_range(schema, 0..schema.len()), e.to_string())
        }
        ParseSchemaCause::Definition(e) => parse_definition_diagnostic(schema, e),
    }
}

fn parse_definition_diagnostic(schema: &str, e: &ParseDefinitionError) -> Diagnostic {
    match e {
        ParseDefinitionError::Parser(_) => {
            Diagnostic::new_simple(get_range(schema, 0..schema.len()), e.to_string())
        }
        ParseDefinitionError::Comment(e) => parse_comment_diagnostic(schema, e),
        ParseDefinitionError::Attribute(e) => match &e.cause {
            ParseAttributeCause::Parser(_) => {
                Diagnostic::new_simple(get_range(schema, 0..schema.len()), e.to_string())
            }
            ParseAttributeCause::Literal(e) => match &e.cause {
                ParseLiteralCause::Parser(_) => {
                    Diagnostic::new_simple(get_range(schema, 0..schema.len()), e.to_string())
                }
                ParseLiteralCause::FoundReference { at }
                | ParseLiteralCause::InvalidInt { at }
                | ParseLiteralCause::ParseInt { at, .. } => {
                    Diagnostic::new_simple(get_range(schema, *at..*at), e.cause.to_string())
                }
            },
        },
        ParseDefinitionError::Module(e) => match &e.cause {
            ParseModuleCause::Parser(_) => {
                Diagnostic::new_simple(get_range(schema, 0..schema.len()), e.to_string())
            }
            ParseModuleCause::InvalidName { at } => {
                Diagnostic::new_simple(get_range(schema, *at..*at), e.cause.to_string())
            }
            ParseModuleCause::Definition(e) => parse_definition_diagnostic(schema, e),
        },
        ParseDefinitionError::Struct(e) => match &e.cause {
            ParseStructCause::Parser(_) => {
                Diagnostic::new_simple(get_range(schema, 0..schema.len()), e.to_string())
            }
            ParseStructCause::InvalidName { at } => {
                Diagnostic::new_simple(get_range(schema, *at..*at), e.cause.to_string())
            }
            ParseStructCause::Generics(e) => parse_generics_diagnostic(schema, e),
            ParseStructCause::Fields(e) => parse_fields_diagnostic(schema, e),
        },
        ParseDefinitionError::Enum(e) => match &e.cause {
            ParseEnumCause::Parser(_) => {
                Diagnostic::new_simple(get_range(schema, 0..schema.len()), e.to_string())
            }
            ParseEnumCause::InvalidName { at } | ParseEnumCause::InvalidVariantName { at } => {
                Diagnostic::new_simple(get_range(schema, *at..*at), e.cause.to_string())
            }
            ParseEnumCause::Generics(e) => parse_generics_diagnostic(schema, e),
            ParseEnumCause::Field(e) => parse_fields_diagnostic(schema, e),
            ParseEnumCause::Comment(e) => parse_comment_diagnostic(schema, e),
            ParseEnumCause::Id(e) => parse_id_diagnostic(schema, e),
        },
        ParseDefinitionError::Const(e) => match &e.cause {
            ParseConstCause::Parser(_) => {
                Diagnostic::new_simple(get_range(schema, 0..schema.len()), e.to_string())
            }
            ParseConstCause::UnexpectedChar { at, .. } | ParseConstCause::InvalidName { at } => {
                Diagnostic::new_simple(get_range(schema, *at..*at), e.cause.to_string())
            }
            ParseConstCause::Type(e) => parse_type_diagnostic(schema, e),
            ParseConstCause::Literal(e) => match &e.cause {
                ParseLiteralCause::Parser(_) => {
                    Diagnostic::new_simple(get_range(schema, 0..schema.len()), e.to_string())
                }
                ParseLiteralCause::FoundReference { at }
                | ParseLiteralCause::InvalidInt { at }
                | ParseLiteralCause::ParseInt { at, .. } => {
                    Diagnostic::new_simple(get_range(schema, *at..*at), e.cause.to_string())
                }
            },
        },
        ParseDefinitionError::Alias(e) => match &e.cause {
            ParseAliasCause::Parser(_) => {
                Diagnostic::new_simple(get_range(schema, 0..schema.len()), e.to_string())
            }
            ParseAliasCause::InvalidName { at } => {
                Diagnostic::new_simple(get_range(schema, *at..*at), e.cause.to_string())
            }
            ParseAliasCause::Generics(e) => parse_generics_diagnostic(schema, e),
            ParseAliasCause::Type(e) => parse_type_diagnostic(schema, e),
        },
        ParseDefinitionError::Import(e) => parse_import_cause_diagnostic(schema, &e.cause),
    }
}

fn parse_type_diagnostic(schema: &str, e: &ParseTypeError) -> Diagnostic {
    match &e.cause {
        ParseTypeCause::Parser(_) => {
            Diagnostic::new_simple(get_range(schema, 0..schema.len()), e.to_string())
        }
        ParseTypeCause::Type(e) => parse_type_diagnostic(schema, e),
        ParseTypeCause::Segment(c) => parse_import_cause_diagnostic(schema, c),
    }
}

fn parse_comment_diagnostic(schema: &str, e: &ParseCommentError) -> Diagnostic {
    Diagnostic::new_simple(get_range(schema, e.at.clone()), e.to_string())
}

fn parse_import_cause_diagnostic(schema: &str, c: &ParseImportCause) -> Diagnostic {
    match c {
        ParseImportCause::Parser(_) => {
            Diagnostic::new_simple(get_range(schema, 0..schema.len()), c.to_string())
        }
        ParseImportCause::InvalidSegmentName { at } => {
            Diagnostic::new_simple(get_range(schema, *at..*at), c.to_string())
        }
        ParseImportCause::StructName(c) => match c {
            ParseStructCause::Parser(_) => {
                Diagnostic::new_simple(get_range(schema, 0..schema.len()), c.to_string())
            }
            ParseStructCause::InvalidName { at } => {
                Diagnostic::new_simple(get_range(schema, *at..*at), c.to_string())
            }
            ParseStructCause::Generics(e) => parse_generics_diagnostic(schema, e),
            ParseStructCause::Fields(e) => parse_fields_diagnostic(schema, e),
        },
        ParseImportCause::EnumName(c) => match c {
            ParseEnumCause::Parser(_) => {
                Diagnostic::new_simple(get_range(schema, 0..schema.len()), c.to_string())
            }
            ParseEnumCause::InvalidName { at } | ParseEnumCause::InvalidVariantName { at } => {
                Diagnostic::new_simple(get_range(schema, *at..*at), c.to_string())
            }
            ParseEnumCause::Generics(e) => parse_generics_diagnostic(schema, e),
            ParseEnumCause::Field(e) => parse_fields_diagnostic(schema, e),
            ParseEnumCause::Comment(e) => parse_comment_diagnostic(schema, e),
            ParseEnumCause::Id(e) => parse_id_diagnostic(schema, e),
        },
    }
}

fn parse_generics_diagnostic(schema: &str, e: &ParseGenericsError) -> Diagnostic {
    match &e.cause {
        stef_parser::error::ParseGenericsCause::Parser(_) => {
            Diagnostic::new_simple(get_range(schema, 0..schema.len()), e.to_string())
        }
        stef_parser::error::ParseGenericsCause::InvalidName { at } => {
            Diagnostic::new_simple(get_range(schema, *at..*at), e.cause.to_string())
        }
    }
}

fn parse_fields_diagnostic(schema: &str, e: &ParseFieldsError) -> Diagnostic {
    match &e.cause {
        ParseFieldsCause::Parser(_) => {
            Diagnostic::new_simple(get_range(schema, 0..schema.len()), e.to_string())
        }
        ParseFieldsCause::InvalidName { at } => {
            Diagnostic::new_simple(get_range(schema, *at..*at), e.cause.to_string())
        }
        ParseFieldsCause::Type(e) => parse_type_diagnostic(schema, e),
        ParseFieldsCause::Id(e) => parse_id_diagnostic(schema, e),
        ParseFieldsCause::Comment(e) => parse_comment_diagnostic(schema, e),
    }
}

fn parse_id_diagnostic(schema: &str, e: &ParseIdError) -> Diagnostic {
    Diagnostic::new_simple(get_range(schema, e.at.clone()), e.to_string())
}

fn validate_schema_diagnostic(file: Url, schema: &str, e: validate::Error) -> Diagnostic {
    use validate::{DuplicateFieldId, DuplicateId, DuplicateName, Error, InvalidGenericType};

    let (message, first, second) = match e {
        Error::DuplicateId(e) => match e {
            DuplicateId::EnumVariant(e) => (e.to_string(), e.first, e.second),
            DuplicateId::Field(e) => match e {
                DuplicateFieldId::Named(e) => (e.to_string(), e.first, e.second),
                DuplicateFieldId::Unnamed(e) => (e.to_string(), e.first, e.second),
            },
        },
        Error::DuplicateName(e) => match e {
            DuplicateName::EnumVariant(e) => (e.to_string(), e.first, e.second),
            DuplicateName::Field(e) => (e.to_string(), e.first, e.second),
            DuplicateName::InModule(e) => (e.to_string(), e.first, e.second),
        },
        Error::InvalidGeneric(e) => match e {
            InvalidGenericType::Duplicate(e) => (e.to_string(), e.first, e.second),
            InvalidGenericType::Unused(e) => {
                let message = e.to_string();
                return Diagnostic::new_simple(get_range(schema, e.declared), message);
            }
        },
        Error::TupleSize(e) => {
            let message = e.to_string();
            return Diagnostic::new_simple(get_range(schema, e.declared), message);
        }
    };

    diagnostic_with_related(
        get_range(schema, second),
        message,
        vec![lsp::DiagnosticRelatedInformation {
            location: lsp::Location::new(file, get_range(schema, first)),
            message: "first used here".to_owned(),
        }],
    )
}

fn diagnostic_with_related(
    range: lsp::Range,
    message: String,
    related: Vec<lsp::DiagnosticRelatedInformation>,
) -> Diagnostic {
    Diagnostic::new(range, None, None, None, message, Some(related), None)
}

#[allow(clippy::cast_possible_truncation)]
fn get_range(schema: &str, location: Range<usize>) -> lsp::Range {
    let start_line = schema[..location.start].lines().count().saturating_sub(1);
    let start_char = schema[..location.start]
        .lines()
        .last()
        .map_or(0, utf16::len);

    let end_line = schema[..location.end].lines().count().saturating_sub(1);
    let end_char = schema[..location.end].lines().last().map_or(0, utf16::len);

    lsp::Range::new(
        lsp::Position::new(start_line as u32, start_char as u32),
        lsp::Position::new(end_line as u32, end_char as u32),
    )
}
