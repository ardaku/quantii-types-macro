use std::io::{stderr, stdout, Write};
use std::process::{Command, Stdio};

fn main() {
    let out = Command::new("cargo")
        .arg("test")
        .arg("--quiet")
        .args(["--target", "wasm32-unknown-unknown"])
        .args(["-p", "tester"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .output()
        .expect("Failed to execute cargo");

    stdout()
        .write_all(&out.stdout)
        .expect("Failed to write to console's stdout!");
    stderr()
        .write_all(&out.stderr)
        .expect("Failed to write to console's stderr!");
}

#[cfg(test)]
pub mod tests {
    use quantii_types_macro::closure_to_fp;
    use super::closure_to_fn_once;

    #[test]
    fn it_works() {
        let x = "Hello, World".to_owned();
        // Not a function pointer!!
        let input_closure: Box<dyn FnOnce(()) -> String> = closure_to_fn_once(move |_| x);
        drop(input_closure);
        
        let fp = closure_to_fp!(input_closure);
    }
}

#[allow(dead_code)]
fn closure_to_fn_once<T, F: FnOnce(()) -> T>(in_fn: F) -> Box<F> {
    Box::new(in_fn)
}
