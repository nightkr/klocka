pub fn retry_until<F, Ret, Cond>(f: F, cond: Cond, tries: u32) -> Option<Ret>
    where F : Fn() -> Ret,
          Cond : Fn(&Ret) -> bool {
    for _ in 0..tries {
        let rv = f();
        if cond(&rv) {
            return Some(rv);
        }
    }
    None
}
