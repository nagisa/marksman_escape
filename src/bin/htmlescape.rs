extern crate marksman_escape;

use marksman_escape::Escape;
use std::io::{stdin, stdout, ReadExt, Write};

fn main(){
    let mut stdout = stdout();
    let mut lstdout = stdout.lock();
    let mut stdin = stdin();

    for escaped in Escape::new(stdin.lock().bytes().map(|x|{ x.unwrap() })) {
        lstdout.write(&[escaped; 1]);
    }
}
