pub fn log<T: AsRef<str>>(s: T) {
    eprintln!("LOGASS: {}", s.as_ref());
}