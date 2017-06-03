use std::io::Error as IoError;
use log::SetLoggerError;
use serenity::Error as SerenityError;
use toml::de::Error as TomlParseError;

error_chain! {
    foreign_links {
        TomlParseError(TomlParseError);
        Io(IoError);
        SetLogger(SetLoggerError);
        Serenity(SerenityError);
    }
}
