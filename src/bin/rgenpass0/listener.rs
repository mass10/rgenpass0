extern crate serde_json;

use super::application_error;

#[derive(Debug, Default, serde_derive::Serialize, serde_derive::Deserialize)]
struct ApplicationRequestContent {
	content: String,
}

/// 要求をパースします。
///
/// # Arguments
/// `request_string` 要求の本文
#[allow(unused)]
fn parse_request_data_json(request_string: &str) -> Result<ApplicationRequestContent, Box<dyn std::error::Error>> {
	// 要求をパース
	let result = serde_json::from_str::<ApplicationRequestContent>(request_string);
	if result.is_err() {
		// println!("[ERROR] 要求文字列をパースできません。情報: {}", result.err().unwrap());
		return Err(Box::new(application_error::ApplicationError::new("無効な要求です。")));
	}
	let request_data = result.unwrap();
	return Ok(request_data);
}

/// ピアのソケットからリクエストデータを読みます。
fn accept(mut peer_socket: std::net::TcpStream, current_complexity: u8) -> Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;
	use std::io::Write;

	// 読み込みタイムアウトは15秒
	let result = peer_socket.set_read_timeout(Some(std::time::Duration::from_secs(15)));
	if result.is_err() {
		// println!("[ERROR] タイムアウト設定に失敗しています。(情報: {})", result.err().unwrap());
		return Ok("FATAL".to_string());
	}

	// リクエストデータ
	let mut line = "".to_string();
	let result = peer_socket.read_to_string(&mut line);
	if result.is_err() {
		let error = result.err().unwrap();
		if error.kind() != std::io::ErrorKind::WouldBlock {
			// println!("[ERROR] リクエストデータの受信が完了しませんでした。情報: {}, [{}]", error, line);
			return Ok("REJECTED".to_string());
		}
	}

	// リクエストデータをパース
	let result = parse_request_data_json(&line);
	if result.is_err() {
		// let error = result.err().unwrap();
		// println!("[ERROR] リクエストデータのパースに失敗しました。情報: {}", error);
		return Ok("REJECTED".to_string());
	}

	// リクエストデータは読み捨て
	// let request_data = result.unwrap();

	// 応答
	let json_text = format!("{{\"content\": \"HELO\", \"current\": \"{}\"}}", current_complexity);
	peer_socket.write_all(json_text.as_bytes())?;

	return Ok("ACCEPTED".to_string());
}

/// リスナーを起動します。
fn start_listener() -> Result<(), Box<dyn std::error::Error>> {
	use super::util;

	// リスナーを起動します。
	let addresses = [std::net::SocketAddr::from(([127, 0, 0, 1], 8082))];
	let result = std::net::TcpListener::bind(&addresses[..]);
	if result.is_err() {
		let error = result.err().unwrap();
		let message_text = error.to_string();
		if message_text.contains("10048") {
			// Address already in use
			return Ok(());
		}
		// その他の問題
		return Ok(());
	}

	let listener = result.unwrap();

	let result = listener.set_nonblocking(true);
	if result.is_err() {
		println!("[ERROR] ノンブロッキング初期化操作でエラーが発生しました。情報: {}", result.err().unwrap());
	}

	// タイムキーパー
	let mut time_keeper = util::TimeKeeper::new();

	// 複雑性
	let mut current_complexity = 0u8;

	// 接続要求を待っています。
	for stream in listener.incoming() {
		if time_keeper.is_over() {
			break;
		}
		if stream.is_ok() {
			// 接続要求を検出しました。
			let result = accept(stream.unwrap(), current_complexity);
			if result.is_err() {
				println!("[ERROR] ピアとの通信中にエラーが発生しました。情報: {}", result.err().unwrap());
			} else {
				let status = result.ok().unwrap();
				if status == "ACCEPTED" {
					current_complexity += 1;
					time_keeper.reset();
				}
			}
			// コネクションを切断
			// peer_socket.shutdown(std::net::Shutdown::Both)?;
			std::thread::sleep(std::time::Duration::from_millis(5));
		} else if stream.is_err() {
			let e = stream.err().unwrap();
			if e.kind() == std::io::ErrorKind::WouldBlock {
				std::thread::sleep(std::time::Duration::from_millis(5));
			} else {
				println!("[ERROR] サーバーは待機中にエラーを検出しました。情報: {}", e);
				break;
			}
		}
	}

	return Ok(());
}

pub fn run_as_server() -> Result<(), Box<dyn std::error::Error>> {
	return start_listener();
}
