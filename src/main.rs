use std::process;

struct Ignorable(bool); // newtype
struct Command(&'static str, Ignorable);

impl Command {
  fn new(cmd : &'static str) -> Self {
    Command(cmd, Ignorable(false))
  }
}

struct Rebase {
  command: Vec<Command>,
}

impl Rebase {
  fn new() -> Self {
    Rebase {
      command: vec![Command("stash", Ignorable(true)), Command::new("fetch"), Command::new("checkout master"), Command("rebase origin/master", Ignorable(true))],
    }
  }

  fn run(&self) {
    for cmd in &self.command {
      println!(">> git {}", cmd.0);
      let output = process::Command::new("git")
        .args(cmd.0.split(' ').collect::<Vec<&str>>())
        .output() // return output if it's Ok.
        .expect(&format!("{} failed", cmd.0));
      if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stderr));
        // destructuring
        let Ignorable(ignore) = cmd.1;
        if ignore {
          break;
        }
      }
    }
  }
}

fn main() {
  let rebase = Rebase::new();
  rebase.run();
}
