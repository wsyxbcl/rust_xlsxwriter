// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{
    Chart, ChartFormat, ChartLine, ChartLineDashType, ChartPoint, ChartSolidFill, ChartType,
    Workbook, XlsxError,
};

mod common;

// Test to demonstrate charts.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    let data = [[2, 5, 4, 1, 7, 4]];
    for (col_num, col_data) in data.iter().enumerate() {
        for (row_num, row_data) in col_data.iter().enumerate() {
            worksheet.write_number(row_num as u32, col_num as u16, *row_data)?;
        }
    }

    let points = vec![
        ChartPoint::default(),
        ChartPoint::new().set_format(
            ChartFormat::new().set_border(
                ChartLine::new()
                    .set_color("#FF0000")
                    .set_dash_type(ChartLineDashType::SquareDot),
            ),
        ),
        ChartPoint::default(),
        ChartPoint::new().set_format(
            ChartFormat::new().set_solid_fill(ChartSolidFill::new().set_color("#FFFF00")),
        ),
    ];

    let mut chart = Chart::new(ChartType::Pie);
    chart
        .add_series()
        .set_values("=Sheet1!$A$1:$A$6")
        .set_points(&points);

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_points02() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_points02")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
