use std::fs::File;
use std::io::prelude::*;

pub fn createfile(filename: &str, content: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content)?;
    Ok(())
}

pub fn genwificonf<'longevity>(ssid: &str, hw_mode: &str, channel: &u8, passphrase: &str) -> String {
    format!("interface=wlan0
# SSID to be used in IEEE 802.11 management frames
ssid={}
# Driver interface type (hostap/wired/none/nl80211/bsd)
driver=nl80211
# Country code (ISO/IEC 3166-1)
#country_code=US

# Operation mode (a = IEEE 802.11a (5 GHz), b = IEEE 802.11b (2.4 GHz)
hw_mode={}
# Channel number
channel={}
# Maximum number of stations allowed
#max_num_sta=5

# Bit field: bit0 = WPA, bit1 = WPA2
wpa=2
# Bit field: 1=wpa, 2=wep, 3=both
auth_algs=1

# Set of accepted cipher suites; disabling insecure TKIP
wpa_pairwise=CCMP
# Set of accepted key management algorithms
wpa_key_mgmt=WPA-PSK
wpa_passphrase={}

# hostapd event logger configuration
logger_stdout=-1
logger_stdout_level=2

ignore_broadcast_ssid=0
macaddr_acl=0

# Uncomment and modify the following section if your device supports 802.11n
## Enable 802.11n support
ieee80211n=1
## QoS support
wmm_enabled=1
## Use iw list to show device capabilities and modify ht_capab accordingly
#ht_capab=[HT40+][SHORT-GI-40][TX-STBC][RX-STBC1][DSSS_CCK-40]", ssid, hw_mode, channel, passphrase)
}
