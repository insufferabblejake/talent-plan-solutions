#[allow(unused_imports)]
use::std:: {
    env
};

fn main() {
    let args = env::args();
    for (i, arg) in args.enumerate() {
        println!("{}: {:?}", i, arg);
    }
}
