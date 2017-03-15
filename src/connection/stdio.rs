use std::process::Command;
use std::process::Child;

struct StdIOConfiguration<'std> {
    path: &'std str,
    arg: &'std str
}

struct StdIOConnection {    
    process: Child
}

impl<'std> StdIOConnection {
    fn new_with_config(config: StdIOConfiguration<'std>) -> StdIOConnection {
        let process = Command::new(config.path)
                              .arg(config.arg)
                              .spawn()
                              .expect(("failed to execute ".to_string() + config.path).as_ref());
        StdIOConnection {
            process: process
        }
    }
}
