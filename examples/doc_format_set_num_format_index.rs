// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2022, John McNamara, jmcnamara@cpan.org

//! The following example demonstrates setting one of the inbuilt format indices
//! for a format.

use rust_xlsxwriter::{Format, Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    // Create a new Excel file.
    let mut workbook = Workbook::new("formats.xlsx");

    // Add a worksheet.
    let worksheet = workbook.add_worksheet();

    let format = Format::new().set_num_format_index(15);

    worksheet.write_number(0, 0, 44927.521, &format)?;

    workbook.close()?;

    Ok(())
}