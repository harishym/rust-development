extern crate calamine;
use crate::file_writer;
use calamine::{Reader, Xlsx};
use std::fs::File;
use std::io::BufReader;
use std::string::String;
pub struct InputData {
    pub rate_index: String,
    pub combo: String,
    pub term_slab: String,
}

pub struct OutputData {
    pub from_months: i32,
    pub from_year: i32,
    pub from_days: String,
    pub to_year: i32,
    pub to_months: i32,
    pub to_days: String,
    pub id: i32,
    pub from_amt: String,
    pub to_amt: String,
    pub rate_index: String,
}

pub struct Period {
    from_year: i32,
    from_months: i32,
    from_days: String,
    to_year: i32,
    to_months: i32,
    to_days: String,
}
pub fn operation(workbook: &mut Xlsx<BufReader<File>>, mut out_file: &mut File) {
    let mut id = 1;

    if let Some(Ok(reader)) = workbook.worksheet_range("Sheet1") {
        for column in reader.rows().skip(1) {
            let excel = InputData {
                rate_index: column[1].to_string(),
                combo: column[8].to_string(),
                term_slab: column[6].to_string(),
            };
            let combo_contents: Vec<&str> = excel.combo.split("-").collect();
            let period = get_period(&excel);
            let out = OutputData {
                from_year: period.from_year,
                from_months: period.from_months,
                from_days: period.from_days,
                to_year: period.to_year,
                to_months: period.to_months,
                to_days: period.to_days,
                from_amt: combo_contents[0].to_string(),
                to_amt: combo_contents[1].to_string(),
                id: id,
                rate_index: excel.rate_index,
            };
            file_writer::write_file(out, &mut out_file);
            id = id + 1;
        }
    }
}

pub fn get_period(excel: &InputData) -> Period {
    let mut term_slab_contents: Vec<&str> = excel.term_slab.split(" ").collect();
    let from_months: i32 = term_slab_contents[0]
        .parse()
        .expect("Error while parsing months to integer");
    let from_year = from_months / 12;
    let from_months = from_months % 12;
    let to_year;
    let mut to_months;
    let to_days;
    let mut month = String::new();
    term_slab_contents.push("mon");
    if term_slab_contents[4].to_uppercase() != "MONTHS" {
        to_year = 10;
        to_months = 0;
        to_days = String::from("0");
    } else {
        for i in 0..(term_slab_contents[3].len()) {
            if term_slab_contents[3]
                .chars()
                .nth(i)
                .expect("Error while seperating month from alphanumeric")
                .is_numeric()
            {
                month.push(
                    term_slab_contents[3]
                        .chars()
                        .nth(i)
                        .expect("Error while pushing the seperated alphanumeric"),
                );
            }
        }
        to_months = (&month)
            .parse()
            .expect("Error while parsing months to integer");
        to_days = term_slab_contents[5].to_string();
        to_year = to_months / 12;
        to_months = to_months % 12;
    }

    Period {
        from_year: from_year,
        from_months: from_months,
        from_days: term_slab_contents[2].to_string(),
        to_year: to_year,
        to_months: to_months,
        to_days: to_days,
    }
}
