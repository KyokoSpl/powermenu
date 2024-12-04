use std::process::Command;

pub fn lockscreen() {
    let wm = Command::new("wmctrl").arg("-m").output();
    let output = wm.expect("Failed to execute command");
    let wm_name = String::from_utf8_lossy(&output.stdout);
    println!("wm_name{}", wm_name);
    if wm_name.contains("Hyprland") {
        let _ = Command::new("hyprlock").output();
        let _ = Command::new("killall").arg("powermenu").output();
    }
}
pub fn logout() {
    let wm = Command::new("wmctrl").arg("-m").output();
    let output = wm.expect("Failed to execute command");
    let wm_name = String::from_utf8_lossy(&output.stdout);
    println!("wm_name{}", wm_name);
    if wm_name.contains("Hyprland") {
        println!("hyprland");
        let _ = Command::new("hyprctl").arg("dispatch").arg("exit").output();
    } else {
        println!("not hypr");
    }
}
pub fn reboot() {
    let _ = Command::new("systemctl").arg("reboot").output();
}
pub fn shutdown() {
    let _ = Command::new("systemctl").arg("poweroff").output();
}
pub fn suspend() {
    let _ = Command::new("systemctl").arg("suspend").output();
}
