// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2022, John McNamara, jmcnamara@cpan.org

//! The following example demonstrates setting a currency format for a worksheet
//! cell.

use rust_xlsxwriter::{Format, Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    // Create a new Excel file.
    let mut workbook = Workbook::new("currency_format.xlsx");

    // Add a worksheet.
    let worksheet = workbook.add_worksheet();

    // Add a format.
    let currency_format = Format::new().set_num_format("[$$-409]#,##0.00");

    worksheet.write_number(0, 0, 1234.56, &currency_format)?;

    workbook.close()?;

    Ok(())
}