use std::process::Command;

pub fn lockscreen() {
    let wm = Command::new("wmctrl").arg("-m").output();
    let output = wm.expect("Failed to execute command");
    let wm_name = String::from_utf8_lossy(&output.stdout);
    println!("wm_name{}", wm_name);
    if wm_name.contains("Hyprland") {
        let _ = Command::new("hyprlock").output();
        let _ = Command::new("killall").arg("powermenu").output();
    } else if wm_name.contains("LG3D") {
        let _lockscreen = Command::new("betterlockscreen").arg("-l").output();
        let _ = Command::new("killall").arg("powermenu").output();
    } else if wm_name.contains("dwm") {
        let _lockscreen = Command::new("betterlockscreen").arg("-l").output();
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
    } else if wm_name.contains("LG3D") {
        let _ = Command::new("qtile")
            .arg("cmd-obj")
            .arg("-o")
            .arg("cmd")
            .arg("-f")
            .arg("shutdown")
            .output();
    } else if wm_name.contains("dwm") {
        let _ = Command::new("pkill").arg("dwm").output();
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
