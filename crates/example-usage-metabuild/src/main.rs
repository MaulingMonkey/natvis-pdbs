use example_crate_with_natvis::Flags;

fn main() {
    let flags = Flags::A | Flags::B;
    bugsalot::debugger::break_if_attached();
    println!("{:?}", flags);
}
