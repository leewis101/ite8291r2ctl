fn main() {
    if let Err(e) = ite8291r2ctl::run() {
        eprintln!("{e}");
    }
}
