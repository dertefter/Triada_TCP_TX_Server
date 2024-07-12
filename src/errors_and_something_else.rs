use std::convert::TryFrom;

pub enum ErrorType{
    BitesCountError,
    WrongADError,
    WrongIDError,
    CommandError,
    AnotherError
}

impl ErrorType{
    pub fn bytes(self) -> [u8; 2] {
        match self {
            ErrorType::BitesCountError => { <[u8; 2]>::try_from("00".as_bytes()).unwrap() }
            ErrorType::WrongADError => {<[u8; 2]>::try_from("01".as_bytes()).unwrap()}
            ErrorType::WrongIDError => {<[u8; 2]>::try_from("02".as_bytes()).unwrap()}
            ErrorType::CommandError => {<[u8; 2]>::try_from("03".as_bytes()).unwrap()}
            ErrorType::AnotherError => {<[u8; 2]>::try_from("04".as_bytes()).unwrap()}
        }
    }

    pub fn string(self) -> String {
        match self {
            ErrorType::BitesCountError => {"00".to_string()}
            ErrorType::WrongADError => {"01".to_string()}
            ErrorType::WrongIDError => {"02".to_string()}
            ErrorType::CommandError => {"03".to_string()}
            ErrorType::AnotherError => {"04".to_string()}
        }
    }
}