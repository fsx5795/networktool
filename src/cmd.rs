use local_ip_address;
#[tauri::command]
pub fn get_ipaddr() -> String {
    let interfaces = local_ip_address::list_afinet_netifas().unwrap();
    let mut s = interfaces.iter().filter(|(_, ip)| ip.to_string().len() > 10 && ip.to_string().len() <= 16).map(|(_, ip)| ip.to_string() + ",").collect::<String>();
    s.pop();
    s
}
#[tauri::command]
pub fn open_socket(addr: String, port: String, multaddr: String) -> () {
    let socket = std::net::UdpSocket::bind(addr).unwrap();
    socket.set_broadcast(true).unwrap();
    socket.set_multicast_loop_v4(false).unwrap();
    if !multaddr.is_empty() {
        let multaddr = multaddr.split('.').map(|s| s.parse().unwrap()).collect::<Vec<u8>>();
        socket.join_multicast_v4(&std::net::Ipv4Addr::new(multaddr[0], multaddr[1], multaddr[2], multaddr[3]), &std::net::Ipv4Addr::new(0, 0, 0, 0)).unwrap();
    }
}