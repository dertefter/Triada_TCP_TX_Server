use std::net::Ipv4Addr;
use errors_and_something_else::ErrorType;
use server_handler::{CommandId, DeviceAddress, ServerHandler};
use tcp_server::{TcpServer, TcpServerConfig};

mod tcp_server;
mod server_handler;
mod errors_and_something_else;

fn main() {
    let tcp_server_config = TcpServerConfig{
        ipv4addr: Ipv4Addr::new(127, 0, 0, 1),
        port: 8080
    };

    let mut my_id_list: Vec<CommandId> = Vec::new();

    let id1 = CommandId::from_string("00".to_string(), test_function1);
    let id2 = CommandId::from_string("01".to_string(), test_function2);

    my_id_list.push(id1);
    my_id_list.push(id2);

    let test_ad = DeviceAddress::from_string("00".to_string(), my_id_list);

    let mut my_ad_list: Vec<DeviceAddress> = Vec::new();

    my_ad_list.push(test_ad);

    let my_handler = ServerHandler{
        ad_list: my_ad_list
    };

    let handle_func = my_handler.get_func();

    let tcp_server = TcpServer::new(Box::new(handle_func), tcp_server_config);

    tcp_server.start_server();
}

fn test_function1() -> Result<Vec<u8>, ErrorType> {
    Ok(Vec::from("0000000000000000000000".as_bytes()))
}

fn test_function2() -> Result<Vec<u8>, ErrorType> {
    Ok(Vec::from("111111111111111111111111111".as_bytes()))
}
