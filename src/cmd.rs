use std::process::Command;

pub fn run(cmd: String) -> Result<String, String> {
    let output = Command::new("sh")
	.arg("-c")
	.arg(cmd)
	.output()
	.expect("failed to execute process");

    Ok(String::from_utf8(output.stdout).unwrap())
}
