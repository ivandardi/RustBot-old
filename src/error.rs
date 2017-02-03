use serde_json::Error as SerdeError;
use std::io::Error as IoError;
use log::SetLoggerError;
use serenity::Error as SerenityError;

error_chain! {
    foreign_links {
        SerdeJson(SerdeError);
        Io(IoError);
        SetLogger(SetLoggerError);
        Serenity(SerenityError);
    }
}