extern crate calamine;
use calamine::{open_workbook, Xlsx};
use std::fs::File;
use std::io::BufReader;

pub fn create_file(input_file: &str) -> File {
    let file_handle = File::create(input_file).expect("Error creating output file");
    file_handle
}

pub fn open_file(output_file: &str) -> Xlsx<BufReader<File>> {
    let workbook: Xlsx<_> = open_workbook(output_file).expect("Error while opening the input file");
    workbook
}
