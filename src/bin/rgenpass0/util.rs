/// タイムキーパー
pub struct TimeKeeper {
	/// 開始タイムスタンプ
	start: std::time::Instant,
}

impl TimeKeeper {
	/// 新しいインスタンスを返します。
	pub fn new() -> TimeKeeper {
		return TimeKeeper { start: std::time::Instant::now() };
	}

	/// 終了の判断
	pub fn is_over(&self) -> bool {
		let current_time = std::time::Instant::now();
		let elapsed = current_time - self.start;
		return 2300 <= elapsed.as_millis();
	}

	/// タイマーをリセット
	pub fn reset(&mut self) {
		self.start = std::time::Instant::now();
	}
}

pub fn get_runnning_path() -> Result<String, Box<dyn std::error::Error>> {
	let path = std::env::current_exe()?;
	let path = path.as_path();
	let path = path.to_str().unwrap().to_string();
	return Ok(path);
}
