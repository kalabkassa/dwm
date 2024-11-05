use std::fs;
use std::thread;
use std::time::Duration;
use std::io::{self, ErrorKind};
use std::process::{Command, Stdio};
use std::io::{Write, BufReader, BufRead};

fn is_device_connected(device_mac: &str) -> bool {
    let output = Command::new("bluetoothctl")
        .arg("info")
        .arg(device_mac)
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to start bluetoothctl");

        let connected_bytes = b"Connected: yes";
        output.stdout.windows(connected_bytes.len()).any(|window| window == connected_bytes)    
}

fn get_system_info(device_mac: &str) -> String {
    let volume = Command::new("pamixer")
        .arg("--get-volume-human")
        .output()
        .expect("Failed to get volume")
        .stdout;

    let headset_info = Command::new("upower")
        .arg("-i")
        .arg("/org/freedesktop/UPower/devices/headset_dev_24_81_C7_F8_61_72")
        .output()
        .expect("Failed to get headset info");

    let battery_capacity = Command::new("cat")
        .arg("/sys/class/power_supply/BAT0/capacity")
        .output()
        .expect("Failed to get battery capacity");

    let memory_usage = Command::new("free")
        .arg("-m")
        .output()
        .expect("Failed to get memory usage");

    let date = Command::new("date")
        .arg("+%A %d %B %H:%M")
        .output()
        .expect("Failed to get date");

    let wifi_ssid = Command::new("iwgetid")
        .arg("wlan0")
        .arg("-r")
        .output()
        .expect("Failed to get Wi-Fi SSID");

    // Convert outputs to strings
    let volume_str = String::from_utf8_lossy(&volume).trim().to_string();
    let headset_name = String::from_utf8_lossy(&headset_info.stdout)
        .lines()
        .nth(1) // Adjust according to the relevant line
        .unwrap_or("")
        .trim()
        .replace("model:", "") 
        .trim()
        .to_string();
    let headset_battery = String::from_utf8_lossy(&headset_info.stdout)
        .lines()
        .nth(9) // Adjust according to the relevant line
        .unwrap_or("")
        .trim()
        .replace("percentage:", "") 
        .trim()
        .to_string();
    let battery_str = String::from_utf8_lossy(&battery_capacity.stdout).trim().to_string();
    let memory_str = String::from_utf8_lossy(&memory_usage.stdout) .lines()
        .nth(1) // Get the second line for "Mem"
        .map(|line| line.split_whitespace().nth(2).unwrap_or("")) // Get the second column (used)
        .unwrap_or("")
        .to_string();
    let date_str = String::from_utf8_lossy(&date.stdout).trim().to_string();
    let wifi_str = String::from_utf8_lossy(&wifi_ssid.stdout).trim().to_string();

    let headset_str;
    if is_device_connected(device_mac) {
        headset_str = headset_name + "  " + &headset_battery;
    } else {
        headset_str = String::from("Disconnected");
    }   

    // Construct the final string
    format!(
        "{} | {} | {} % | {} MB | {} | wifi: {}",
        volume_str,
        headset_str,
        battery_str,
        memory_str,
        date_str,
        wifi_str
    )
}

fn set_bar(system_info: &str) {
    Command::new("xsetroot")
        .arg("-name")
        .arg(system_info)
        .output()
        .expect("Failed to set the bar");
}

fn connect(device_mac: &str) {
    let mut child = Command::new("bluetoothctl")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start bluetoothctl");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    let stdout = child.stdout.take().expect("Failed to open stdout");
    
    // Send the connect command
    writeln!(stdin, "connect {}", device_mac).expect("Failed to write to stdin");

    // Read output for a response
    let reader = BufReader::new(stdout);
    let mut connection_successful = false;

    for line in reader.lines() {
        match line {
            Ok(output) => {
                println!("{}", output);
                if output.contains("Connection successful") {
                    connection_successful = true;
                    break;
                } else if output.contains("Failed to connect") {
                    println!("Failed to connect to {}", device_mac);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading stdout: {}", e);
                break;
            }
        }

        // Timeout check (5 seconds)
        thread::sleep(Duration::from_millis(100));
    }

    // Exit the bluetoothctl session
    writeln!(stdin, "exit").expect("Failed to write to stdin");

    // Wait for the child process to finish
    let _ = child.wait().expect("Child process wasn't running");
    
    if connection_successful {
        println!("Successfully connected to {}", device_mac);
    }
}

fn read_battery_info() -> io::Result<(String, i32)> {
    let status_path = "/sys/class/power_supply/BAT0/status"; // Adjust if needed
    let capacity_path = "/sys/class/power_supply/BAT0/capacity"; // Adjust if needed

    let status = fs::read_to_string(status_path)?.trim().to_string();
    let capacity = fs::read_to_string(capacity_path)?
        .trim()
        .parse::<i32>()
        .map_err(|_| io::Error::new(ErrorKind::InvalidData, "Invalid capacity data"))?;

    Ok((status, capacity))
}

fn send_notification() {
    let _ = std::process::Command::new("notify-send")
        .arg("Battery Low")
        .arg("-t")
        .arg("1000")
        .output();
}

fn main() {
    let device_mac = "24:81:C7:F8:61:72"; 

    thread::spawn( move || {
        loop {
            set_bar(&get_system_info(device_mac));
            thread::sleep(Duration::from_millis(100));
        }
    });

    loop {       
        if !is_device_connected(device_mac) {
            connect(device_mac)
        }

        match read_battery_info() {
            Ok((status, level)) => {
                if status == "Discharging" && level < 25 {
                    send_notification();
                }
            }
            Err(e) => eprintln!("Error reading battery information: {}", e),
        }

        thread::sleep(Duration::from_millis(1000));
    }
}