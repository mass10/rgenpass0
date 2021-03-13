use super::client;
use super::generator;
use super::listener;
use super::server;

/// 実行
pub fn run(request: &str) -> Result<(), Box<dyn std::error::Error>> {
	// オプションによる振り分け
	if request == "--server" {
		// 自身がサーバーになって処理を終了する。
		listener::run_as_server()?;

		return Ok(());
	} else {
		// サーバーに状況を問い合わせます。
		let current_complexity = client::try_to_request_server();
		if current_complexity == 0 {
			// サーバーの起動を試みます。
			// println!("[TRACE] サーバーの起動を試みます。");
			server::spawn_server_process()?;
		}
		// 出力
		generator::generate_password(current_complexity)?;

		return Ok(());
	}
}
