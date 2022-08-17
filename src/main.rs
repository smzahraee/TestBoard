use std::process::Command;
use std::time::Duration;
use std::{fmt::format, thread::sleep};
use sysfs_gpio::{Direction, Pin};
use rand::Rng;

const CAN_NAME:&str="can0";
const PING_ADDRESS:&str="192.168.25.160";
const GPIO_PIN: u64 = 89;

fn main() {
    // gpio
    std::thread::spawn(|| {
        println!("Starting GPIO");
        
        let my_led = Pin::new(GPIO_PIN);
        my_led
            .with_exported(|| {
                my_led.set_direction(Direction::Out).unwrap();
                loop {
                    my_led.set_value(0).unwrap();
                    sleep(Duration::from_millis(200));
                    my_led.set_value(1).unwrap();
                    sleep(Duration::from_millis(200));
                }
            })
            .unwrap();
    });

    //serialport
    std::thread::spawn(|| {
        println!("\nStarting Serial Send");

        let ports = serialport::available_ports().expect("No ports found!");
        println!("find {} serial ports", ports.len());
        for mut p in ports {
            let name = p.port_name.clone();
            println!("openning {} {:?}", p.port_name, p.port_type);
            let mut port = serialport::new(p.port_name, 115200)
                .open()
                .expect(concat!("Failed to open port ", stringify!(p.port_name)));

            let msg = format!("hello serial {}", name);
            port.write_all(msg.as_bytes()).expect("Can not send");
        }
    });

    //iio
    std::thread::spawn(|| {
        println!("\nStarting IIO read");
        for i in 0..7 {
            let v = std::fs::read_to_string(format!(
                "/sys/bus/iio/devices/iio:device0/in_voltage{}_raw",
                i
            ))
            .expect("Should have been able to read the file");
            println!("in_voltage{i}_raw is {v}");
        }
    });

    //can
    std::thread::spawn(|| {
        println!("\nStarting Can read");
        let sock_rx=socketcan::CANSocket::open(CAN_NAME).unwrap();

        loop {
            let frame=sock_rx.read_frame().unwrap();
            let data=frame.data().clone();
            println!("can read frame {:?}", data.to_vec());
        }
    });
    //can
    std::thread::spawn(|| {
        println!("\nStarting Can write");
        let sock_rx=socketcan::CANSocket::open(CAN_NAME).unwrap();
        let mut counter=0;
        loop {
            if { let tmp = counter; counter += 1; tmp }>10
             {return;}
            let random_bytes = rand::thread_rng().gen::<[u8; 8]>();
            let frame = socketcan::CANFrame::new(127,&random_bytes,true,false).unwrap();
            sock_rx.write_frame(&frame).unwrap();
            let data=frame.data().clone();
            println!("can write frame {:?}", data.to_vec());
            sleep(Duration::from_millis(250));
            
        }
    });

    print!("\npinging the {}",PING_ADDRESS);
    Command::new("ping")
        .arg(PING_ADDRESS)
        .spawn()
        .expect("ping command failed to start");

    loop {
        std::thread::sleep(Duration::from_millis(1));
    }
}
