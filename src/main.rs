mod cmd;

fn main() {
    let hello = cmd::run("echo hello".to_string()).unwrap();
    println!("{}", hello);
}
