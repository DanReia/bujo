use crate::data::Data;
use ansi_term::Colour::{Fixed, Purple};
use terminal_size::{terminal_size, Width};

pub fn basic_print(mut data: Data) {
    let size = terminal_size();
    let (Width(w), _) = size.unwrap();

    let data_hash = data.read().content;
    let max_id_width = match data_hash.keys().max() {
        Some(x) => x.to_string().len(),
        None => 1,
    };
    let border_color = 240;
    // println!("{}","-".repeat(usize::from(w)));
    let remainder = usize::from(w) - max_id_width - 2;
    println!(
        "{}{}{}",
        Fixed(border_color).paint("-".repeat(max_id_width + 1)),
        Fixed(border_color).paint("+"),
        Fixed(border_color).paint("-".repeat(remainder))
    );
    println!(
        "{}{}{}",
        " ".repeat(max_id_width + 1),
        Fixed(border_color).paint("|"),
        Purple.paint(" bujo")
    );
    println!(
        "{}{}{}",
        Fixed(border_color).paint("-".repeat(max_id_width + 1)),
        Fixed(border_color).paint("+"),
        Fixed(border_color).paint("-".repeat(remainder))
    );
    for c in data.read().content.iter() {
        let num_blanks = max_id_width - c.0.to_string().len();
        println!(
            "{}{} {} {} {}",
            " ".repeat(num_blanks),
            c.0,
            Fixed(border_color).paint("|"),
            c.1.signifier,
            c.1.content
        );
    }
}
