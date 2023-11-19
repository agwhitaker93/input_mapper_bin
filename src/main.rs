#![allow(dead_code)]
use std::io::prelude::*;
use active_win_pos_rs::get_active_window;
use evdev::{Device, enumerate, InputEventKind, Key, uinput::{VirtualDevice, VirtualDeviceBuilder}, AttributeSet, InputEvent, EventType};

pub fn pick_device() -> Device {
    let mut args = std::env::args_os();
    args.next();
    if let Some(dev_file) = args.next() {
        Device::open(dev_file).unwrap()
    } else {
        let mut devices = enumerate().map(|t| t.1).collect::<Vec<_>>();
        // readdir returns them in reverse order from their eventN names for some reason
        devices.reverse();
        for (i, d) in devices.iter().enumerate() {
            println!("{}: {}", i, d.name().unwrap_or("Unnamed device"));
        }
        print!("Select the device [0-{}]: ", devices.len());
        let _ = std::io::stdout().flush();
        let mut chosen = String::new();
        std::io::stdin().read_line(&mut chosen).unwrap();
        let n = chosen.trim().parse::<usize>().unwrap();
        devices.into_iter().nth(n).unwrap()
    }
}

fn virtual_mouse() -> std::io::Result<VirtualDevice> {
    return VirtualDeviceBuilder::new()?
        .name("input-mapper::fake-mouse")
        .with_keys(&AttributeSet::from_iter([
            Key::BTN_LEFT,
            Key::BTN_MIDDLE,
            Key::BTN_RIGHT,
        ]))?
        .build();
}

fn window_stuff() {
    match get_active_window() {
        Ok(active_window) => {
            println!("Active window: {:#?}", active_window);
        }
        Err(()) => {
            println!("Error occurred while getting the active window");
        }
    }
}

fn evdev_stuff() {
    let mut dev = pick_device();
    let mut virtual_mouse = virtual_mouse().unwrap();
    println!("Events:");
    loop {
        for ev in dev.fetch_events().unwrap() {
            let kind = ev.kind();
            let value = ev.value();
            match kind {
                InputEventKind::Key(Key::KEY_T) => {
                    println!("Detected KEY_T event with value {}, emitting BTN_MIDDLE", value);
                    let middle_down = InputEvent::new(EventType::KEY, Key::BTN_MIDDLE.0, value);
                    virtual_mouse.emit(&[middle_down]).unwrap();
                }
                _ => {
                    // ignore
                }
            }
        }
    }
}

fn main() {
    window_stuff();
    evdev_stuff();
}
