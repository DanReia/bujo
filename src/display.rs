use crate::data::{BujoObject, Data};
use ansi_term::Colour::{Fixed, Purple};
use terminal_size::{terminal_size, Height, Width};
use chrono::{DateTime,Local,Utc,NaiveDateTime};

#[allow(dead_code)]
pub struct Printer {
    data: Data,
    max_id_width: usize,
    terminal_width: u16,
    terminal_height: u16,
    border_color: u8,
    div_to_end: usize,
}

impl Printer {
    pub fn new(mut incoming_data: Data) -> Printer {
        let read_data = incoming_data.read();

        //get the max width for the id column based on largest id in HashMap
        let data_hash = &read_data.content;
        let max_id_width_temp = match data_hash.keys().max() {
            Some(x) => x.to_string().len(),
            None => 1,
        };

        //Get height and width of active terminal
        let size = terminal_size();
        let (Width(w), Height(h)) = size.unwrap();

        //Get distance from divider to the end of the terminal
        let remainder = usize::from(w) - max_id_width_temp - 2;

        Printer {
            data: read_data,
            max_id_width: max_id_width_temp,
            terminal_width: w,
            terminal_height: h,
            border_color: 240,
            div_to_end: remainder,
        }
    }

    fn print_header(&self, title: String) {
        println!(
            "{}{}{}",
            Fixed(self.border_color).paint("-".repeat(self.max_id_width + 1)),
            Fixed(self.border_color).paint("+"),
            Fixed(self.border_color).paint("-".repeat(self.div_to_end))
        );
        println!(
            "{}{}{}",
            " ".repeat(self.max_id_width + 1),
            Fixed(self.border_color).paint("| "),
            Purple.paint(title)
        );
        println!(
            "{}{}{}",
            Fixed(self.border_color).paint("-".repeat(self.max_id_width + 1)),
            Fixed(self.border_color).paint("+"),
            Fixed(self.border_color).paint("-".repeat(self.div_to_end))
        );
    }

    fn print_vec(&self, data_vector: Vec<(&i64, &BujoObject)>, title: String) {
        self.print_header(title);
        for c in data_vector.iter() {
            let num_blanks = self.max_id_width - c.0.to_string().len();
            println!(
                "{}{} {} {} {}",
                " ".repeat(num_blanks),
                c.0,
                Fixed(self.border_color).paint("|"),
                c.1.signifier,
                c.1.content
            );
        }
    }
    pub fn daily(&self){
        //Filter data where timestamp is equal to today's date
        let mut data_vec_filtered: Vec<(&i64, &BujoObject)> = self.data.content.iter().filter(|x|{
            let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(x.1.time_added, 0), Utc);
            Local::now().date()==dt.date()
        }).collect();

        //Sort data by timestamp
        data_vec_filtered.sort_by_key(|a| a.1.time_added);

        //Display
        self.print_vec(data_vec_filtered,String::from("Daily"));
    }

    pub fn all(&self) {
        //Sort data by timestamp
        let mut data_vec: Vec<(&i64, &BujoObject)> = self.data.content.iter().collect();
        data_vec.sort_by_key(|a| a.1.time_added);
        self.print_vec(data_vec,String::from("Bujo"));
    }
}
