// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::Workbook;

mod common;

// Test case to demonstrate creating a basic file with some string cell data.
fn create_new_xlsx_file(filename: &str) {
    let mut workbook = Workbook::new(filename);
    let worksheet = workbook.add_worksheet();

    worksheet.write_string_only(0, 0, "Hello");
    worksheet.write_string_only(1, 0, "World");
    worksheet.write_string_only(2, 0, "Hello");
    worksheet.write_string_only(3, 0, "World");

    workbook.close();
}

#[test]
fn bootstrap07_write_repeated_strings() {
    let testcase = "bootstrap07";

    let (excel_file, xlsxwriter_file) = common::get_xlsx_filenames(testcase);
    create_new_xlsx_file(&xlsxwriter_file);
    common::assert_eq(&excel_file, &xlsxwriter_file);
    common::remove_test_xlsx_file(&xlsxwriter_file);
}