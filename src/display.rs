use crate::data::{BujoObject, Data};
use ansi_term::Colour::{Fixed, Purple};
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use terminal_size::{terminal_size, Height, Width};

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
    pub fn new(incoming_data: Data) -> Printer {
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

    fn print_subtasks(&self, parent_task: (&i64, &BujoObject), num_blanks: usize) {
        let mut subvec: Vec<(&i64, &BujoObject)> = parent_task
            .1
            .subtasks
            .iter()
            .filter(|x| {
                let dt = DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(x.1.current_date, 0),
                    Utc,
                );
                Local::now().date() == dt.date()
            })
            .collect();
        subvec.sort_by_key(|a| a.1.daily_id);
        for d in subvec {
            println!(
                "{}{} {} {} {} {}",
                " ".repeat(num_blanks),
                d.1.daily_id,
                Fixed(self.border_color).paint("|"),
                " ".repeat(parent_task.1.signifier.len()),
                d.1.signifier,
                d.1.content
            );
            if d.1.subtasks.len() > 0 {
                self.print_subtasks(d, num_blanks);
            }
        }
    }
    pub fn daily(&self) {
        //Filter data where timestamp is equal to today's date
        let mut data_vec_filtered: Vec<(&i64, &BujoObject)> = self
            .data
            .content
            .iter()
            .filter(|x| {
                let dt = DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(x.1.current_date, 0),
                    Utc,
                );
                Local::now().date() == dt.date()
            })
            .collect();

        //Sort data by timestamp
        data_vec_filtered.sort_by_key(|a| a.1.daily_id);

        //Display
        self.print_header(String::from("Daily"));
        for c in data_vec_filtered.iter() {
            let num_blanks = self.max_id_width - c.1.daily_id.to_string().len();
            println!(
                "{}{} {} {} {}",
                " ".repeat(num_blanks),
                c.1.daily_id,
                Fixed(self.border_color).paint("|"),
                c.1.signifier,
                c.1.content
            );
            
            //Recursion!!
            self.print_subtasks(*c,num_blanks);

            // let mut subvec: Vec<(&i64, &BujoObject)> =
            //     c.1.subtasks
            //         .iter()
            //         .filter(|x| {
            //             let dt = DateTime::<Utc>::from_utc(
            //                 NaiveDateTime::from_timestamp(x.1.current_date, 0),
            //                 Utc,
            //             );
            //             Local::now().date() == dt.date()
            //         })
            //         .collect();
            // subvec.sort_by_key(|a| a.1.daily_id);
            // for d in subvec {
            //     println!(
            //         "{}{} {} {} {} {}",
            //         " ".repeat(num_blanks),
            //         d.1.daily_id,
            //         Fixed(self.border_color).paint("|"),
            //         " ".repeat(c.1.signifier.len()),
            //         d.1.signifier,
            //         d.1.content
            //     );
            // }
        }
    }

    pub fn all(&self) {
        //Sort data by timestamp
        let mut data_vec: Vec<(&i64, &BujoObject)> = self.data.content.iter().collect();
        data_vec.sort_by_key(|a| a.1.current_date);
        self.print_vec(data_vec, String::from("Bujo"));
    }

    pub fn raw(&self) {
        self.print_header(String::from("Raw"));
        let mut data_vec: Vec<(&i64, &BujoObject)> = self.data.content.iter().collect();
        data_vec.sort_by_key(|a| a.1.current_date);
        for c in data_vec {
            println!("{:?}\n", c);
        }
    }
}
