/// A very simple colorized `cat` clone, using `kit` as a library.
/// See `src/bin/kit` for the full `kit` application.
use kit::PrettyPrinter;

fn main() {
    PrettyPrinter::new()
        .header(true)
        .grid(true)
        .line_numbers(true)
        .input_files(std::env::args_os().skip(1))
        .print()
        .unwrap();
}
