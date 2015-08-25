extern crate marksman_escape;

use marksman_escape::Escape;
use std::io::{stdin, stdout, Read, Write};

fn main(){
    let stdin = stdin();
    let stdout = stdout();
    let mut lstdout = stdout.lock();

    for escaped in Escape::new(stdin.lock().bytes().map(|x|{ x.unwrap() })) {
        let _ = lstdout.write(&[escaped; 1]);
    }
}
