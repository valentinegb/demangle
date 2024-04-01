use clap::{CommandFactory, Parser, ValueEnum};
use symbolic::common::{Language, Name, NameMangling};
use symbolic::demangle::{Demangle, DemangleOptions};

/// Demangles a symbol of a supported language (see <LANGUAGE>)
///
/// Powered by symbolic and clap.
#[derive(Parser)]
#[command(version)]
struct DemangleArgs {
    /// Language of the symbol to demangle
    #[arg(value_enum)]
    language: SupportedLanguage,
    /// Symbol to demangle
    symbol: String,
}

#[repr(u32)]
#[derive(ValueEnum, Clone)]
#[value(rename_all = "PascalCase")]
enum SupportedLanguage {
    #[value(name = "C++", alias = "c++", alias = "Cpp", alias = "cpp")]
    Cpp = 2,
    #[value(alias = "objc")]
    ObjC = 5,
    #[value(alias = "rust", alias = "rs")]
    Rust = 7,
    #[value(alias = "swift")]
    Swift = 8,
}

impl Into<Language> for SupportedLanguage {
    fn into(self) -> Language {
        Language::from_u32(self as u32)
    }
}

fn main() {
    let DemangleArgs { language, symbol } = DemangleArgs::parse();
    let mut command = DemangleArgs::command();
    let name = Name::new(symbol, NameMangling::Mangled, language.into());

    println!(
        "{}",
        name.demangle(DemangleOptions::complete())
            .unwrap_or_else(|| {
                command
                    .error(
                        clap::error::ErrorKind::ValueValidation,
                        "Demangling of the symbol failed.",
                    )
                    .exit()
            })
    );
}
