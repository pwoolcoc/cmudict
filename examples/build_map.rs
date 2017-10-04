extern crate cmudict;

fn main() {
    let args = ::std::env::args().skip(1).collect::<Vec<_>>();
    let num = args[0].parse().expect("Could not get num");
    let map = cmudict::build_map(Some(num));
}

