use haxor::Process;

fn main() {
    dbg!(Process::with_pid(3440).unwrap());
    dbg!(Process::with_name("notepad.exe").unwrap());
}
