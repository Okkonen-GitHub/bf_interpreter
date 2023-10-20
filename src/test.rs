#[cfg(test)]
mod tests {

    use std::process::Command;

    macro_rules! pathfmt {
        ($p:expr) => {
            concat!("tests/", $p)
        };
    }
    macro_rules! runcmd {
        ($src:expr, $expect: expr $(, $ascii_flag: literal)?) => {
            let cmd = Command::new("cargo")
                .args(["run", "--", $($ascii_flag,)? pathfmt!($src)])
                .output()
                .expect("Couldn't run the command");
            let op = cmd.stdout;
            let ex = $expect.chars().map(|x| x as u8).collect::<Vec<u8>>();
            assert_eq!(op, ex)
        };
    }

    #[test]
    fn simple_instructions() {
        runcmd!("simple.b", "2");
    }
    #[test]
    fn loop_test() {
        runcmd!("loop.b", "254");
    }
    #[test]
    fn hello_world() {
        runcmd!("hello_world.b", "7210110810811132871111141081003310");
        runcmd!("hello_world.b", "Hello World!\n", "-a");
    }
    #[test]
    fn hello_world_complex() {
        runcmd!("complex_hellow.b", "7210110810811132871111141081003310");
        runcmd!("complex_hellow.b", "Hello World!\n", "-a");
    }
    #[test]
    fn hello_world_other() {
        runcmd!("other_hw.b", "7210110810811144328711111410810033");
        runcmd!("other_hw.b", "Hello, World!", "-a");
    }
    #[test]
    fn shortest_hw() {
        runcmd!("shortest_hw.b", "7210110810811144328711111410810033");
        runcmd!("shortest_hw.b", "Hello, World!", "-a");
    }
}
