use crate::env::get_bool_with_default;
use crate::semantic_version::sem_ver_traits::{
    ColouredTextRenderer, SemVerVisualizer, TextRenderer,
};
use colored::Colorize;
use std::fmt::{Display, Formatter};

pub const ALPHA_IDENTIFIER: &str = "alpha";
pub const BETA_IDENTIFIER: &str = "beta";

#[derive(PartialOrd, PartialEq)]
enum Levels {
    Major = 1,
    Minor = 2,
    Patch = 3,
    BuildIter = 4,
}
#[derive(Clone, Debug, PartialOrd, PartialEq)]
enum PreReleaseFlag {
    alpha,
    beta,
}

impl Display for PreReleaseFlag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PreReleaseFlag::alpha => {
                write!(f, "alpha")
            }
            PreReleaseFlag::beta => {
                write!(f, "beta")
            }
        }
    }
}

#[derive(Builder, Debug)]
pub struct SemanticVersion {
    #[builder(default = "0")]
    major: u16,
    #[builder(default = "1")]
    minor: u16,
    #[builder(default = "0")]
    patch: u16,

    #[builder(default = "0")]
    build_iteration: u16,

    #[builder(default = "None")]
    pre_release_flag: Option<PreReleaseFlag>,
}

impl ToString for SemanticVersion {
    fn to_string(&self) -> String {
        TextRenderer::construct_str(self)
    }
}

impl Default for SemanticVersion {
    fn default() -> Self {
        Self {
            major: 0,
            minor: 1,
            patch: 0,
            build_iteration: 0,
            pre_release_flag: None,
        }
    }
}

impl SemanticVersion {
    fn log_message<T: Display>(&self, prefix: &str, field_name: &str, old_value: T, new_value: T) {
        let coloured = get_bool_with_default("COLOURED", false);
        let verbose = get_bool_with_default("VERBOSE", false);

        if !verbose {
            return;
        }

        let (field_str, from_str, to_str) = match coloured {
            true => (
                field_name.magenta().to_string(),
                format!("{}", old_value.to_string().cyan()),
                format!("{}", new_value.to_string().cyan()),
            ),
            false => (
                field_name.to_string(),
                old_value.to_string(),
                new_value.to_string(),
            ),
        };

        println!("🔲 {prefix} {field_str} from {from_str} => {to_str}")
    }
    pub fn coloured_to_string(&self) -> String {
        ColouredTextRenderer::construct_str(self)
    }
    pub fn inc_major(&mut self) {
        self.log_message("Incremented", "major", self.major, self.major + 1);
        self.major += 1;
        self.reset_fields(Levels::Major)
    }

    pub fn inc_minor(&mut self) {
        self.log_message("Incremented", "minor", self.minor, self.minor + 1);

        self.minor += 1;
        self.reset_fields(Levels::Minor)
    }

    pub fn inc_patch(&mut self) {
        self.log_message("Incremented", "patch", self.patch, self.patch + 1);

        self.patch += 1;
        self.reset_fields(Levels::Patch)
    }

    pub fn inc_build(&mut self) {
        self.log_message(
            "Incremented",
            "build",
            self.build_iteration,
            self.build_iteration + 1,
        );

        self.build_iteration += 1;
        self.reset_fields(Levels::BuildIter)
    }

    pub fn set_alpha(&mut self) {
        self.pre_release_flag = Some(PreReleaseFlag::alpha)
    }

    pub fn set_beta(&mut self) {
        self.pre_release_flag = Some(PreReleaseFlag::beta)
    }

    pub fn remove_flag(&mut self) {
        self.pre_release_flag = None
    }
}

// Resetting field functions
impl SemanticVersion {
    fn reset_fields(&mut self, level: Levels) {
        if level < Levels::Major {
            self.major = 0;
        }

        if level < Levels::Minor {
            self.minor = 0;
        }

        if level < Levels::Patch {
            self.patch = 0;
        }

        if level < Levels::BuildIter {
            self.build_iteration = 0;
            self.pre_release_flag = None
        }
    }
}

impl SemanticVersionBuilder {
    pub fn as_alpha(&mut self) -> &mut Self {
        self.pre_release_flag = Some(Some(PreReleaseFlag::alpha));

        self
    }

    pub fn as_beta(&mut self) -> &mut Self {
        self.pre_release_flag = Some(Some(PreReleaseFlag::beta));

        self
    }
}

mod sem_ver_traits {
    use crate::semantic_version::SemanticVersion;
    use colored::Colorize;

    pub trait SemVerVisualizer: SemVerPrivateVisInterface {
        fn construct_str(sem_ver: &SemanticVersion) -> String {
            let pre_release_string =
                Self::edit_pre_release_hook(Self::construct_pre_release_str(sem_ver));

            let build_str = Self::edit_pre_release_hook(Self::construct_build_str(sem_ver));

            let version = Self::edit_pre_release_hook(Self::construct_version_str(sem_ver));

            format! {
                "{}{}{}",version, pre_release_string,build_str
            }
        }

        fn edit_pre_release_hook(pre_release: String) -> String {
            pre_release
        }

        fn edit_build_hook(build: String) -> String {
            build
        }

        fn edit_version_hook(version: String) -> String {
            version
        }
    }

    trait SemVerPrivateVisInterface {
        fn construct_version_str(sem_ver: &SemanticVersion) -> String {
            format!(
                "{:?}.{:?}.{:?}",
                sem_ver.major, sem_ver.minor, sem_ver.patch
            )
        }

        fn construct_pre_release_str(sem_ver: &SemanticVersion) -> String {
            let pre_release_string = match &sem_ver.pre_release_flag {
                Some(flag) => {
                    format!("-{:?}", flag)
                }
                None => "".to_owned(),
            };
            pre_release_string
        }

        fn construct_build_str(sem_ver: &SemanticVersion) -> String {
            let build_str = match sem_ver.build_iteration > 0 {
                true => {
                    format! {"+{}", sem_ver.build_iteration}
                }
                false => "".to_owned(),
            };
            build_str
        }
    }

    pub struct TextRenderer;

    impl SemVerPrivateVisInterface for TextRenderer {}

    impl SemVerVisualizer for TextRenderer {}

    pub struct ColouredTextRenderer;

    impl SemVerPrivateVisInterface for ColouredTextRenderer {}

    impl SemVerVisualizer for ColouredTextRenderer {
        fn edit_pre_release_hook(pre_release: String) -> String {
            pre_release.cyan().to_string()
        }

        fn edit_build_hook(build: String) -> String {
            build.green().to_string()
        }

        fn edit_version_hook(version: String) -> String {
            version.magenta().to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_semver(semvar: &SemanticVersion, major: u16, minor: u16, patch: u16, build: u16) {
        assert_eq!(semvar.major, major);
        assert_eq!(semvar.minor, minor);
        assert_eq!(semvar.patch, patch);
        assert_eq!(semvar.build_iteration, build);
    }

    #[test]
    fn test_inc_major() {
        let mut initial = SemanticVersionBuilder::default().build().unwrap();

        initial.inc_major();

        assert_semver(&initial, 1, 0, 0, 0)
    }

    #[test]
    fn test_inc_minor() {
        let mut initial = SemanticVersionBuilder::default().build().unwrap();

        initial.inc_minor();

        assert_semver(&initial, 0, 2, 0, 0)
    }

    #[test]
    fn test_inc_patch() {
        let mut initial = SemanticVersionBuilder::default().build().unwrap();

        initial.inc_patch();

        assert_semver(&initial, 0, 1, 1, 0)
    }

    #[test]
    fn test_inc_build() {
        let mut initial = SemanticVersionBuilder::default().build().unwrap();

        initial.inc_build();

        assert_semver(&initial, 0, 1, 0, 1)
    }

    #[test]
    fn test_set_alpha() {
        let mut initial = SemanticVersionBuilder::default().build().unwrap();

        initial.set_alpha();

        assert_semver(&initial, 0, 1, 0, 0);
        assert_eq!(initial.pre_release_flag, Some(PreReleaseFlag::alpha));
    }

    #[test]
    fn test_set_beta() {
        let mut initial = SemanticVersionBuilder::default().build().unwrap();

        initial.set_beta();

        assert_semver(&initial, 0, 1, 0, 0);
        assert_eq!(initial.pre_release_flag, Some(PreReleaseFlag::beta));
    }

    #[test]
    fn test_unset_pre_release_flag() {
        let mut initial = SemanticVersionBuilder::default().build().unwrap();

        initial.remove_flag();

        assert_semver(&initial, 0, 1, 0, 0);
        assert_eq!(initial.pre_release_flag, None);
    }

    #[test]
    fn test_initial() {
        let initial = SemanticVersionBuilder::default().build().unwrap();

        assert_semver(&initial, 0, 1, 0, 0)
    }

    #[test]
    fn test_to_string() {
        let initial = SemanticVersionBuilder::default().build().unwrap();

        assert_eq!(initial.to_string(), "0.1.0")
    }

    #[test]
    fn test_full_to_string() {
        let mut initial = SemanticVersionBuilder::default()
            .patch(24)
            .build_iteration(42)
            .as_alpha()
            .build()
            .unwrap();

        initial.set_beta();

        assert_eq!(initial.to_string(), "0.1.24-beta+42")
    }

    #[test]
    fn test_remove_beta_to_string() {
        let mut initial = SemanticVersionBuilder::default()
            .patch(24)
            .build_iteration(42)
            .as_beta()
            .build()
            .unwrap();

        initial.remove_flag();

        assert_eq!(initial.to_string(), "0.1.24+42")
    }
}
