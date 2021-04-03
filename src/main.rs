mod funct;

fn main(){
    let conf: String = funct::genwificonf("sala", "g", &10, "Koompi-Onelab");

    let result = funct::createfile("/etc/hostapd.conf", &conf.as_bytes());
    match result {
        Ok(()) => (),
        Err(err) => eprintln!("{:?}", err),
    }
}

