use std::convert::{TryFrom, TryInto};
use std::io::Read;
use crate::errors_and_something_else::{ErrorType};

#[derive(Clone)]
pub struct ServerHandler{
    pub ad_list: Vec<DeviceAddress>,
}

impl ServerHandler{
    fn get_response(&self, message: Message) -> Result<Message, ErrorType>{
        for ad in &self.ad_list{
            if ad.ad_name == message.ad{
                for id in ad.id_list.clone(){
                    if id.id_name == message.id{
                        let action_data = match (id.action)() {
                            Err(..) => {return Err(ErrorType::CommandError)},
                            Ok(data) => data
                        };
                        return Ok(Message{
                            prefix: message.prefix,
                            ad: message.ad,
                            id: message.id,
                            data: action_data,
                        })
                    }
                }
                return Err(ErrorType::WrongIDError)
            }
        }
        return Err(ErrorType::WrongADError)
    }

    pub fn handle(&self, received_data: Vec<u8>) -> Result<Vec<u8>, ErrorType> {
        let received_message = Self::parse_message(received_data);
        match received_message {
            Ok(message) => {
                let message_for_send = self.get_response(message.clone());
                match message_for_send {
                    Ok(message) => {
                        let bytes = Self::generate_bytes_from_message(message);
                        println!("{:?}", bytes);
                        Ok(bytes)
                    }
                    Err(error_type) => {
                        let bytes = Self::generate_bytes_from_error(error_type, None);
                        println!("{:?}", bytes);
                        Ok(bytes)
                    }
                }
            }
            Err(error_type) => {
                Err(error_type)
            }
        }
    }

    pub fn get_func(self) -> Box<dyn Fn(Vec<u8>) -> Result<Vec<u8>, ErrorType> + Send + 'static> {
        Box::new(move |data: Vec<u8>| self.handle(data))
    }

    fn generate_bytes_from_message(message: Message) -> Vec<u8>{
        let mut out: Vec<u8> = Vec::new();
        out.extend(&message.prefix);
        out.extend(&message.ad);
        out.extend(&message.id);
        out.extend(&message.data);
        out.extend("\n".as_bytes().iter());
        out
    }

    fn generate_bytes_from_error(error_type: ErrorType, message: Option<Message>) -> Vec<u8> {
        if message.is_none(){
            return Vec::new()
        } else{
            let message = message.unwrap();
            let error_sign = "FE".as_bytes();
            let error_about = error_type.bytes();
            let mut out: Vec<u8> = Vec::new();
            out.extend(message.prefix);
            out.extend(message.ad);
            out.extend(error_sign);
            out.extend(error_about);
            out.extend("\n".as_bytes());
            out
        }
    }


    fn parse_message(bytes_data: Vec<u8>) -> Result<Message, ErrorType>{
        if bytes_data.len() < 6 {
            return Err(ErrorType::BitesCountError)
        } else if !bytes_data.is_ascii(){
            return Err(ErrorType::AnotherError)
        }else{
            let prefix: [u8; 2] = bytes_data.get(0..2).unwrap().try_into().unwrap();
            let this_ad: [u8; 2] =  bytes_data.get(2..4).unwrap().try_into().unwrap();
            let this_id: [u8; 2] = bytes_data.get(4..6).unwrap().try_into().unwrap();
            let this_data: Vec<u8> = Vec::from(bytes_data.get(6..).unwrap());
            Ok(
                Message{
                    prefix,
                    id: this_id,
                    ad: this_ad,
                    data: this_data
                }
            )
        }
    }

}

#[derive(Clone)]
struct Message{
    prefix: [u8; 2],
    ad: [u8; 2],
    id: [u8; 2],
    data: Vec<u8>
}



#[derive(Clone)]
pub struct DeviceAddress {
    ad_name: [u8; 2],
    id_list: Vec<CommandId>
}

impl DeviceAddress {
    pub fn from_string(s: String, id_list: Vec<CommandId>) -> DeviceAddress{
        if !s.is_ascii() {
            panic!("Not ASCII!")
        } else if s.len() != 2{
            panic!("You need place two characters!")
        }
        DeviceAddress{
            ad_name: <[u8; 2]>::try_from(s.as_bytes()).expect("Can not create name from string"),
            id_list,
        }
    }

    pub fn from_bytes(b: [u8; 2], id_list: Vec<CommandId>) -> DeviceAddress{
        DeviceAddress{
            ad_name: b,
            id_list,
        }
    }
}

#[derive(Clone)]
pub struct CommandId {
    pub(crate) id_name: [u8; 2],
    pub(crate) action: fn() -> Result<Vec<u8>, ErrorType>
}

impl CommandId {
    pub fn from_string(s: String, action: fn() -> Result<Vec<u8>, ErrorType>) -> CommandId{
        if !s.is_ascii() {
            panic!("Not ASCII!")
        } else if s.len() != 2{
            panic!("You need place two characters!")
        }
        CommandId{
            id_name: <[u8; 2]>::try_from(s.as_bytes()).expect("Can not create name from string"),
            action,
        }
    }

    pub fn from_bytes(b: [u8; 2], action: fn() -> Result<Vec<u8>, ErrorType>) -> CommandId{
        CommandId{
            id_name: b,
            action,
        }
    }
}