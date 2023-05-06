fn main() {
    if let Err(e) = ite8291r2_ctl::run() {
        eprintln!("{e}");
    }
}
