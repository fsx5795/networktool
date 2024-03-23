use local_ip_address;
#[tauri::command]
pub fn get_ipaddr() -> String {
    println!("-------------");
    let interfaces = local_ip_address::list_afinet_netifas().unwrap();
    interfaces.iter().map(|(_, ip)| ip.to_string()).collect::<String>()
}