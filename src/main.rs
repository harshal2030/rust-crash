mod print;
mod vars;
mod types;
mod strings;

fn main() {
    print::print();
    vars::run();
    types::run();
    strings::run();
}
