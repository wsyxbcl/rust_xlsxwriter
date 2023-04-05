// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Chart, ChartSeries, ChartType, Workbook, XlsxError};

mod common;

// Test to demonstrate charts.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    let data = [[1, 2, 3], [2, 4, 6], [3, 6, 9], [4, 8, 12], [5, 10, 15]];
    for (row_num, row_data) in data.iter().enumerate() {
        for (col_num, col_data) in row_data.iter().enumerate() {
            worksheet.write_number(row_num as u32, col_num as u16, *col_data)?;
        }
    }

    let mut chart = Chart::new(ChartType::Column);
    chart
        .push_series(ChartSeries::new().set_values(("Sheet1", 0, 0, 4, 0)))
        .push_series(ChartSeries::new().set_values(("Sheet1", 0, 1, 4, 1)))
        .push_series(ChartSeries::new().set_values(("Sheet1", 0, 2, 4, 2)));

    chart.x_axis().set_name("XXX");
    chart.y_axis().set_name("YYY");

    // Set the chart axis ids to match the random values in the Excel file.
    chart.set_axis_ids(43704320, 43706624);

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_axis02() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_axis02")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
