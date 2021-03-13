use super::util;

// サーバーの起動を試みます。
pub fn spawn_server_process() -> Result<(), Box<dyn std::error::Error>> {
	let executing_app_path = util::get_runnning_path()?;
	let mut command = std::process::Command::new(&executing_app_path);
	command.arg("--server");
	command.spawn()?;
	return Ok(());
}
