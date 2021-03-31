use super::client;
use super::generator;
use super::listener;
use super::server;

pub struct Application;

impl Application {
	/// 新しいインスタンスを返します。
	///
	/// ### Returns
	/// `Application` の新しいインスタンス
	pub fn new() -> Application {
		Application {}
	}

	/// 実行
	///
	/// ### Arguments
	/// * `request` 呼び出しオプション
	pub fn run(&self, request: &str) -> Result<(), Box<dyn std::error::Error>> {
		// オプションによる振り分け
		if request == "--server" {
			// 自身がサーバーになって処理を終了する。
			let listener = listener::Listener::new();
			listener.run_as_server()?;

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
}
