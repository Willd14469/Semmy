use crate::semantic_version::{SemanticVersion, SemanticVersionBuilder};
use regex::Regex;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

struct ParseError(String);

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.0)
    }
}

impl Error for ParseError {}

/// https://regexper.com/#%5E%28%5Cd%29%2B%5C.%28%5Cd%29%2B%5C.%28%5Cd%29%2B%28%3F%3A-%28%5Cw%29%3F%28%5C.%28%5Cw%29%29*%29%28%3F%3A%5C%2B%28%5Cd%29%2B%29%3F%24
pub fn parse(input: &str) -> anyhow::Result<SemanticVersion> {
    let re =
        Regex::new(r"^(?P<major>[[:digit:]])+\.(?P<minor>[[:digit:]])+\.(?P<patch>[[:digit:]])+(?:-(?P<pre_release>alpha|beta))?(?:(\+|--)(?P<build>[[:digit:]]+))?$")
            .unwrap();

    let matches = re
        .captures(input)
        .ok_or(ParseError("Unable to parse the version".to_string()))?;

    let mut semvar_builder = SemanticVersionBuilder::default();

    let major: u16 = matches
        .name("major")
        .ok_or(ParseError("unable to get major version".to_string()))?
        .as_str()
        .parse()?;

    let minor: u16 = matches
        .name("minor")
        .ok_or(ParseError("unable to get minor version".to_string()))?
        .as_str()
        .parse()?;

    let patch: u16 = matches
        .name("patch")
        .ok_or(ParseError("unable to get patch version".to_string()))?
        .as_str()
        .parse()?;

    semvar_builder.major(major).minor(minor).patch(patch);

    let build_iter = matches.name("build");
    if let Some(build_iter_match) = build_iter {
        semvar_builder.build_iteration(build_iter_match.as_str().parse::<u16>()?);
    };

    let pre_release_flag = matches.name("pre_release");
    if let Some(pre_release_flag_match) = pre_release_flag {
        match pre_release_flag_match.as_str() {
            "alpha" => {
                semvar_builder.as_alpha();
            }
            "beta" => {
                semvar_builder.as_beta();
            }
            _ => return Err(ParseError("unknown pre-release flag".to_string()).into()),
        }
    }

    Ok(semvar_builder.build()?)
}
