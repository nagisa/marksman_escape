extern crate marksman_escape;

use marksman_escape::Unescape;
use std::io::{stdin, stdout, Read, Write};

fn main(){
    let stdin = stdin();
    let stdout = stdout();
    let mut lstdout = stdout.lock();

    for unescaped in Unescape::new(stdin.lock().bytes().map(|x|{ x.unwrap() })) {
        let _ = lstdout.write(&[unescaped; 1]);
    }
}
