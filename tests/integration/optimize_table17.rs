// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Table, TableColumn, TableFunction, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet_with_low_memory();

    for col_num in 1..=10u16 {
        worksheet.set_column_width(col_num, 10.288)?;
    }

    worksheet.write(0, 0, "Column1")?;
    worksheet.write(0, 1, "Column2")?;
    worksheet.write(0, 2, "Column3")?;
    worksheet.write(0, 3, "Column4")?;
    worksheet.write(0, 4, "Column5")?;
    worksheet.write(0, 5, "Column6")?;
    worksheet.write(0, 6, "Column7")?;
    worksheet.write(0, 7, "Column8")?;
    worksheet.write(0, 8, "Column9")?;
    worksheet.write(0, 9, "Column10")?;
    worksheet.write(0, 10, "Total")?;

    let columns = vec![
        TableColumn::new().set_total_label("Total"),
        TableColumn::default(),
        TableColumn::new().set_total_function(TableFunction::Average),
        TableColumn::new().set_total_function(TableFunction::Count),
        TableColumn::new().set_total_function(TableFunction::CountNumbers),
        TableColumn::new().set_total_function(TableFunction::Max),
        TableColumn::new().set_total_function(TableFunction::Min),
        TableColumn::new().set_total_function(TableFunction::Sum),
        TableColumn::new().set_total_function(TableFunction::StdDev),
        TableColumn::new().set_total_function(TableFunction::Var),
    ];

    let table = Table::new().set_columns(&columns).set_total_row(true);

    worksheet.add_table(2, 1, 5, 10, &table)?;

    worksheet.write(3, 1, 0)?;
    worksheet.write(3, 2, 0)?;
    worksheet.write(3, 3, 0)?;
    worksheet.write(3, 6, 4)?;
    worksheet.write(3, 7, 0)?;
    worksheet.write(3, 8, 1)?;
    worksheet.write(3, 9, 0)?;
    worksheet.write(3, 10, 0)?;
    worksheet.write(4, 1, 0)?;
    worksheet.write(4, 2, 0)?;
    worksheet.write(4, 3, 0)?;
    worksheet.write(4, 6, 5)?;
    worksheet.write(4, 7, 0)?;
    worksheet.write(4, 8, 2)?;
    worksheet.write(4, 9, 0)?;
    worksheet.write(4, 10, 0)?;

    // Overwrite a formula with the same formula to move the current row to 5.
    worksheet.write_formula(5, 6, "SUBTOTAL(104,[Column6])")?;

    // Set formula values.
    worksheet.set_formula_result(0, 6, "5");
    worksheet.set_formula_result(0, 8, "3");

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize_table17() {
    let test_runner = common::TestRunner::new()
        .set_name("table17")
        .set_function(create_new_xlsx_file)
        .ignore_calc_chain()
        .unique("optimize")
        .ignore_worksheet_spans()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}