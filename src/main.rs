use rppal::gpio::{Gpio, Level, Mode};
use rppal::system::DeviceInfo;
use env_logger;
use actix;
use actix_web::{self, middleware, server, App, HttpRequest, HttpResponse};
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService, RequestIdentity};

use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::{self, Duration};
use std::fs::File;
use std::io::Write;

// pin numbers
const RELAY_LED: u8 = 15;
const STATUS_LED: u8 = 24;
const SENSE_PIN: u8 = 23;

struct AppState {
	gpio_ctrl: Arc<Mutex<Gpio>>
}

fn toggle(req: &HttpRequest<AppState>) -> HttpResponse {
	println!("{:?}", req);
	let gpio = req.state().gpio_ctrl.lock().unwrap();
	gpio.write(RELAY_LED, Level::High);
	gpio.write(STATUS_LED, Level::High);
	thread::sleep(Duration::from_secs(1));
	gpio.write(RELAY_LED, Level::Low);
	gpio.write(STATUS_LED, Level::Low);
	HttpResponse::Ok().body("Relay toggled")
}

fn sense(req: &HttpRequest<AppState>) -> HttpResponse {
	println!("{:?}", req);
	let gpio = req.state().gpio_ctrl.lock().unwrap();
	let sense_value = gpio.read(SENSE_PIN).unwrap();
	println!("Sense: {:?}", sense_value);
	HttpResponse::Ok().body(match sense_value {
		Level::High => "CLOSED",
		Level::Low => "OPEN"
	})
}

fn main() {
	println!("Garage Controller has started.");

	let device_info = DeviceInfo::new().unwrap();
	println!(
		"Model: {} (SoC: {})",
		device_info.model(),
		device_info.soc()
	);

	// Initialize Gpio
	let mut gpio = Gpio::new().unwrap();
	gpio.set_mode(RELAY_LED, Mode::Output);
	gpio.set_mode(STATUS_LED, Mode::Output);
	gpio.set_mode(SENSE_PIN, Mode::Input);

	let gpio_mutex = Arc::new(Mutex::new(gpio));

	::std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();
	let sys = actix::System::new("ws-example");

	server::new(move || {
		App::with_state(AppState {gpio_ctrl: gpio_mutex.clone()})
		.middleware(middleware::Logger::default())
		.middleware(IdentityService::new(
			CookieIdentityPolicy::new(&[0; 32])
			.name("auth-example")
			.secure(false)
		))
		.resource("/toggle", |r| r.f(toggle))
		.resource("/sense", |r| r.f(sense))
	}).bind("0.0.0.0:8888")
	.unwrap()
	.start();

	println!("Started HTTP server: 0.0.0.0:8888");
	let _ = sys.run();

	
}
