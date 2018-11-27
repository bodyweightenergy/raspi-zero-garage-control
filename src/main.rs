extern crate jsonrpc_core;
extern crate jsonrpc_http_server;
extern crate rppal;
// extern crate serde_json;

use jsonrpc_core::*;
use jsonrpc_http_server::*;
// use serde_json::*;
use rppal::gpio::{Gpio, Level, Mode};
use rppal::system::DeviceInfo;

use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

// pin numbers
const RELAY_LED: u8 = 14;
const STATUS_LED: u8 = 24;
const SENSE_PIN: u8 = 23;

fn main() {
	println!("Garage Controller has started.");

	let device_info = DeviceInfo::new().unwrap();
	println!(
		"Model: {} (SoC: {})",
		device_info.model(),
		device_info.soc()
	);

	// Make pins output
	let mut gpio = Gpio::new().unwrap();
	gpio.set_mode(RELAY_LED, Mode::Output);
	gpio.set_mode(STATUS_LED, Mode::Output);
	gpio.set_mode(SENSE_PIN, Mode::Input);

	let gpio_mutex = Arc::new(Mutex::new(gpio));
	let gpio_mutex_toggle = Arc::clone(&gpio_mutex);
	let gpio_mutex_sense = Arc::clone(&gpio_mutex);

	// let mutex_1 = Arc::clone(&mutex);
	// let t1 = thread::spawn(move || {
	// 	for _ in 0..10 {
	// 		gpio_lock_write(&mutex_1, RELAY_LED, Level::High);
	// 		sleep(Duration::from_millis(500));
	// 		gpio_lock_write(&mutex_1, RELAY_LED, Level::Low);
	// 		sleep(Duration::from_millis(500));
	// 	}
	// });

	// let mutex_2 = Arc::clone(&mutex);
	// let t2 = thread::spawn(move || {
	// 	for _ in 0..10 {
	// 		gpio_lock_write(&mutex_2, STATUS_LED, Level::High);
	// 		sleep(Duration::from_millis(700));
	// 		gpio_lock_write(&mutex_2, STATUS_LED, Level::Low);
	// 		sleep(Duration::from_millis(700));
	// 	}
	// });

	// let _ = t1.join();
	// let _ = t2.join();

	let mut io = IoHandler::new();
	io.add_method("say_hello", |_: Params| {
		Ok(Value::String("hello".to_string()))
	});

	io.add_method("toggle", move |params: Params| {
		let mut toggle_interval = 1000;
		let mut repetitions = 1;
		match params {
			Params::Map(m) => {
				// Apply "interval" if found
				if m.contains_key("interval") {
					match m.get("interval") {
						Some(v) => match v {
							Value::Number(n) => match n.as_u64() {
								Some(i) => toggle_interval = i,
								None => {}
							},
							_ => {}
						},
						None => {}
					};
				}
				// Apply "repetitions" if found
				if m.contains_key("repetitions") {
					match m.get("repetitions") {
						Some(v) => match v {
							Value::Number(n) => match n.as_u64() {
								Some(i) => repetitions = i,
								None => {}
							},
							_ => {}
						},
						None => {}
					};
				}
			}
			_ => {}
		}
		let mutex_clone = gpio_mutex_toggle.clone();
		for _ in 0..repetitions {
			gpio_lock_write(&mutex_clone, RELAY_LED, Level::High);
			sleep(Duration::from_millis(toggle_interval));
			gpio_lock_write(&mutex_clone, RELAY_LED, Level::Low);
			sleep(Duration::from_millis(toggle_interval));
		}
		Ok(Value::String("Done".to_string()))
	});

	io.add_method("sense", move |_: Params| {
		let handle = gpio_mutex_sense.lock().unwrap();
		match handle.read(SENSE_PIN) {
			Ok(level) => Ok(Value::String(format!("{}", level))),
			Err(_) => Err(Error::internal_error())
		}
	});

	let _server = ServerBuilder::new(io)
		.start_http(&"192.168.1.197:3030".parse().unwrap())
		.expect("Unable to start RPC server");

	println!("Server started @ 192.168.1.197:3030");
	_server.wait();
}

fn gpio_lock_write(mutex: &Arc<Mutex<Gpio>>, pin: u8, level: Level) {
	let handle = mutex.lock().unwrap();
	handle.write(pin, level);
	println!("set pin {} to {}", pin, level);
}
