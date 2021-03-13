use super::application_error;

#[derive(Debug, Default, serde_derive::Serialize, serde_derive::Deserialize)]
struct ApplicationResponseContent {
	content: String,
	current: String,
}

/// 要求をパースします。
///
/// # Arguments
/// `request_string` 要求の本文
#[allow(unused)]
fn parse_response_data_json(request_string: &str) -> Result<ApplicationResponseContent, Box<dyn std::error::Error>> {
	// 要求をパース
	let result = serde_json::from_str::<ApplicationResponseContent>(request_string);
	if result.is_err() {
		println!("[TRACE] 応答のパースに失敗しました。情報: {}", result.err().unwrap());
		return Err(Box::new(application_error::ApplicationError::new("無効な要求です。")));
	}
	let request_data = result.unwrap();
	return Ok(request_data);
}

/// メッセージ送信側
fn try_get() -> Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;
	use std::io::Write;

	// println!("[TRACE] set connection timeout.");
	let addr1 = std::net::SocketAddr::from(([127, 0, 0, 1], 8082));
	std::net::TcpStream::connect_timeout(&addr1, std::time::Duration::from_micros(200))?;
	let addresses = [addr1];

	// サーバーに接続を試みます。
	// println!("[TRACE] サーバーに接続を試みます。");
	// let result = std::net::TcpStream::connect("127.0.0.1:8082");
	let result = std::net::TcpStream::connect(&addresses[..]);
	if result.is_err() {
		let error = result.err().unwrap();
		println!("[ERROR] サーバーに接続できません。情報: {}", error);
		return Ok("".to_string());
	}
	let mut connection = result.unwrap();

	// 要求を送信
	let message = "{\"content\": \"HELLO\"}";
	connection.write_all(message.as_bytes())?;

	// 応答を受信
	let mut response_buffer = "".to_string();
	connection.read_to_string(&mut response_buffer)?;
	let response = parse_response_data_json(&response_buffer)?;

	// 切断
	connection.shutdown(std::net::Shutdown::Both)?;

	return Ok(response.current);
}

pub fn try_to_request_server() -> u8 {
	// ローカルサーバーに接続を試みます。
	let result = try_get();
	if result.is_err() {
		// let error = result.err().unwrap();
		// println!("[ERROR] GET 失敗。情報: {}", error);
		return 0;
	}

	// 複雑性の水位線
	let current_complexity = result.unwrap();
	let current_complexity: u8 = current_complexity.parse().unwrap_or_default();
	return current_complexity;
}
