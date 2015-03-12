extern crate marksman_escape;

use marksman_escape::Unescape;
use std::io::{stdin, stdout, ReadExt, Write};

fn main(){
    let mut stdout = stdout();
    let mut lstdout = stdout.lock();
    let mut stdin = stdin();

    for unescaped in Unescape::new(stdin.lock().bytes().map(|x|{ x.unwrap() })) {
        lstdout.write(&[unescaped; 1]);
    }
}
