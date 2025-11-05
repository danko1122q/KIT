/// A program that prints its own source code using the kit library
use kit::{PagingMode, PrettyPrinter, WrappingMode};

fn main() {
    PrettyPrinter::new()
        .header(true)
        .grid(true)
        .line_numbers(true)
        .use_italics(true)
        // The following line will be highlighted in the output:
        .highlight(line!() as usize)
        .theme("1337")
        .wrapping_mode(WrappingMode::Character)
        .paging_mode(PagingMode::QuitIfOneScreen)
        .input_file(file!())
        .print()
        .unwrap();
}
