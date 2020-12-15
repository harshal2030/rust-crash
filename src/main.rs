mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vector;

fn main() {
    print::print();
    vars::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
    vector::run();
}
