use bossy::Command;

fn main() -> bossy::Result<()> {
    Command::impure("ls").with_arg("-l").run_and_wait()?;
    Ok(())
}
