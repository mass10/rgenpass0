mod application;
mod application_error;
mod client;
mod generator;
mod listener;
mod server;
mod util;

/// Usage.
fn usage() {
	println!("No usage.");
}

/// アプリケーションのエントリーポイント
fn main() {
	let mut request = "".to_string();
	for e in std::env::args().skip(1) {
		if e == "--help" {
			usage();
			return;
		}
		if e == "--server" {
			request = "--server".to_string();
		}
	}

	// アプリケーションを起動
	let result = application::run(&request);
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
}
