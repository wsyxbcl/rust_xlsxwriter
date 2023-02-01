// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{HeaderImagePosition, Image, Workbook, XlsxError};

mod common;

// Test to demonstrate adding header/footer images to worksheets.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let image = Image::new("tests/input/images/red.jpg")?;

    let worksheet1 = workbook.add_worksheet();
    worksheet1.set_header("&L&G");
    worksheet1.set_header_image(&image, HeaderImagePosition::Left)?;

    let worksheet2 = workbook.add_worksheet();
    worksheet2.set_header("&L&G");
    worksheet2.set_header_image(&image, HeaderImagePosition::Left)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_header_image15() {
    let test_runner = common::TestRunner::new()
        .set_name("header_image15")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
