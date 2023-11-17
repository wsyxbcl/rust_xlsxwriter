// image - A module to represent Excel conditional formats.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

//! # Working with Conditional Formats
//!
//! Conditional formatting is a feature of Excel which allows you to apply a
//! format to a cell or a range of cells based on certain criteria. For example
//! you might apply rules like the following to highlight cells in different
//! ranges.
//!
//! <img
//! src="https://rustxlsxwriter.github.io/images/conditional_format_dialog.png">
//!
//! With `rust_xlsxwriter` we could emulate this by creating [`Format`]
//! instances and applying them to conditional format rules, like this:
//!
//! ```
//! # // This code is available in examples/doc_conditional_format_cell1.rs
//! #
//! # use rust_xlsxwriter::{
//! #     ConditionalFormatCell, ConditionalFormatCellCriteria, Format, Workbook, XlsxError,
//! # };
//! #
//! # fn main() -> Result<(), XlsxError> {
//! #     // Create a new Excel file object.
//! #     let mut workbook = Workbook::new();
//! #     let worksheet = workbook.add_worksheet();
//! #
//! #     // Add some sample data.
//! #     let data = [
//! #         [90, 80, 50, 10, 20, 90, 40, 90, 30, 40],
//! #         [20, 10, 90, 100, 30, 60, 70, 60, 50, 90],
//! #         [10, 50, 60, 50, 20, 50, 80, 30, 40, 60],
//! #         [10, 90, 20, 40, 10, 40, 50, 70, 90, 50],
//! #         [70, 100, 10, 90, 10, 10, 20, 100, 100, 40],
//! #         [20, 60, 10, 100, 30, 10, 20, 60, 100, 10],
//! #         [10, 60, 10, 80, 100, 80, 30, 30, 70, 40],
//! #         [30, 90, 60, 10, 10, 100, 40, 40, 30, 40],
//! #         [80, 90, 10, 20, 20, 50, 80, 20, 60, 90],
//! #         [60, 80, 30, 30, 10, 50, 80, 60, 50, 30],
//! #     ];
//! #     worksheet.write_row_matrix(2, 1, data)?;
//! #
//! #     // Set the column widths for clarity.
//! #     for col_num in 1..=10u16 {
//! #         worksheet.set_column_width(col_num, 6)?;
//! #     }
//! #
//!     // Add a format. Light red fill with dark red text.
//!     let format1 = Format::new()
//!         .set_font_color("9C0006")
//!         .set_background_color("FFC7CE");
//!
//!     // Add a format. Green fill with dark green text.
//!     let format2 = Format::new()
//!         .set_font_color("006100")
//!         .set_background_color("C6EFCE");
//!
//!     // Write a conditional format over a range.
//!     let conditional_format = ConditionalFormatCell::new()
//!         .set_criteria(ConditionalFormatCellCriteria::GreaterThanOrEqualTo)
//!         .set_value(50)
//!         .set_format(format1);
//!
//!     worksheet.add_conditional_format(2, 1, 11, 10, &conditional_format)?;
//!
//!     // Write another conditional format over the same range.
//!     let conditional_format = ConditionalFormatCell::new()
//!         .set_criteria(ConditionalFormatCellCriteria::LessThan)
//!         .set_value(50)
//!         .set_format(format2);
//!
//!     worksheet.add_conditional_format(2, 1, 11, 10, &conditional_format)?;
//!
//! #     // Save the file.
//! #     workbook.save("conditional_format.xlsx")?;
//! #
//! #     Ok(())
//! # }
//! ```
//!
//! Which would produce an output like the following. Cells with values >= 50
//! are in light red. Values < 50 are in light green.
//!
//! <img
//! src="https://rustxlsxwriter.github.io/images/conditional_format_cell1.png">
//!
//!
//!
//!
//! # Replicating an Excel conditional format with `rust_xlsxwriter`
//!
//! It is important not to try to reverse engineer Excel's conditional
//! formatting rules from `rust_xlsxwriter`. If you aren't familiar with the
//! syntax and functionality of conditional formats then a better place to start
//! is in Excel. Create a conditional format in Excel to meet your needs and
//! then port it over to `rust_xlsxwriter`.
//!
//! There are several common features of all conditional formats:
//!
//! - A range: The range that the conditional format applies to. This is usually
//!   set via the
//!   [`Worksheet::add_conditional_format()`](crate::Worksheet::add_conditional_format)
//!   method.
//! - A rule: This can be an equality like `>=` or a rule like "Top 10".
//! - A target: This is usually a cell or range that the rule applies to. This
//!   mainly applies to Cell style conditional formats. For other types of
//!   conditional format the "range" is the target.
//! - A format: The cell format with properties such as text or background color
//!   to high the cell if the rule matches.
//!
//! The following are the structs that represent the main conditional format
//! variants in Excel. See each of these sections for more information:
//!
//! - [`ConditionalFormatCell`]: The Cell style conditional format. This is the
//!   most common style of conditional formats which uses simple equalities such
//!   as "equal to" or "greater than" or "between". See the example above.
//! - [`ConditionalFormatAverage`]: The Average/Standard Deviation style
//!   conditional format.
//! - [`ConditionalFormatFormula`]: The Formula style conditional format.
//! - [`ConditionalFormatBlank`]: The Blank/Non-blank style conditional format.
//! - [`ConditionalFormatError`]: The Error/Non-error style conditional format.
//! - [`ConditionalFormatDuplicate`]: The Duplicate/Unique style conditional
//!   format.
//! - [`ConditionalFormatDate`]: The Dates Occurring style conditional format.
//! - [`ConditionalFormatText`]: The Text conditional format for rules like
//!   "contains" or "begins with".
//! - [`ConditionalFormatTop`]: The Top/Bottom style conditional format.
//! - [`ConditionalFormat2ColorScale`]: The 2 color scale style conditional
//!   format.
//! - [`ConditionalFormat3ColorScale`]: The 3 color scale style conditional
//!   format.
//! - [`ConditionalFormatDataBar`]: The Data Bar scale style conditional format.
//!
//!
//!
//!
//! # Excel's limitations on conditional format properties
//!
//! It is important to note that not all of Excel's cell format properties can
//! be modified with a conditional format.
//!
//! For example the view below of the Excel conditional format dialog shows the
//! limited number of font properties that can be set. The available properties
//! are highlighted with green.
//!
//! <img
//! src="https://rustxlsxwriter.github.io/images/conditional_format_limitations.png">
//!
//! Properties that **cannot** be modified in a conditional format are font
//! name, font size, superscript and subscript, diagonal borders, all alignment
//! properties and all protection properties.
//!
//!
//!
//!
//! # Selecting a non-contiguous range
//!
//! In Excel it is possible to select several non-contiguous cells or ranges
//! like `"B3:D6 I3:K6 B9:D12 I9:K12"` and apply a conditional format to them.
//!
//! It is possible to achieve a similar effect with `rust_xlsxwriter` by using
//! repeated calls to
//! [`Worksheet::add_conditional_format()`](crate::Worksheet::add_conditional_format).
//! However this approach results in multiple identical conditional formats
//! applied to different cell ranges rather than one conditional format applied
//! to a multiple range selection.
//!
//! If this distinction is important to you then you can get the Excel multiple
//! selection effect using the `set_multi_range()` which is provided for all the
//! `ConditionalFormat` types. See the example below and note that the cells
//! outside the selected ranges do not have any conditional formatting.
//!
//!
//! Note, you can use an Excel range like
//! `"$B$3:$D$6,$I$3:$K$6,$B$9:$D$12,$I$9:$K$12"` or omit the `$` anchors and
//! replace the commas with spaces to have a clearer range like `"B3:D6 I3:K6
//! B9:D12 I9:K12"`. The documentation and examples use the latter format for
//! clarity but it you are copying and pasting from Excel you can use the first
//! format.
//!
//! ```
//! # // This code is available in examples/doc_conditional_format_multi_range.rs
//! #
//! # use rust_xlsxwriter::{
//! #     ConditionalFormatCell, ConditionalFormatCellCriteria, Format, Workbook, XlsxError,
//! # };
//! #
//! # fn main() -> Result<(), XlsxError> {
//! #     // Create a new Excel file object.
//! #     let mut workbook = Workbook::new();
//! #     let worksheet = workbook.add_worksheet();
//! #
//! #     // Add some sample data.
//! #     let data = [
//! #         [90, 80, 50, 10, 20, 90, 40, 90, 30, 40],
//! #         [20, 10, 90, 100, 30, 60, 70, 60, 50, 90],
//! #         [10, 50, 60, 50, 20, 50, 80, 30, 40, 60],
//! #         [10, 90, 20, 40, 10, 40, 50, 70, 90, 50],
//! #         [70, 100, 10, 90, 10, 10, 20, 100, 100, 40],
//! #         [20, 60, 10, 100, 30, 10, 20, 60, 100, 10],
//! #         [10, 60, 10, 80, 100, 80, 30, 30, 70, 40],
//! #         [30, 90, 60, 10, 10, 100, 40, 40, 30, 40],
//! #         [80, 90, 10, 20, 20, 50, 80, 20, 60, 90],
//! #         [60, 80, 30, 30, 10, 50, 80, 60, 50, 30],
//! #     ];
//! #     worksheet.write_row_matrix(2, 1, data)?;
//! #
//! #     // Set the column widths for clarity.
//! #     for col_num in 1..=10u16 {
//! #         worksheet.set_column_width(col_num, 6)?;
//! #     }
//! #
//! #     // Add a format. Light red fill with dark red text.
//! #     let format1 = Format::new()
//! #         .set_font_color("9C0006")
//! #         .set_background_color("FFC7CE");
//! #
//! #     // Add a format. Green fill with dark green text.
//! #     let format2 = Format::new()
//! #         .set_font_color("006100")
//! #         .set_background_color("C6EFCE");
//! #
//!     // Write a conditional format over a non-contiguous range.
//!     let conditional_format = ConditionalFormatCell::new()
//!         .set_criteria(ConditionalFormatCellCriteria::GreaterThanOrEqualTo)
//!         .set_value(50)
//!         .set_multi_range("B3:D6 I3:K6 B9:D12 I9:K12")
//!         .set_format(format1);
//!
//!     worksheet.add_conditional_format(2, 1, 11, 10, &conditional_format)?;
//!
//!     // Write another conditional format over the same range.
//!     let conditional_format = ConditionalFormatCell::new()
//!         .set_criteria(ConditionalFormatCellCriteria::LessThan)
//!         .set_value(50)
//!         .set_multi_range("B3:D6 I3:K6 B9:D12 I9:K12")
//!         .set_format(format2);
//!
//!     worksheet.add_conditional_format(2, 1, 11, 10, &conditional_format)?;
//!
//! #     // Save the file.
//! #     workbook.save("conditional_format.xlsx")?;
//! #
//! #     Ok(())
//! # }
//! ```
//!
//! This creates conditional format rules like this:
//!
//! <img
//! src="https://rustxlsxwriter.github.io/images/conditional_format_multi_range_rules.png">
//!
//! And the following output file:
//!
//! <img
//! src="https://rustxlsxwriter.github.io/images/conditional_format_multi_range.png">
//!
//!
//!
//!
//! # Relative and absolute references in conditional formats
//!
//! When dealing with Excel conditional formats it is important to distinguish
//! between relative and absolute cell references in Excel.
//!
//! Relative cell references change when they are copied while absolute
//! references maintain fixed row and column references. In Excel absolute
//! references are prefixed by the dollar symbol like `$A$1`. See the Microsoft
//! Office documentation for more information on [relative and absolute
//! references].
//!
//! [relative and absolute references]:
//!     https://support.microsoft.com/en-us/office/switch-between-relative-absolute-and-mixed-references-dfec08cd-ae65-4f56-839e-5f0d8d0baca9
//!
//! In conditional formats each of these four variants has a different effect on
//! how the conditional format is applied:
//!
//! - `A1`: Column and row are relative. The conditional format rule is applied
//!   to each cell in the conditional format range.
//! - `$A1`: Column is absolute and row is relative. The conditional format rule
//!   is applied to each row based on the first cell in the row.
//! - `A$1`: Column is relative and row is absolute. The conditional format rule
//!   is applied to each column based on the first cell in the column.
//! - `$A$1`: Column and row are absolute. The conditional format rule is
//!   applied to the entire range based on the first cell in the range.
//!
//! The effect is shown in the following example that highlights cells with even
//! numbered values:
//!
//!
//! ```
//! # // This code is available in examples/doc_conditional_format_anchor.rs
//! #
//! # use rust_xlsxwriter::{ConditionalFormatFormula, Format, Workbook, XlsxError};
//! #
//! # fn main() -> Result<(), XlsxError> {
//! #     // Create a new Excel file object.
//! #     let mut workbook = Workbook::new();
//! #
//! #     // Add a format. Green fill with dark green text.
//! #     let format = Format::new()
//! #         .set_font_color("006100")
//! #         .set_background_color("C6EFCE");
//! #
//! #     // Add some sample data.
//! #     let data = [
//! #         [34, 73, 39, 32, 75, 48, 75, 66],
//! #         [5, 24, 1, 84, 54, 62, 60, 3],
//! #         [28, 79, 97, 13, 85, 93, 93, 22],
//! #         [27, 71, 40, 17, 18, 79, 90, 93],
//! #         [88, 25, 33, 23, 67, 1, 59, 79],
//! #         [23, 100, 20, 88, 29, 33, 38, 54],
//! #         [7, 57, 88, 28, 10, 26, 37, 7],
//! #         [53, 78, 1, 96, 26, 45, 47, 33],
//! #         [60, 54, 81, 66, 81, 90, 80, 93],
//! #         [70, 5, 46, 14, 71, 19, 66, 36],
//! #     ];
//! #
//! #     // Add a new worksheet and write the sample data.
//! #     let worksheet = workbook.add_worksheet();
//! #     worksheet.write_row_matrix(2, 1, data)?;
//! #
//!     // The rule is applied to each cell in the range.
//!     let conditional_format = ConditionalFormatFormula::new()
//!         .set_value("=ISEVEN(B3)")
//!         .set_format(&format);
//!
//! #     worksheet.add_conditional_format(2, 1, 11, 8, &conditional_format)?;
//! #
//! #     // Add a new worksheet and write the sample data.
//! #     let worksheet = workbook.add_worksheet();
//! #     worksheet.write_row_matrix(2, 1, data)?;
//! #
//!     // The rule is applied to each row based on the first row in the column.
//!     let conditional_format = ConditionalFormatFormula::new()
//!         .set_value("=ISEVEN($B3)")
//!         .set_format(&format);
//!
//! #     worksheet.add_conditional_format(2, 1, 11, 8, &conditional_format)?;
//! #
//! #     // Add a new worksheet and write the sample data.
//! #     let worksheet = workbook.add_worksheet();
//! #     worksheet.write_row_matrix(2, 1, data)?;
//! #
//!     // The rule is applied to each column based on the first cell in the column.
//!     let conditional_format = ConditionalFormatFormula::new()
//!         .set_value("=ISEVEN(B$3)")
//!         .set_format(&format);
//!
//! #     worksheet.add_conditional_format(2, 1, 11, 8, &conditional_format)?;
//! #
//! #     // Add a new worksheet and write the sample data.
//! #     let worksheet = workbook.add_worksheet();
//! #     worksheet.write_row_matrix(2, 1, data)?;
//! #
//!     // The rule is applied to the entire range based on the first cell in the range.
//!     let conditional_format = ConditionalFormatFormula::new()
//!         .set_value("=ISEVEN($B$3)")
//!         .set_format(format);
//!
//! #     worksheet.add_conditional_format(2, 1, 11, 8, &conditional_format)?;
//! #
//! #     // Save the file.
//! #     workbook.save("conditional_format.xlsx")?;
//! #
//! #     Ok(())
//! # }
//! ```
//!
//! Output for the formula `=ISEVEN(B3)`:
//!
//! <img
//! src="https://rustxlsxwriter.github.io/images/conditional_format_anchor1.png">
//!
//! Output for the formula `=ISEVEN($B3)`:
//!
//! <img
//! src="https://rustxlsxwriter.github.io/images/conditional_format_anchor2.png">
//!
//! Output for the formula `=ISEVEN(B$3)`:
//!
//! <img
//! src="https://rustxlsxwriter.github.io/images/conditional_format_anchor3.png">
//!
//! Output for the formula `=ISEVEN($B$3)`:
//!
//! <img
//! src="https://rustxlsxwriter.github.io/images/conditional_format_anchor4.png">
//!
#![warn(missing_docs)]

mod tests;

#[cfg(feature = "chrono")]
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::{xmlwriter::XMLWriter, Color, ExcelDateTime, Format, Formula, IntoColor, XlsxError};
use std::{borrow::Cow, fmt};

// -----------------------------------------------------------------------
// ConditionalFormat trait
// -----------------------------------------------------------------------

/// Trait for generic conditional format types.
///
pub trait ConditionalFormat {
    /// Validate the conditional format.
    ///
    /// # Errors
    ///
    /// * [`XlsxError::ConditionalFormatError`] - A general error that is raised
    ///   when a conditional formatting parameter is incorrect or missing.
    ///
    fn validate(&self) -> Result<(), XlsxError>;

    /// Return the conditional format rule as an XML string.
    fn rule(&self, dxf_index: Option<u32>, priority: u32, range: &str, guid: &str) -> String;

    /// Return the extended x14 conditional format rule as an XML string.
    fn x14_rule(&self, guid: &str) -> String;

    /// Get a mutable reference to the format object in the conditional format.
    fn format_as_mut(&mut self) -> Option<&mut Format>;

    /// Get the index of the format object in the conditional format.
    fn format_index(&self) -> Option<u32>;

    /// Get the multi-cell range for the conditional format, if present.
    fn multi_range(&self) -> String;

    /// Check if the conditional format uses Excel 2010+ extensions.
    fn has_x14_extensions(&self) -> bool;

    /// Clone a reference into a concrete Box type.
    fn box_clone(&self) -> Box<dyn ConditionalFormat + Send>;
}

macro_rules! generate_conditional_format_impls {
    ($($t:ty)*) => ($(
        impl ConditionalFormat for $t {
            fn validate(&self) -> Result<(), XlsxError> {
                self.validate()
            }

            fn rule(&self, dxf_index: Option<u32>, priority: u32, range: &str, guid: &str) -> String {
                self.rule(dxf_index, priority, range, guid)
            }

            fn x14_rule(&self, guid: &str) -> String {
                self.x14_rule(guid)
            }


            fn format_as_mut(&mut self) -> Option<&mut Format> {
                self.format_as_mut()
            }

            fn format_index(&self) -> Option<u32> {
                self.format_index()
            }

            fn multi_range(&self) -> String {
                self.multi_range()
            }

            fn has_x14_extensions(&self) -> bool {
                self.has_x14_extensions()
            }

            fn box_clone(&self) -> Box<dyn ConditionalFormat + Send> {
                Box::new(self.clone())
            }
        }
    )*)
}
generate_conditional_format_impls!(
    ConditionalFormatAverage
    ConditionalFormatBlank
    ConditionalFormatCell
    ConditionalFormatDate
    ConditionalFormatDuplicate
    ConditionalFormatError
    ConditionalFormatFormula
    ConditionalFormatText
    ConditionalFormatTop
    ConditionalFormat2ColorScale
    ConditionalFormat3ColorScale
    ConditionalFormatDataBar
);

// -----------------------------------------------------------------------
// ConditionalFormatCell
// -----------------------------------------------------------------------

/// The `ConditionalFormatCell` struct represents a Cell conditional format.
///
/// `ConditionalFormatCell` is used to represent a Cell style conditional format
/// in Excel. Cell conditional formats use simple equalities such as "equal to"
/// or "greater than" or "between".
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_cell_intro.png">
///
/// For more information see [Working with Conditional Formats](crate::conditional_format).
///
/// # Examples
///
/// Example of adding a cell type conditional formatting to a worksheet. Cells
/// with values >= 50 are in light red. Values < 50 are in light green.
///
/// ```
/// # // This code is available in examples/doc_conditional_format_cell1.rs
/// #
/// # use rust_xlsxwriter::{
/// #     ConditionalFormatCell, ConditionalFormatCellCriteria, Format, Workbook, XlsxError,
/// # };
/// #
/// # fn main() -> Result<(), XlsxError> {
/// #     // Create a new Excel file object.
/// #     let mut workbook = Workbook::new();
/// #     let worksheet = workbook.add_worksheet();
/// #
/// #     // Add some sample data.
/// #     let data = [
/// #         [90, 80, 50, 10, 20, 90, 40, 90, 30, 40],
/// #         [20, 10, 90, 100, 30, 60, 70, 60, 50, 90],
/// #         [10, 50, 60, 50, 20, 50, 80, 30, 40, 60],
/// #         [10, 90, 20, 40, 10, 40, 50, 70, 90, 50],
/// #         [70, 100, 10, 90, 10, 10, 20, 100, 100, 40],
/// #         [20, 60, 10, 100, 30, 10, 20, 60, 100, 10],
/// #         [10, 60, 10, 80, 100, 80, 30, 30, 70, 40],
/// #         [30, 90, 60, 10, 10, 100, 40, 40, 30, 40],
/// #         [80, 90, 10, 20, 20, 50, 80, 20, 60, 90],
/// #         [60, 80, 30, 30, 10, 50, 80, 60, 50, 30],
/// #     ];
/// #     worksheet.write_row_matrix(2, 1, data)?;
/// #
/// #     // Set the column widths for clarity.
/// #     for col_num in 1..=10u16 {
/// #         worksheet.set_column_width(col_num, 6)?;
/// #     }
/// #
/// #     // Add a format. Light red fill with dark red text.
/// #     let format1 = Format::new()
/// #         .set_font_color("9C0006")
/// #         .set_background_color("FFC7CE");
/// #
/// #     // Add a format. Green fill with dark green text.
/// #     let format2 = Format::new()
/// #         .set_font_color("006100")
/// #         .set_background_color("C6EFCE");
/// #
///     // Write a conditional format over a range.
///     let conditional_format = ConditionalFormatCell::new()
///         .set_criteria(ConditionalFormatCellCriteria::GreaterThanOrEqualTo)
///         .set_value(50)
///         .set_format(format1);
///
///     worksheet.add_conditional_format(2, 1, 11, 10, &conditional_format)?;
///
///     // Write another conditional format over the same range.
///     let conditional_format = ConditionalFormatCell::new()
///         .set_criteria(ConditionalFormatCellCriteria::LessThan)
///         .set_value(50)
///         .set_format(format2);
///
///     worksheet.add_conditional_format(2, 1, 11, 10, &conditional_format)?;
///
/// #     // Save the file.
/// #     workbook.save("conditional_format.xlsx")?;
/// #
/// #     Ok(())
/// # }
/// ```
///
/// This creates conditional format rules like this:
///
/// <img src="https://rustxlsxwriter.github.io/images/conditional_format_cell1_rules.png">
///
/// And the following output file:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_cell1.png">
///
///
/// Example of adding a cell type conditional formatting to a worksheet. Values
/// between 30 and 70 are highlighted in light red. Values outside that range
/// are in light green.
///
/// ```
/// # // This code is available in examples/doc_conditional_format_cell2.rs
/// #
/// # use rust_xlsxwriter::{
/// #     ConditionalFormatCell, ConditionalFormatCellCriteria, Format, Workbook, XlsxError,
/// # };
/// #
/// # fn main() -> Result<(), XlsxError> {
/// #     // Create a new Excel file object.
/// #     let mut workbook = Workbook::new();
/// #     let worksheet = workbook.add_worksheet();
/// #
/// #     // Add some sample data.
/// #     let data = [
/// #         [90, 80, 50, 10, 20, 90, 40, 90, 30, 40],
/// #         [20, 10, 90, 100, 30, 60, 70, 60, 50, 90],
/// #         [10, 50, 60, 50, 20, 50, 80, 30, 40, 60],
/// #         [10, 90, 20, 40, 10, 40, 50, 70, 90, 50],
/// #         [70, 100, 10, 90, 10, 10, 20, 100, 100, 40],
/// #         [20, 60, 10, 100, 30, 10, 20, 60, 100, 10],
/// #         [10, 60, 10, 80, 100, 80, 30, 30, 70, 40],
/// #         [30, 90, 60, 10, 10, 100, 40, 40, 30, 40],
/// #         [80, 90, 10, 20, 20, 50, 80, 20, 60, 90],
/// #         [60, 80, 30, 30, 10, 50, 80, 60, 50, 30],
/// #     ];
/// #     worksheet.write_row_matrix(2, 1, data)?;
/// #
/// #     // Set the column widths for clarity.
/// #     for col_num in 1..=10u16 {
/// #         worksheet.set_column_width(col_num, 6)?;
/// #     }
/// #
/// #     // Add a format. Light red fill with dark red text.
/// #     let format1 = Format::new()
/// #         .set_font_color("9C0006")
/// #         .set_background_color("FFC7CE");
/// #
/// #     // Add a format. Green fill with dark green text.
/// #     let format2 = Format::new()
/// #         .set_font_color("006100")
/// #         .set_background_color("C6EFCE");
/// #
///     // Write a conditional format over a range.
///     let conditional_format = ConditionalFormatCell::new()
///         .set_criteria(ConditionalFormatCellCriteria::Between)
///         .set_minimum(30)
///         .set_maximum(70)
///         .set_format(format1);
///
///     worksheet.add_conditional_format(2, 1, 11, 10, &conditional_format)?;
///
///     // Write another conditional format over the same range.
///     let conditional_format = ConditionalFormatCell::new()
///         .set_criteria(ConditionalFormatCellCriteria::NotBetween)
///         .set_minimum(30)
///         .set_maximum(70)
///         .set_format(format2);
///
///     worksheet.add_conditional_format(2, 1, 11, 10, &conditional_format)?;
///
/// #     // Save the file.
/// #     workbook.save("conditional_format.xlsx")?;
/// #
/// #     Ok(())
/// # }
/// ```
///
/// This creates conditional format rules like this:
///
/// <img src="https://rustxlsxwriter.github.io/images/conditional_format_cell2_rules.png">
///
/// And the following output file:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_cell2.png">
///
#[derive(Clone)]
pub struct ConditionalFormatCell {
    minimum: ConditionalFormatValue,
    maximum: ConditionalFormatValue,
    criteria: ConditionalFormatCellCriteria,
    multi_range: String,
    stop_if_true: bool,
    has_x14_extensions: bool,
    pub(crate) format: Option<Format>,
}

/// **Section 1**: The following methods are specific to `ConditionalFormatCell`.
impl ConditionalFormatCell {
    /// Create a new Cell conditional format struct.
    #[allow(clippy::new_without_default)]
    pub fn new() -> ConditionalFormatCell {
        ConditionalFormatCell {
            minimum: ConditionalFormatValue::new_from_string(""),
            maximum: ConditionalFormatValue::new_from_string(""),
            criteria: ConditionalFormatCellCriteria::None,
            multi_range: String::new(),
            stop_if_true: false,
            has_x14_extensions: false,
            format: None,
        }
    }

    /// Set the value of the Cell conditional format rule.
    ///
    /// # Parameters
    ///
    /// * `value` - Any type that can convert into a [`ConditionalFormatValue`]
    ///   which is effectively all types supported by Excel.
    ///
    /// # Examples
    ///
    /// Example of adding a cell type conditional formatting to a worksheet. Cells
    /// with values >= 50 are in light green.
    ///
    /// ```
    /// # // This code is available in examples/doc_conditional_format_cell_set_value.rs
    /// #
    /// # use rust_xlsxwriter::{
    /// #     ConditionalFormatCell, ConditionalFormatCellCriteria, Format, Workbook, XlsxError,
    /// # };
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #     let worksheet = workbook.add_worksheet();
    /// #
    /// #     // Add some sample data.
    /// #     let data = [10, 80, 50, 10, 20, 60, 40, 70, 30, 40];
    /// #
    /// #     worksheet.write_column(0, 0, data)?;
    /// #
    /// #     // Add a format. Green fill with dark green text.
    /// #     let format = Format::new()
    /// #         .set_font_color("006100")
    /// #         .set_background_color("C6EFCE");
    /// #
    ///     // Write a conditional format over a range.
    ///     let conditional_format = ConditionalFormatCell::new()
    ///         .set_criteria(ConditionalFormatCellCriteria::GreaterThanOrEqualTo)
    ///         .set_value(50)
    ///         .set_format(format);
    ///
    ///     worksheet.add_conditional_format(0, 0, 9, 0, &conditional_format)?;
    ///
    /// #     // Save the file.
    /// #     workbook.save("conditional_format.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img src="https://rustxlsxwriter.github.io/images/conditional_format_cell_set_value.png">
    ///
    pub fn set_value(self, value: impl Into<ConditionalFormatValue>) -> ConditionalFormatCell {
        self.set_minimum(value)
    }

    /// Set the minimum value of the Cell "between" and "not between"
    /// conditional format rules.
    ///
    /// # Parameters
    ///
    /// * `value` - Any type that can convert into a [`ConditionalFormatValue`]
    ///   which is effectively all types supported by Excel.
    ///
    /// # Examples
    ///
    /// Example of adding a cell type conditional formatting to a worksheet.
    /// Values between 40 and 60 are highlighted in light green.
    ///
    /// ```
    /// # // This code is available in examples/doc_conditional_format_cell_set_minimum.rs
    /// #
    /// # use rust_xlsxwriter::{
    /// #     ConditionalFormatCell, ConditionalFormatCellCriteria, Format, Workbook, XlsxError,
    /// # };
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #     let worksheet = workbook.add_worksheet();
    /// #
    /// #     // Add some sample data.
    /// #     let data = [10, 80, 50, 10, 20, 60, 40, 70, 30, 40];
    /// #
    /// #     worksheet.write_column(0, 0, data)?;
    /// #
    /// #     // Add a format. Green fill with dark green text.
    /// #     let format = Format::new()
    /// #         .set_font_color("006100")
    /// #         .set_background_color("C6EFCE");
    /// #
    ///     // Write a conditional format over a range.
    ///     let conditional_format = ConditionalFormatCell::new()
    ///         .set_criteria(ConditionalFormatCellCriteria::Between)
    ///         .set_minimum(40)
    ///         .set_maximum(60)
    ///         .set_format(format);
    ///
    ///     worksheet.add_conditional_format(0, 0, 9, 0, &conditional_format)?;
    ///
    /// #     // Save the file.
    /// #     workbook.save("conditional_format.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img
    /// src="https://rustxlsxwriter.github.io/images/conditional_format_cell_set_minimum.png">
    ///
    pub fn set_minimum(
        mut self,
        value: impl Into<ConditionalFormatValue>,
    ) -> ConditionalFormatCell {
        self.minimum = value.into();
        self.minimum.quote_string();
        self
    }

    /// Set the maximum value of the Cell "between" and "not between"
    /// conditional format rules.
    ///
    /// Set the example above.
    ///
    /// # Parameters
    ///
    /// * `value` - Any type that can convert into a [`ConditionalFormatValue`]
    ///   which is effectively all types supported by Excel.
    ///
    pub fn set_maximum(
        mut self,
        value: impl Into<ConditionalFormatValue>,
    ) -> ConditionalFormatCell {
        self.maximum = value.into();
        self.maximum.quote_string();
        self
    }

    /// Set the criteria for the conditional format rule such as `=`, `!=`, `>`,
    /// `<`, `>=`, `<=`, `between` or `not between`.
    ///
    /// # Parameters
    ///
    /// * `criteria` - A [`ConditionalFormatCellCriteria`] enum value.
    ///
    pub fn set_criteria(
        mut self,
        criteria: ConditionalFormatCellCriteria,
    ) -> ConditionalFormatCell {
        self.criteria = criteria;
        self
    }

    // Validate the conditional format.
    pub(crate) fn validate(&self) -> Result<(), XlsxError> {
        let error_message = match self.criteria {
            ConditionalFormatCellCriteria::None => "'criteria' must be set".to_string(),
            ConditionalFormatCellCriteria::Between | ConditionalFormatCellCriteria::NotBetween => {
                if self.minimum.value.is_empty() {
                    "'minimum' value must be set".to_string()
                } else if self.maximum.value.is_empty() {
                    "'maximum' value must be set".to_string()
                } else {
                    String::new()
                }
            }
            _ => {
                if self.minimum.value.is_empty() {
                    "'value' must be set".to_string()
                } else {
                    String::new()
                }
            }
        };

        if !error_message.is_empty() {
            return Err(XlsxError::ConditionalFormatError(error_message));
        }

        Ok(())
    }

    //  Return the conditional format rule as an XML string.
    pub(crate) fn rule(
        &self,
        dxf_index: Option<u32>,
        priority: u32,
        _range: &str,
        _guid: &str,
    ) -> String {
        let mut writer = XMLWriter::new();
        let mut attributes = vec![("type", "cellIs".to_string())];

        // Set the format index if present.
        if let Some(dxf_index) = dxf_index {
            attributes.push(("dxfId", dxf_index.to_string()));
        }

        // Set the rule priority order.
        attributes.push(("priority", priority.to_string()));

        // Set the "Stop if True" property.
        if self.stop_if_true {
            attributes.push(("stopIfTrue", "1".to_string()));
        }

        attributes.push(("operator", self.criteria.to_string()));

        // Write the rule.
        writer.xml_start_tag("cfRule", &attributes);
        writer.xml_data_element_only("formula", &self.minimum.value);

        if self.criteria == ConditionalFormatCellCriteria::Between
            || self.criteria == ConditionalFormatCellCriteria::NotBetween
        {
            writer.xml_data_element_only("formula", &self.maximum.value);
        }

        writer.xml_end_tag("cfRule");

        writer.read_to_string()
    }

    // Return an extended x14 rule for conditional formats that support it.
    #[allow(clippy::unused_self)]
    pub(crate) fn x14_rule(&self, _guid: &str) -> String {
        String::new()
    }
}

// -----------------------------------------------------------------------
// ConditionalFormatBlank
// -----------------------------------------------------------------------

/// The `ConditionalFormatBlank` struct represents a Blank/Non-blank conditional
/// format.
///
/// `ConditionalFormatBlank` is used to represent a Blank or Non-blank style
/// conditional format in Excel. A Blank conditional format highlights blank
/// values in a range while the inverted version highlights non-blanks values.
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_blank_intro.png">
///
/// For more information see [Working with Conditional
/// Formats](crate::conditional_format).
///
/// # Examples
///
/// Example of how to add a blank/non-blank conditional formatting to a
/// worksheet. Blank values are in light red. Non-blank values are in light
/// green. Note, that we invert the Blank rule to get Non-blank values.
///
/// ```
/// # // This code is available in examples/doc_conditional_format_blank.rs
/// #
/// # use rust_xlsxwriter::{ConditionalFormatBlank, Format, Workbook, XlsxError};
/// #
/// # fn main() -> Result<(), XlsxError> {
/// #     // Create a new Excel file object.
/// #     let mut workbook = Workbook::new();
/// #     let worksheet = workbook.add_worksheet();
/// #
/// #     // Add some sample data.
/// #     let data = [
/// #         "Not blank", "", "", "Not blank", "Not blank", "", "Not blank", "Not blank", "", "Not blank", "", "Not blank",
/// #     ];
/// #     worksheet.write_column(0, 0, data)?;
/// #
/// #     // Set the column widths for clarity.
/// #     for col_num in 1..=10u16 {
/// #         worksheet.set_column_width(col_num, 6)?;
/// #     }
/// #
/// #     // Add a format. Light red fill with dark red text.
/// #     let format1 = Format::new()
/// #         .set_font_color("9C0006")
/// #         .set_background_color("FFC7CE");
/// #
/// #     // Add a format. Green fill with dark green text.
/// #     let format2 = Format::new()
/// #         .set_font_color("006100")
/// #         .set_background_color("C6EFCE");
/// #
///     // Write a conditional format over a range.
///     let conditional_format = ConditionalFormatBlank::new().set_format(format1);
///
///     worksheet.add_conditional_format(0, 0, 11, 0, &conditional_format)?;
///
///     // Invert the blank conditional format to show non-blank values.
///     let conditional_format = ConditionalFormatBlank::new().invert().set_format(format2);
///
///     worksheet.add_conditional_format(0, 0, 11, 0, &conditional_format)?;
///
/// #     // Save the file.
/// #     workbook.save("conditional_format.xlsx")?;
/// #
/// #     Ok(())
/// # }
/// ```
///
/// This creates conditional format rules like this:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_blank_rules.png">
///
/// And the following output file:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_blank.png">
///
#[derive(Clone)]
pub struct ConditionalFormatBlank {
    is_inverted: bool,
    multi_range: String,
    stop_if_true: bool,
    has_x14_extensions: bool,
    pub(crate) format: Option<Format>,
}

/// **Section 1**: The following methods are specific to `ConditionalFormatBlank`.
impl ConditionalFormatBlank {
    /// Create a new Blank conditional format struct.
    #[allow(clippy::new_without_default)]
    pub fn new() -> ConditionalFormatBlank {
        ConditionalFormatBlank {
            is_inverted: false,
            multi_range: String::new(),
            stop_if_true: false,
            has_x14_extensions: false,
            format: None,
        }
    }

    /// Invert the functionality of the conditional format to get Non-blank
    /// values instead of Blank values.
    ///
    /// See the example above.
    ///
    pub fn invert(mut self) -> ConditionalFormatBlank {
        self.is_inverted = true;
        self
    }

    // Validate the conditional format.
    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::unused_self)]
    pub(crate) fn validate(&self) -> Result<(), XlsxError> {
        Ok(())
    }

    //  Return the conditional format rule as an XML string.
    pub(crate) fn rule(
        &self,
        dxf_index: Option<u32>,
        priority: u32,
        range: &str,
        _guid: &str,
    ) -> String {
        let mut writer = XMLWriter::new();
        let mut attributes = vec![];
        let anchor = &range_to_anchor(range);

        // Write the type.
        if self.is_inverted {
            attributes.push(("type", "notContainsBlanks".to_string()));
        } else {
            attributes.push(("type", "containsBlanks".to_string()));
        }

        // Set the format index if present.
        if let Some(dxf_index) = dxf_index {
            attributes.push(("dxfId", dxf_index.to_string()));
        }

        // Set the rule priority order.
        attributes.push(("priority", priority.to_string()));

        // Set the "Stop if True" property.
        if self.stop_if_true {
            attributes.push(("stopIfTrue", "1".to_string()));
        }

        // Create the formula for the rule.
        let formula = if self.is_inverted {
            format!("LEN(TRIM({anchor}))>0")
        } else {
            format!("LEN(TRIM({anchor}))=0")
        };

        // Write the rule.
        writer.xml_start_tag("cfRule", &attributes);
        writer.xml_data_element_only("formula", &formula);
        writer.xml_end_tag("cfRule");

        writer.read_to_string()
    }

    // Return an extended x14 rule for conditional formats that support it.
    #[allow(clippy::unused_self)]
    pub(crate) fn x14_rule(&self, _guid: &str) -> String {
        String::new()
    }
}

// -----------------------------------------------------------------------
// ConditionalFormatError
// -----------------------------------------------------------------------

/// The `ConditionalFormatError` struct represents an Error/Non-error conditional
/// format.
///
/// `ConditionalFormatError` is used to represent an Error or Non-error style
/// conditional format in Excel. An error conditional format highlights error
/// values in a range while the inverted version highlights non-errors values.
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_error_intro.png">
///
/// For more information see [Working with Conditional
/// Formats](crate::conditional_format).
///
/// # Examples
///
/// Example of how to add an error/non-error conditional formatting to a
/// worksheet. Error values are in light red. Non-error values are in light
/// green. Note, that we invert the Error rule to get Non-error values.
///
/// ```
/// # // This code is available in examples/doc_conditional_format_error.rs
/// #
/// # use rust_xlsxwriter::{ConditionalFormatError, Format, Formula, Workbook, XlsxError};
/// #
/// # fn main() -> Result<(), XlsxError> {
/// #     // Create a new Excel file object.
/// #     let mut workbook = Workbook::new();
/// #     let worksheet = workbook.add_worksheet();
/// #
/// #     // Add some sample data.
/// #     worksheet.write(0, 0, Formula::new("=1/1"))?;
/// #     worksheet.write(1, 0, Formula::new("=1/0"))?;
/// #     worksheet.write(2, 0, Formula::new("=1/0"))?;
/// #     worksheet.write(3, 0, Formula::new("=1/1"))?;
/// #     worksheet.write(4, 0, Formula::new("=1/1"))?;
/// #     worksheet.write(5, 0, Formula::new("=1/0"))?;
/// #     worksheet.write(6, 0, Formula::new("=1/1"))?;
/// #     worksheet.write(7, 0, Formula::new("=1/1"))?;
/// #     worksheet.write(8, 0, Formula::new("=1/0"))?;
/// #     worksheet.write(9, 0, Formula::new("=1/1"))?;
/// #     worksheet.write(10, 0, Formula::new("=1/0"))?;
/// #     worksheet.write(11, 0, Formula::new("=1/1"))?;
/// #
/// #     // Set the column widths for clarity.
/// #     for col_num in 1..=10u16 {
/// #         worksheet.set_column_width(col_num, 6)?;
/// #     }
/// #
/// #     // Add a format. Light red fill with dark red text.
/// #     let format1 = Format::new()
/// #         .set_font_color("9C0006")
/// #         .set_background_color("FFC7CE");
/// #
/// #     // Add a format. Green fill with dark green text.
/// #     let format2 = Format::new()
/// #         .set_font_color("006100")
/// #         .set_background_color("C6EFCE");
/// #
///     // Write a conditional format over a range.
///     let conditional_format = ConditionalFormatError::new().set_format(format1);
///
///     worksheet.add_conditional_format(0, 0, 11, 0, &conditional_format)?;
///
///     // Invert the error conditional format to show non-error values.
///     let conditional_format = ConditionalFormatError::new().invert().set_format(format2);
///
///     worksheet.add_conditional_format(0, 0, 11, 0, &conditional_format)?;
///
/// #     // Save the file.
/// #     workbook.save("conditional_format.xlsx")?;
/// #
/// #     Ok(())
/// # }
/// ```
///
/// This creates conditional format rules like this:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_error_rules.png">
///
/// And the following output file:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_error.png">
///
#[derive(Clone)]
pub struct ConditionalFormatError {
    is_inverted: bool,
    multi_range: String,
    stop_if_true: bool,
    has_x14_extensions: bool,
    pub(crate) format: Option<Format>,
}

/// **Section 1**: The following methods are specific to `ConditionalFormatError`.
impl ConditionalFormatError {
    /// Create a new Error conditional format struct.
    #[allow(clippy::new_without_default)]
    pub fn new() -> ConditionalFormatError {
        ConditionalFormatError {
            is_inverted: false,
            multi_range: String::new(),
            stop_if_true: false,
            has_x14_extensions: false,
            format: None,
        }
    }

    /// Invert the functionality of the conditional format to get Non-error
    /// values instead of Error values.
    ///
    /// See the example above.
    ///
    pub fn invert(mut self) -> ConditionalFormatError {
        self.is_inverted = true;
        self
    }

    // Validate the conditional format.
    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::unused_self)]
    pub(crate) fn validate(&self) -> Result<(), XlsxError> {
        Ok(())
    }

    //  Return the conditional format rule as an XML string.
    pub(crate) fn rule(
        &self,
        dxf_index: Option<u32>,
        priority: u32,
        range: &str,
        _guid: &str,
    ) -> String {
        let mut writer = XMLWriter::new();
        let mut attributes = vec![];
        let anchor = &range_to_anchor(range);

        // Write the type.
        if self.is_inverted {
            attributes.push(("type", "notContainsErrors".to_string()));
        } else {
            attributes.push(("type", "containsErrors".to_string()));
        }

        // Set the format index if present.
        if let Some(dxf_index) = dxf_index {
            attributes.push(("dxfId", dxf_index.to_string()));
        }

        // Set the rule priority order.
        attributes.push(("priority", priority.to_string()));

        // Set the "Stop if True" property.
        if self.stop_if_true {
            attributes.push(("stopIfTrue", "1".to_string()));
        }

        // Create the formula for the rule.
        let formula = if self.is_inverted {
            format!("NOT(ISERROR({anchor}))")
        } else {
            format!("ISERROR({anchor})")
        };

        // Write the rule.
        writer.xml_start_tag("cfRule", &attributes);
        writer.xml_data_element_only("formula", &formula);
        writer.xml_end_tag("cfRule");

        writer.read_to_string()
    }

    // Return an extended x14 rule for conditional formats that support it.
    #[allow(clippy::unused_self)]
    pub(crate) fn x14_rule(&self, _guid: &str) -> String {
        String::new()
    }
}

// -----------------------------------------------------------------------
// ConditionalFormatDuplicate
// -----------------------------------------------------------------------

/// The `ConditionalFormatDuplicate` struct represents a Duplicate/Unique
/// conditional format.
///
/// `ConditionalFormatDuplicate` is used to represent a Duplicate or Unique
/// style conditional format in Excel. Duplicate conditional formats show
/// duplicated values in a range while Unique conditional formats show unique
/// values.
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_duplicate_intro.png">
///
/// For more information see [Working with Conditional Formats](crate::conditional_format).
///
/// # Examples
///
/// Example of how to add a duplicate/unique conditional formatting to a
/// worksheet. Duplicate values are in light red. Unique values are in light
/// green. Note, that we invert the Duplicate rule to get Unique values.
///
/// ```
/// # // This code is available in examples/doc_conditional_format_duplicate.rs
/// #
/// # use rust_xlsxwriter::{ConditionalFormatDuplicate, Format, Workbook, XlsxError};
/// #
/// # fn main() -> Result<(), XlsxError> {
/// #     // Create a new Excel file object.
/// #     let mut workbook = Workbook::new();
/// #     let worksheet = workbook.add_worksheet();
/// #
/// #     // Add some sample data.
/// #     let data = [
/// #         [34, 72, 38, 30, 75, 48, 75, 66, 84, 86],
/// #         [6, 24, 1, 84, 54, 62, 60, 3, 26, 59],
/// #         [28, 79, 97, 13, 85, 93, 93, 22, 5, 14],
/// #         [27, 71, 40, 17, 18, 79, 90, 93, 29, 47],
/// #         [88, 25, 33, 23, 67, 1, 59, 79, 47, 36],
/// #         [24, 100, 20, 88, 29, 33, 38, 54, 54, 88],
/// #         [6, 57, 88, 28, 10, 26, 37, 7, 41, 48],
/// #         [52, 78, 1, 96, 26, 45, 47, 33, 96, 36],
/// #         [60, 54, 81, 66, 81, 90, 80, 93, 12, 55],
/// #         [70, 5, 46, 14, 71, 19, 66, 36, 41, 21],
/// #     ];
/// #     worksheet.write_row_matrix(2, 1, data)?;
/// #
/// #     // Set the column widths for clarity.
/// #     for col_num in 1..=10u16 {
/// #         worksheet.set_column_width(col_num, 6)?;
/// #     }
/// #
/// #     // Add a format. Light red fill with dark red text.
/// #     let format1 = Format::new()
/// #         .set_font_color("9C0006")
/// #         .set_background_color("FFC7CE");
/// #
/// #     // Add a format. Green fill with dark green text.
/// #     let format2 = Format::new()
/// #         .set_font_color("006100")
/// #         .set_background_color("C6EFCE");
/// #
///     // Write a conditional format over a range.
///     let conditional_format = ConditionalFormatDuplicate::new().set_format(format1);
///
///     worksheet.add_conditional_format(2, 1, 11, 10, &conditional_format)?;
///
///     // Invert the duplicate conditional format to show unique values.
///     let conditional_format = ConditionalFormatDuplicate::new()
///         .invert()
///         .set_format(format2);
///
///     worksheet.add_conditional_format(2, 1, 11, 10, &conditional_format)?;
///
/// #     // Save the file.
/// #     workbook.save("conditional_format.xlsx")?;
/// #
/// #     Ok(())
/// # }
/// ```
///
/// This creates conditional format rules like this:
///
/// <img src="https://rustxlsxwriter.github.io/images/conditional_format_duplicate_rules.png">
///
/// And the following output file:
///
/// <img src="https://rustxlsxwriter.github.io/images/conditional_format_duplicate.png">
///
#[derive(Clone)]
pub struct ConditionalFormatDuplicate {
    is_inverted: bool,
    multi_range: String,
    stop_if_true: bool,
    has_x14_extensions: bool,
    pub(crate) format: Option<Format>,
}

/// **Section 1**: The following methods are specific to `ConditionalFormatDuplicate`.
impl ConditionalFormatDuplicate {
    /// Create a new Duplicate conditional format struct.
    #[allow(clippy::new_without_default)]
    pub fn new() -> ConditionalFormatDuplicate {
        ConditionalFormatDuplicate {
            is_inverted: false,
            multi_range: String::new(),
            stop_if_true: false,
            has_x14_extensions: false,
            format: None,
        }
    }

    /// Invert the functionality of the conditional format to get Unique values
    /// instead of Duplicate values.
    ///
    /// See the example above.
    ///
    pub fn invert(mut self) -> ConditionalFormatDuplicate {
        self.is_inverted = true;
        self
    }

    // Validate the conditional format.
    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::unused_self)]
    pub(crate) fn validate(&self) -> Result<(), XlsxError> {
        Ok(())
    }

    //  Return the conditional format rule as an XML string.
    pub(crate) fn rule(
        &self,
        dxf_index: Option<u32>,
        priority: u32,
        _range: &str,
        _guid: &str,
    ) -> String {
        let mut writer = XMLWriter::new();
        let mut attributes = vec![];

        if self.is_inverted {
            attributes.push(("type", "uniqueValues".to_string()));
        } else {
            attributes.push(("type", "duplicateValues".to_string()));
        }

        // Set the format index if present.
        if let Some(dxf_index) = dxf_index {
            attributes.push(("dxfId", dxf_index.to_string()));
        }

        // Set the rule priority order.
        attributes.push(("priority", priority.to_string()));

        // Set the "Stop if True" property.
        if self.stop_if_true {
            attributes.push(("stopIfTrue", "1".to_string()));
        }

        // Write the rule.
        writer.xml_empty_tag("cfRule", &attributes);

        writer.read_to_string()
    }

    // Return an extended x14 rule for conditional formats that support it.
    #[allow(clippy::unused_self)]
    pub(crate) fn x14_rule(&self, _guid: &str) -> String {
        String::new()
    }
}

// -----------------------------------------------------------------------
// ConditionalFormatFormula
// -----------------------------------------------------------------------

/// The `ConditionalFormatFormula` struct represents a Formula style conditional
/// format.
///
/// `ConditionalFormatFormula` is used to represent a Formula style conditional
/// format in Excel. A Formula conditional format highlights formula values in a
/// range based on a user supplied formula.
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_formula_intro.png">
///
/// For more information see [Working with Conditional
/// Formats](crate::conditional_format).
///
/// # Examples
///
/// Example of adding a Formula type conditional formatting to a worksheet.
/// Cells with odd numbered values are in light red while even numbered values
/// are in light green.
///
/// ```
/// # // This code is available in examples/doc_conditional_format_formula.rs
/// #
/// # use rust_xlsxwriter::{ConditionalFormatFormula, Format, Workbook, XlsxError};
/// #
/// # fn main() -> Result<(), XlsxError> {
/// #     // Create a new Excel file object.
/// #     let mut workbook = Workbook::new();
/// #     let worksheet = workbook.add_worksheet();
/// #
/// #     // Add some sample data.
/// #     let data = [
/// #         [34, 72, 38, 30, 75, 48, 75, 66, 84, 86],
/// #         [6, 24, 1, 84, 54, 62, 60, 3, 26, 59],
/// #         [28, 79, 97, 13, 85, 93, 93, 22, 5, 14],
/// #         [27, 71, 40, 17, 18, 79, 90, 93, 29, 47],
/// #         [88, 25, 33, 23, 67, 1, 59, 79, 47, 36],
/// #         [24, 100, 20, 88, 29, 33, 38, 54, 54, 88],
/// #         [6, 57, 88, 28, 10, 26, 37, 7, 41, 48],
/// #         [52, 78, 1, 96, 26, 45, 47, 33, 96, 36],
/// #         [60, 54, 81, 66, 81, 90, 80, 93, 12, 55],
/// #         [70, 5, 46, 14, 71, 19, 66, 36, 41, 21],
/// #     ];
/// #     worksheet.write_row_matrix(2, 1, data)?;
/// #
/// #     // Set the column widths for clarity.
/// #     for col_num in 1..=10u16 {
/// #         worksheet.set_column_width(col_num, 6)?;
/// #     }
/// #
/// #     // Add a format. Light red fill with dark red text.
/// #     let format1 = Format::new()
/// #         .set_font_color("9C0006")
/// #         .set_background_color("FFC7CE");
/// #
/// #     // Add a format. Green fill with dark green text.
/// #     let format2 = Format::new()
/// #         .set_font_color("006100")
/// #         .set_background_color("C6EFCE");
/// #
///     // Write a conditional format over a range.
///     let conditional_format = ConditionalFormatFormula::new()
///         .set_value("=ISODD(B3)")
///         .set_format(format1);
///
///     worksheet.add_conditional_format(2, 1, 11, 10, &conditional_format)?;
///
///     // Write another conditional format over the same range.
///     let conditional_format = ConditionalFormatFormula::new()
///         .set_value("=ISEVEN(B3)")
///         .set_format(format2);
///
///     worksheet.add_conditional_format(2, 1, 11, 10, &conditional_format)?;
///
/// #     // Save the file.
/// #     workbook.save("conditional_format.xlsx")?;
/// #
/// #     Ok(())
/// # }
/// ```
///
/// This creates conditional format rules like this:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_formula_rules.png">
///
/// And the following output file:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_formula.png">
///
#[derive(Clone)]
pub struct ConditionalFormatFormula {
    formula: Formula,
    multi_range: String,
    stop_if_true: bool,
    has_x14_extensions: bool,
    pub(crate) format: Option<Format>,
}

/// **Section 1**: The following methods are specific to `ConditionalFormatFormula`.
impl ConditionalFormatFormula {
    /// Create a new Formula conditional format struct.
    #[allow(clippy::new_without_default)]
    pub fn new() -> ConditionalFormatFormula {
        ConditionalFormatFormula {
            formula: Formula::new(""),
            multi_range: String::new(),
            stop_if_true: false,
            has_x14_extensions: false,
            format: None,
        }
    }

    /// Set the value of a Formula conditional format rule.
    ///
    /// Set the formula in a Formula style conditional format.
    ///
    /// There are some caveats to be aware of when creating the formula:
    ///
    /// - The formula must evaluate to a boolean, number, date, time or string.
    /// - Some newer "dynamic range" style functions aren't supported by Excel
    ///   in Formula conditional formats.
    /// - The formula should be in English with US style punctuation. See
    ///   [`Formula`] for details.
    ///
    /// If you encounter any issues you should verify that the formula works in
    /// Excel before transferring it to `rust_xlsxwriter`.
    ///
    /// # Parameters
    ///
    /// * `value` - A [`Formula`] value or type that converts "into" a `Formula`
    ///   such as a `&str` or `&Formula`.
    ///
    /// # Examples
    ///
    /// Example of adding a Formula type conditional formatting to a worksheet.
    /// Cells with odd numbered values are in light red while even numbered
    /// values are in light green.
    ///
    /// ```
    /// # // This code is available in examples/doc_conditional_format_formula.rs
    /// #
    /// # use rust_xlsxwriter::{ConditionalFormatFormula, Format, Workbook, XlsxError};
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #     let worksheet = workbook.add_worksheet();
    /// #
    /// #     // Add some sample data.
    /// #     let data = [
    /// #         [34, 72, 38, 30, 75, 48, 75, 66, 84, 86],
    /// #         [6, 24, 1, 84, 54, 62, 60, 3, 26, 59],
    /// #         [28, 79, 97, 13, 85, 93, 93, 22, 5, 14],
    /// #         [27, 71, 40, 17, 18, 79, 90, 93, 29, 47],
    /// #         [88, 25, 33, 23, 67, 1, 59, 79, 47, 36],
    /// #         [24, 100, 20, 88, 29, 33, 38, 54, 54, 88],
    /// #         [6, 57, 88, 28, 10, 26, 37, 7, 41, 48],
    /// #         [52, 78, 1, 96, 26, 45, 47, 33, 96, 36],
    /// #         [60, 54, 81, 66, 81, 90, 80, 93, 12, 55],
    /// #         [70, 5, 46, 14, 71, 19, 66, 36, 41, 21],
    /// #     ];
    /// #     worksheet.write_row_matrix(2, 1, data)?;
    /// #
    /// #     // Set the column widths for clarity.
    /// #     for col_num in 1..=10u16 {
    /// #         worksheet.set_column_width(col_num, 6)?;
    /// #     }
    /// #
    /// #     // Add a format. Light red fill with dark red text.
    /// #     let format1 = Format::new()
    /// #         .set_font_color("9C0006")
    /// #         .set_background_color("FFC7CE");
    /// #
    /// #     // Add a format. Green fill with dark green text.
    /// #     let format2 = Format::new()
    /// #         .set_font_color("006100")
    /// #         .set_background_color("C6EFCE");
    /// #
    ///     // Write a conditional format over a range.
    ///     let conditional_format = ConditionalFormatFormula::new()
    ///         .set_value("=ISODD(B3)")
    ///         .set_format(format1);
    ///
    ///     worksheet.add_conditional_format(2, 1, 11, 10, &conditional_format)?;
    ///
    ///     // Write another conditional format over the same range.
    ///     let conditional_format = ConditionalFormatFormula::new()
    ///         .set_value("=ISEVEN(B3)")
    ///         .set_format(format2);
    ///
    ///     worksheet.add_conditional_format(2, 1, 11, 10, &conditional_format)?;
    ///
    /// #     // Save the file.
    /// #     workbook.save("conditional_format.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img
    /// src="https://rustxlsxwriter.github.io/images/conditional_format_formula.png">
    ///
    pub fn set_value(mut self, value: impl Into<Formula>) -> ConditionalFormatFormula {
        self.formula = value.into();
        self
    }

    // Validate the conditional format.
    pub(crate) fn validate(&self) -> Result<(), XlsxError> {
        if self.formula.expand_formula(true).is_empty() {
            return Err(XlsxError::ConditionalFormatError(
                "Formula value must be set".to_string(),
            ));
        }
        Ok(())
    }

    //  Return the conditional format rule as an XML string.
    pub(crate) fn rule(
        &self,
        dxf_index: Option<u32>,
        priority: u32,
        _range: &str,
        _guid: &str,
    ) -> String {
        let mut writer = XMLWriter::new();
        let mut attributes = vec![];

        // Write the type.
        attributes.push(("type", "expression".to_string()));

        // Set the format index if present.
        if let Some(dxf_index) = dxf_index {
            attributes.push(("dxfId", dxf_index.to_string()));
        }

        // Set the rule priority order.
        attributes.push(("priority", priority.to_string()));

        // Set the "Stop if True" property.
        if self.stop_if_true {
            attributes.push(("stopIfTrue", "1".to_string()));
        }

        // Write the rule.
        writer.xml_start_tag("cfRule", &attributes);
        writer.xml_data_element_only("formula", &self.formula.expand_formula(true));
        writer.xml_end_tag("cfRule");

        writer.read_to_string()
    }

    // Return an extended x14 rule for conditional formats that support it.
    #[allow(clippy::unused_self)]
    pub(crate) fn x14_rule(&self, _guid: &str) -> String {
        String::new()
    }
}

// -----------------------------------------------------------------------
// ConditionalFormatAverage
// -----------------------------------------------------------------------

/// The `ConditionalFormatAverage` struct represents an Average/Standard
/// Deviation style conditional format.
///
/// `ConditionalFormatAverage` is used to represent a Average or Standard Deviation style
/// conditional format in Excel.
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_average_intro.png">
///
/// For more information see [Working with Conditional
/// Formats](crate::conditional_format).
///
/// # Examples
///
/// Example of how to add Average conditional formatting to a worksheet. Above
/// average values are in light red. Below average values are in light green.
///
/// ```
/// # // This code is available in examples/doc_conditional_format_average.rs
/// #
/// # use rust_xlsxwriter::{
/// #     ConditionalFormatAverage, ConditionalFormatAverageCriteria, Format, Workbook, XlsxError,
/// # };
/// #
/// # fn main() -> Result<(), XlsxError> {
/// #     // Create a new Excel file object.
/// #     let mut workbook = Workbook::new();
/// #     let worksheet = workbook.add_worksheet();
/// #
/// #     // Add some sample data.
/// #     let data = [
/// #         [34, 72, 38, 30, 75, 48, 75, 66, 84, 86],
/// #         [6, 24, 1, 84, 54, 62, 60, 3, 26, 59],
/// #         [28, 79, 97, 13, 85, 93, 93, 22, 5, 14],
/// #         [27, 71, 40, 17, 18, 79, 90, 93, 29, 47],
/// #         [88, 25, 33, 23, 67, 1, 59, 79, 47, 36],
/// #         [24, 100, 20, 88, 29, 33, 38, 54, 54, 88],
/// #         [6, 57, 88, 28, 10, 26, 37, 7, 41, 48],
/// #         [52, 78, 1, 96, 26, 45, 47, 33, 96, 36],
/// #         [60, 54, 81, 66, 81, 90, 80, 93, 12, 55],
/// #         [70, 5, 46, 14, 71, 19, 66, 36, 41, 21],
/// #     ];
/// #     worksheet.write_row_matrix(2, 1, data)?;
/// #
/// #     // Set the column widths for clarity.
/// #     for col_num in 1..=10u16 {
/// #         worksheet.set_column_width(col_num, 6)?;
/// #     }
/// #
/// #     // Add a format. Light red fill with dark red text.
/// #     let format1 = Format::new()
/// #         .set_font_color("9C0006")
/// #         .set_background_color("FFC7CE");
/// #
/// #     // Add a format. Green fill with dark green text.
/// #     let format2 = Format::new()
/// #         .set_font_color("006100")
/// #         .set_background_color("C6EFCE");
/// #
///     // Write a conditional format. The default criteria is Above Average.
///     let conditional_format = ConditionalFormatAverage::new().set_format(format1);
///
///     worksheet.add_conditional_format(2, 1, 11, 10, &conditional_format)?;
///
///     // Write another conditional format over the same range.
///     let conditional_format = ConditionalFormatAverage::new()
///         .set_criteria(ConditionalFormatAverageCriteria::BelowAverage)
///         .set_format(format2);
///
///     worksheet.add_conditional_format(2, 1, 11, 10, &conditional_format)?;
///
/// #     // Save the file.
/// #     workbook.save("conditional_format.xlsx")?;
/// #
/// #     Ok(())
/// # }
/// ```
///
/// This creates conditional format rules like this:
///
/// <img src="https://rustxlsxwriter.github.io/images/conditional_format_average_rules.png">
///
/// And the following output file:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_average.png">
///
#[derive(Clone)]
pub struct ConditionalFormatAverage {
    criteria: ConditionalFormatAverageCriteria,
    multi_range: String,
    stop_if_true: bool,
    has_x14_extensions: bool,
    pub(crate) format: Option<Format>,
}

/// **Section 1**: The following methods are specific to `ConditionalFormatAverage`.
impl ConditionalFormatAverage {
    /// Create a new Average conditional format struct.
    #[allow(clippy::new_without_default)]
    pub fn new() -> ConditionalFormatAverage {
        ConditionalFormatAverage {
            criteria: ConditionalFormatAverageCriteria::AboveAverage,
            multi_range: String::new(),
            stop_if_true: false,
            has_x14_extensions: false,
            format: None,
        }
    }

    /// Set the criteria for the Average conditional format rule such as above
    /// or below average or standard deviation ranges.
    ///
    /// # Parameters
    ///
    /// * `criteria` - A [`ConditionalFormatAverageCriteria`] enum value.
    ///
    pub fn set_criteria(
        mut self,
        criteria: ConditionalFormatAverageCriteria,
    ) -> ConditionalFormatAverage {
        self.criteria = criteria;
        self
    }

    // Validate the conditional format.
    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::unused_self)]
    pub(crate) fn validate(&self) -> Result<(), XlsxError> {
        Ok(())
    }

    //  Return the conditional format rule as an XML string.
    pub(crate) fn rule(
        &self,
        dxf_index: Option<u32>,
        priority: u32,
        _range: &str,
        _guid: &str,
    ) -> String {
        let mut writer = XMLWriter::new();
        let mut attributes = vec![("type", "aboveAverage".to_string())];

        // Set the format index if present.
        if let Some(dxf_index) = dxf_index {
            attributes.push(("dxfId", dxf_index.to_string()));
        }

        // Set the rule priority order.
        attributes.push(("priority", priority.to_string()));

        // Set the "Stop if True" property.
        if self.stop_if_true {
            attributes.push(("stopIfTrue", "1".to_string()));
        }

        // Set the Average specific attributes.
        match self.criteria {
            ConditionalFormatAverageCriteria::AboveAverage => {
                // There are no additional attributes for above average.
            }

            ConditionalFormatAverageCriteria::BelowAverage => {
                attributes.push(("aboveAverage", "0".to_string()));
            }

            ConditionalFormatAverageCriteria::EqualOrAboveAverage => {
                attributes.push(("equalAverage", "1".to_string()));
            }

            ConditionalFormatAverageCriteria::EqualOrBelowAverage => {
                attributes.push(("aboveAverage", "0".to_string()));
                attributes.push(("equalAverage", "1".to_string()));
            }

            ConditionalFormatAverageCriteria::OneStandardDeviationAbove => {
                attributes.push(("stdDev", "1".to_string()));
            }

            ConditionalFormatAverageCriteria::OneStandardDeviationBelow => {
                attributes.push(("aboveAverage", "0".to_string()));
                attributes.push(("stdDev", "1".to_string()));
            }

            ConditionalFormatAverageCriteria::TwoStandardDeviationsAbove => {
                attributes.push(("stdDev", "2".to_string()));
            }

            ConditionalFormatAverageCriteria::TwoStandardDeviationsBelow => {
                attributes.push(("aboveAverage", "0".to_string()));

                attributes.push(("stdDev", "2".to_string()));
            }

            ConditionalFormatAverageCriteria::ThreeStandardDeviationsAbove => {
                attributes.push(("stdDev", "3".to_string()));
            }

            ConditionalFormatAverageCriteria::ThreeStandardDeviationsBelow => {
                attributes.push(("aboveAverage", "0".to_string()));

                attributes.push(("stdDev", "3".to_string()));
            }
        }

        // Write the rule.
        writer.xml_empty_tag("cfRule", &attributes);

        writer.read_to_string()
    }

    // Return an extended x14 rule for conditional formats that support it.
    #[allow(clippy::unused_self)]
    pub(crate) fn x14_rule(&self, _guid: &str) -> String {
        String::new()
    }
}

// -----------------------------------------------------------------------
// ConditionalFormatTop
// -----------------------------------------------------------------------

/// The `ConditionalFormatTop` struct represents a Top/Bottom style conditional
/// format.
///
/// `ConditionalFormatTop` is used to represent a Top or Bottom style
/// conditional format in Excel. Top conditional formats show the top X values
/// in a range. The value of the conditional can be a rank, i.e., Top X, or a
/// percentage, i.e., Top X%.
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_top_intro.png">
///
/// For more information see [Working with Conditional
/// Formats](crate::conditional_format).
///
/// # Examples
///
/// Example of how to add Top and Bottom conditional formatting to a worksheet.
/// Top 10 values are in light red. Bottom 10 values are in light green.
///
/// ```
/// # // This code is available in examples/doc_conditional_format_top.rs
/// #
/// # use rust_xlsxwriter::{ConditionalFormatTop, Format, Workbook, XlsxError};
/// #
/// # fn main() -> Result<(), XlsxError> {
/// #     // Create a new Excel file object.
/// #     let mut workbook = Workbook::new();
/// #     let worksheet = workbook.add_worksheet();
/// #
/// #     // Add some sample data.
/// #     let data = [
/// #         [34, 72, 38, 30, 75, 48, 75, 66, 84, 86],
/// #         [6, 24, 1, 84, 54, 62, 60, 3, 26, 59],
/// #         [28, 79, 97, 13, 85, 93, 93, 22, 5, 14],
/// #         [27, 71, 40, 17, 18, 79, 90, 93, 29, 47],
/// #         [88, 25, 33, 23, 67, 1, 59, 79, 47, 36],
/// #         [24, 100, 20, 88, 29, 33, 38, 54, 54, 88],
/// #         [6, 57, 88, 28, 10, 26, 37, 7, 41, 48],
/// #         [52, 78, 1, 96, 26, 45, 47, 33, 96, 36],
/// #         [60, 54, 81, 66, 81, 90, 80, 93, 12, 55],
/// #         [70, 5, 46, 14, 71, 19, 66, 36, 41, 21],
/// #     ];
/// #     worksheet.write_row_matrix(2, 1, data)?;
/// #
/// #     // Set the column widths for clarity.
/// #     for col_num in 1..=10u16 {
/// #         worksheet.set_column_width(col_num, 6)?;
/// #     }
/// #
/// #     // Add a format. Light red fill with dark red text.
/// #     let format1 = Format::new()
/// #         .set_font_color("9C0006")
/// #         .set_background_color("FFC7CE");
/// #
/// #     // Add a format. Green fill with dark green text.
/// #     let format2 = Format::new()
/// #         .set_font_color("006100")
/// #         .set_background_color("C6EFCE");
/// #
///     // Write a conditional format over a range.
///     let conditional_format = ConditionalFormatTop::new()
///         .set_value(10)
///         .set_format(format1);
///
///     worksheet.add_conditional_format(2, 1, 11, 10, &conditional_format)?;
///
///     // Invert the Top conditional format to show Bottom values.
///     let conditional_format = ConditionalFormatTop::new()
///         .invert()
///         .set_value(10)
///         .set_format(format2);
///     worksheet.add_conditional_format(2, 1, 11, 10, &conditional_format)?;
///
/// #     // Save the file.
/// #     workbook.save("conditional_format.xlsx")?;
/// #
/// #     Ok(())
/// # }
/// ```
///
/// This creates conditional format rules like this:
///
/// <img src="https://rustxlsxwriter.github.io/images/conditional_format_top_rules.png">
///
/// And the following output file:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_top.png">
///
#[derive(Clone)]
pub struct ConditionalFormatTop {
    value: u16,
    is_inverted: bool,
    is_percent: bool,
    multi_range: String,
    stop_if_true: bool,
    has_x14_extensions: bool,
    pub(crate) format: Option<Format>,
}

/// **Section 1**: The following methods are specific to `ConditionalFormatTop`.
impl ConditionalFormatTop {
    /// Create a new Top conditional format struct.
    #[allow(clippy::new_without_default)]
    pub fn new() -> ConditionalFormatTop {
        ConditionalFormatTop {
            value: 10,
            is_inverted: false,
            is_percent: false,
            multi_range: String::new(),
            stop_if_true: false,
            has_x14_extensions: false,
            format: None,
        }
    }

    /// Set the top/bottom rank of the conditional format.
    ///
    /// See the example above.
    ///
    pub fn set_value(mut self, value: u16) -> ConditionalFormatTop {
        self.value = value;
        self
    }

    /// Invert the functionality of the conditional format to the get Bottom values
    /// instead of the Top values.
    ///
    /// See the example above.
    ///
    pub fn invert(mut self) -> ConditionalFormatTop {
        self.is_inverted = true;
        self
    }

    /// Show the top/bottom percentage instead of rank.
    ///
    /// See the example above.
    ///
    pub fn set_percent(mut self) -> ConditionalFormatTop {
        self.is_percent = true;
        self
    }

    // Validate the conditional format.
    pub(crate) fn validate(&self) -> Result<(), XlsxError> {
        if !(1..=1000).contains(&self.value) {
            return Err(XlsxError::ConditionalFormatError(
                "value must be in the Excel range 1..1000".to_string(),
            ));
        }

        Ok(())
    }

    //  Return the conditional format rule as an XML string.
    pub(crate) fn rule(
        &self,
        dxf_index: Option<u32>,
        priority: u32,
        _range: &str,
        _guid: &str,
    ) -> String {
        let mut writer = XMLWriter::new();
        let mut attributes = vec![("type", "top10".to_string())];

        // Set the format index if present.
        if let Some(dxf_index) = dxf_index {
            attributes.push(("dxfId", dxf_index.to_string()));
        }

        // Set the rule priority order.
        attributes.push(("priority", priority.to_string()));

        // Set the "Stop if True" property.
        if self.stop_if_true {
            attributes.push(("stopIfTrue", "1".to_string()));
        }

        if self.is_percent {
            attributes.push(("percent", "1".to_string()));
        }

        if self.is_inverted {
            attributes.push(("bottom", "1".to_string()));
        }

        attributes.push(("rank", self.value.to_string()));

        // Write the rule.
        writer.xml_empty_tag("cfRule", &attributes);

        writer.read_to_string()
    }

    // Return an extended x14 rule for conditional formats that support it.
    #[allow(clippy::unused_self)]
    pub(crate) fn x14_rule(&self, _guid: &str) -> String {
        String::new()
    }
}

// -----------------------------------------------------------------------
// ConditionalFormatText
// -----------------------------------------------------------------------

/// The `ConditionalFormatText` struct represents a Text conditional format.
///
/// `ConditionalFormatText` is used to represent a Text style conditional format
/// in Excel. Text conditional formats use simple equalities such as "equal to"
/// or "greater than" or "between".
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_text_intro.png">
///
/// For more information see [Working with Conditional Formats](crate::conditional_format).
///
/// # Examples
///
/// Example of adding a text type conditional formatting to a worksheet.
///
/// ```
/// # // This code is available in examples/doc_conditional_format_text.rs
/// #
/// # use rust_xlsxwriter::{
/// #     ConditionalFormatText, ConditionalFormatTextCriteria, Format, Workbook, XlsxError,
/// # };
/// #
/// # fn main() -> Result<(), XlsxError> {
/// #     // Create a new Excel file object.
/// #     let mut workbook = Workbook::new();
/// #     let worksheet = workbook.add_worksheet();
/// #
/// #     // Add some sample data.
/// #     let data = [
/// #         "apocrustic",
/// #         "burstwort",
/// #         "cloudburst",
/// #         "crustification",
/// #         "distrustfulness",
/// #         "laurustine",
/// #         "outburst",
/// #         "rusticism",
/// #         "thunderburst",
/// #         "trustee",
/// #         "trustworthiness",
/// #         "unburstableness",
/// #         "unfrustratable",
/// #     ];
/// #     worksheet.write_column(0, 0, data)?;
/// #     worksheet.write_column(0, 2, data)?;
/// #
/// #     // Set the column widths for clarity.
/// #     worksheet.set_column_width(0, 20)?;
/// #     worksheet.set_column_width(2, 20)?;
/// #
/// #     // Add a format. Green fill with dark green text.
/// #     let format1 = Format::new()
/// #         .set_font_color("006100")
/// #         .set_background_color("C6EFCE");
/// #
/// #     // Add a format. Light red fill with dark red text.
/// #     let format2 = Format::new()
/// #         .set_font_color("9C0006")
/// #         .set_background_color("FFC7CE");
/// #
///     // Write a text "containing" conditional format over a range.
///     let conditional_format = ConditionalFormatText::new()
///         .set_criteria(ConditionalFormatTextCriteria::Contains)
///         .set_value("rust")
///         .set_format(&format1);
///
///     worksheet.add_conditional_format(0, 0, 12, 0, &conditional_format)?;
///
///     // Write a text "not containing" conditional format over the same range.
///     let conditional_format = ConditionalFormatText::new()
///         .set_criteria(ConditionalFormatTextCriteria::DoesNotContain)
///         .set_value("rust")
///         .set_format(&format2);
///
///     worksheet.add_conditional_format(0, 0, 12, 0, &conditional_format)?;
///
///     // Write a text "begins with" conditional format over a range.
///     let conditional_format = ConditionalFormatText::new()
///         .set_criteria(ConditionalFormatTextCriteria::BeginsWith)
///         .set_value("t")
///         .set_format(&format1);
///
///     worksheet.add_conditional_format(0, 2, 12, 2, &conditional_format)?;
///
///     // Write a text "ends with" conditional format over the same range.
///     let conditional_format = ConditionalFormatText::new()
///         .set_criteria(ConditionalFormatTextCriteria::EndsWith)
///         .set_value("t")
///         .set_format(&format2);
///
///     worksheet.add_conditional_format(0, 2, 12, 2, &conditional_format)?;
/// #
/// #     // Save the file.
/// #     workbook.save("conditional_format.xlsx")?;
/// #
/// #     Ok(())
/// # }
/// ```
///
/// This creates conditional format rules like this:
///
/// <img src="https://rustxlsxwriter.github.io/images/conditional_format_text_rules.png">
///
/// And the following output file:
///
/// <img src="https://rustxlsxwriter.github.io/images/conditional_format_text.png">
///
#[derive(Clone)]
pub struct ConditionalFormatText {
    value: String,
    criteria: ConditionalFormatTextCriteria,
    multi_range: String,
    stop_if_true: bool,
    has_x14_extensions: bool,
    pub(crate) format: Option<Format>,
}

/// **Section 1**: The following methods are specific to `ConditionalFormatText`.
impl ConditionalFormatText {
    /// Create a new Text conditional format struct.
    #[allow(clippy::new_without_default)]
    pub fn new() -> ConditionalFormatText {
        ConditionalFormatText {
            value: String::new(),
            criteria: ConditionalFormatTextCriteria::Contains,
            multi_range: String::new(),
            stop_if_true: false,
            has_x14_extensions: false,
            format: None,
        }
    }

    /// Set the value of the Text conditional format rule.
    ///
    /// # Parameters
    ///
    /// * `value` - A string like value.
    ///
    ///   Newer versions of Excel support support using a cell reference for the
    ///   value but that isn't currently supported by `rust_xlsxwriter`.
    ///
    pub fn set_value(mut self, value: impl Into<String>) -> ConditionalFormatText {
        self.value = value.into();
        self
    }

    /// Set the criteria for the Text conditional format rule such "contains" or
    /// "starts with".
    ///
    /// # Parameters
    ///
    /// * `criteria` - A [`ConditionalFormatTextCriteria`] enum value.
    ///
    pub fn set_criteria(
        mut self,
        criteria: ConditionalFormatTextCriteria,
    ) -> ConditionalFormatText {
        self.criteria = criteria;
        self
    }

    // Validate the conditional format.
    pub(crate) fn validate(&self) -> Result<(), XlsxError> {
        if self.value.is_empty() {
            return Err(XlsxError::ConditionalFormatError(
                "Text conditional format string cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    //  Return the conditional format rule as an XML string.
    pub(crate) fn rule(
        &self,
        dxf_index: Option<u32>,
        priority: u32,
        range: &str,
        _guid: &str,
    ) -> String {
        let mut writer = XMLWriter::new();
        let mut attributes = vec![];
        let text = self.value.clone();
        let anchor = &range_to_anchor(range);

        // Set the rule attributes based on the criteria.
        let formula = match self.criteria {
            ConditionalFormatTextCriteria::Contains => {
                attributes.push(("type", "containsText".to_string()));
                format!(r#"NOT(ISERROR(SEARCH("{text}",{anchor})))"#)
            }
            ConditionalFormatTextCriteria::DoesNotContain => {
                attributes.push(("type", "notContainsText".to_string()));
                format!(r#"ISERROR(SEARCH("{text}",{anchor}))"#)
            }
            ConditionalFormatTextCriteria::BeginsWith => {
                attributes.push(("type", "beginsWith".to_string()));
                format!(r#"LEFT({anchor},1)="{text}""#)
            }
            ConditionalFormatTextCriteria::EndsWith => {
                attributes.push(("type", "endsWith".to_string()));
                format!(r#"RIGHT({anchor},1)="{text}""#)
            }
        };

        // Set the format index if present.
        if let Some(dxf_index) = dxf_index {
            attributes.push(("dxfId", dxf_index.to_string()));
        }

        // Set the rule priority order.
        attributes.push(("priority", priority.to_string()));

        // Set the "Stop if True" property.
        if self.stop_if_true {
            attributes.push(("stopIfTrue", "1".to_string()));
        }

        // Add the attributes.
        attributes.push(("operator", self.criteria.to_string()));
        attributes.push(("text", text));

        writer.xml_start_tag("cfRule", &attributes);
        writer.xml_data_element_only("formula", &formula);
        writer.xml_end_tag("cfRule");

        writer.read_to_string()
    }

    // Return an extended x14 rule for conditional formats that support it.
    #[allow(clippy::unused_self)]
    pub(crate) fn x14_rule(&self, _guid: &str) -> String {
        String::new()
    }
}

// -----------------------------------------------------------------------
// ConditionalFormatDate
// -----------------------------------------------------------------------

/// The `ConditionalFormatDate` struct represents a Dates Occurring style
/// conditional format.
///
/// `ConditionalFormatDate` is used to represent a Dates Occurring style
/// conditional format in Excel. This is used to identify dates in ranges like
/// "Last Week" or "Last Month".
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_date_intro.png">
///
/// For more information see [Working with Conditional
/// Formats](crate::conditional_format).
///
/// # Examples
///
/// Example of adding a Dates Occurring type conditional formatting to a
/// worksheet. Note, the rules in this example such as "Last month", "This
/// month" and "Next month" are applied to the sample dates which by default are
/// for November 2023. Changes the dates to some range closer to the time you
/// run the example.
///
/// ```
/// # // This code is available in examples/doc_conditional_format_date.rs
/// #
/// # use rust_xlsxwriter::{
/// #     ConditionalFormatDate, ConditionalFormatDateCriteria, ExcelDateTime, Format, Workbook,
/// #     XlsxError,
/// # };
/// #
/// # fn main() -> Result<(), XlsxError> {
/// #     // Create a new Excel file object.
/// #     let mut workbook = Workbook::new();
/// #     let worksheet = workbook.add_worksheet();
/// #
/// #     // Create a date format.
/// #     let date_format = Format::new().set_num_format("yyyy-mm-dd");
/// #
/// #     // Add some sample data.
/// #     let dates = [
/// #         "2023-10-01",
/// #         "2023-11-05",
/// #         "2023-11-06",
/// #         "2023-10-04",
/// #         "2023-11-11",
/// #         "2023-11-02",
/// #         "2023-11-04",
/// #         "2023-11-12",
/// #         "2023-12-01",
/// #         "2023-12-13",
/// #         "2023-11-13",
/// #     ];
/// #
/// #     // Map the string dates to ExcelDateTime objects, while capturing any
/// #     // potential conversion errors.
/// #     let dates: Result<Vec<ExcelDateTime>, XlsxError> = dates
/// #         .into_iter()
/// #         .map(ExcelDateTime::parse_from_str)
/// #         .collect();
/// #     let dates = dates?;
/// #
/// #     worksheet.write_column_with_format(0, 0, dates, &date_format)?;
/// #
/// #     // Set the column widths for clarity.
/// #     worksheet.set_column_width(0, 20)?;
/// #
/// #     // Add a format. Light red fill with dark red text.
/// #     let format1 = Format::new()
/// #         .set_font_color("9C0006")
/// #         .set_background_color("FFC7CE");
/// #
/// #     // Add a format. Green fill with dark green text.
/// #     let format2 = Format::new()
/// #         .set_font_color("006100")
/// #         .set_background_color("C6EFCE");
/// #
/// #     // Add a format. Light yellow fill with dark yellow text.
/// #     let format3 = Format::new()
/// #         .set_font_color("9C6500")
/// #         .set_background_color("FFEB9C");
/// #
///     // Write conditional format over the same range.
///     let conditional_format = ConditionalFormatDate::new()
///         .set_criteria(ConditionalFormatDateCriteria::LastMonth)
///         .set_format(format1);
///
///     worksheet.add_conditional_format(0, 0, 10, 0, &conditional_format)?;
///
///     let conditional_format = ConditionalFormatDate::new()
///         .set_criteria(ConditionalFormatDateCriteria::ThisMonth)
///         .set_format(format2);
///
///     worksheet.add_conditional_format(0, 0, 10, 0, &conditional_format)?;
///
///     let conditional_format = ConditionalFormatDate::new()
///         .set_criteria(ConditionalFormatDateCriteria::NextMonth)
///         .set_format(format3);
///
///     worksheet.add_conditional_format(0, 0, 10, 0, &conditional_format)?;
/// #
/// #     // Save the file.
/// #     workbook.save("conditional_format.xlsx")?;
/// #
/// #     Ok(())
/// # }
/// ```
///
/// This creates conditional format rules like this:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_date_rules.png">
///
/// And the following output file:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_date.png">
///
#[derive(Clone)]
pub struct ConditionalFormatDate {
    criteria: ConditionalFormatDateCriteria,
    multi_range: String,
    stop_if_true: bool,
    has_x14_extensions: bool,
    pub(crate) format: Option<Format>,
}

/// **Section 1**: The following methods are specific to `ConditionalFormatDate`.
impl ConditionalFormatDate {
    /// Create a new Date conditional format struct.
    #[allow(clippy::new_without_default)]
    pub fn new() -> ConditionalFormatDate {
        ConditionalFormatDate {
            criteria: ConditionalFormatDateCriteria::Yesterday,
            multi_range: String::new(),
            stop_if_true: false,
            has_x14_extensions: false,
            format: None,
        }
    }

    /// Set the criteria for the Dates Occurring  conditional format rule such
    /// "Today" or "Last Week".
    ///
    /// # Parameters
    ///
    /// * `criteria` - A [`ConditionalFormatDateCriteria`] enum value.
    ///
    pub fn set_criteria(
        mut self,
        criteria: ConditionalFormatDateCriteria,
    ) -> ConditionalFormatDate {
        self.criteria = criteria;
        self
    }

    // Validate the conditional format.
    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::unused_self)]
    pub(crate) fn validate(&self) -> Result<(), XlsxError> {
        Ok(())
    }

    //  Return the conditional format rule as an XML string.
    pub(crate) fn rule(
        &self,
        dxf_index: Option<u32>,
        priority: u32,
        range: &str,
        _guid: &str,
    ) -> String {
        let mut writer = XMLWriter::new();
        let mut attributes = vec![("type", "timePeriod".to_string())];
        let anchor = &range_to_anchor(range);

        // Set the rule attributes based on the criteria.
        let formula = match self.criteria {
            ConditionalFormatDateCriteria::Yesterday => {
                format!("FLOOR({anchor},1)=TODAY()-1")
            }
            ConditionalFormatDateCriteria::Today => {
                format!("FLOOR({anchor},1)=TODAY()")
            }
            ConditionalFormatDateCriteria::Tomorrow => {
                format!("FLOOR({anchor},1)=TODAY()+1")
            }
            ConditionalFormatDateCriteria::Last7Days => {
                format!("AND(TODAY()-FLOOR({anchor},1)<=6,FLOOR({anchor},1)<=TODAY())")
            }

            ConditionalFormatDateCriteria::LastWeek => {
                format!("AND(TODAY()-ROUNDDOWN({anchor},0)>=(WEEKDAY(TODAY())),TODAY()-ROUNDDOWN({anchor},0)<(WEEKDAY(TODAY())+7))")
            }

            ConditionalFormatDateCriteria::ThisWeek => {
                format!("AND(TODAY()-ROUNDDOWN({anchor},0)<=WEEKDAY(TODAY())-1,ROUNDDOWN({anchor},0)-TODAY()<=7-WEEKDAY(TODAY()))")
            }

            ConditionalFormatDateCriteria::NextWeek => {
                format!("AND(ROUNDDOWN({anchor},0)-TODAY()>(7-WEEKDAY(TODAY())),ROUNDDOWN({anchor},0)-TODAY()<(15-WEEKDAY(TODAY())))")
            }

            ConditionalFormatDateCriteria::LastMonth => {
                format!("AND(MONTH({anchor})=MONTH(TODAY())-1,OR(YEAR({anchor})=YEAR(TODAY()),AND(MONTH({anchor})=1,YEAR({anchor})=YEAR(TODAY())-1)))")
            }

            ConditionalFormatDateCriteria::ThisMonth => {
                format!("AND(MONTH({anchor})=MONTH(TODAY()),YEAR({anchor})=YEAR(TODAY()))")
            }

            ConditionalFormatDateCriteria::NextMonth => {
                format!("AND(MONTH({anchor})=MONTH(TODAY())+1,OR(YEAR({anchor})=YEAR(TODAY()),AND(MONTH({anchor})=12,YEAR({anchor})=YEAR(TODAY())+1)))")
            }
        };

        // Set the format index if present.
        if let Some(dxf_index) = dxf_index {
            attributes.push(("dxfId", dxf_index.to_string()));
        }

        // Set the rule priority order.
        attributes.push(("priority", priority.to_string()));

        // Set the "Stop if True" property.
        if self.stop_if_true {
            attributes.push(("stopIfTrue", "1".to_string()));
        }

        // Add the attributes.
        attributes.push(("timePeriod", self.criteria.to_string()));

        writer.xml_start_tag("cfRule", &attributes);
        writer.xml_data_element_only("formula", &formula);
        writer.xml_end_tag("cfRule");

        writer.read_to_string()
    }

    // Return an extended x14 rule for conditional formats that support it.
    #[allow(clippy::unused_self)]
    pub(crate) fn x14_rule(&self, _guid: &str) -> String {
        String::new()
    }
}

// -----------------------------------------------------------------------
// ConditionalFormat2ColorScale
// -----------------------------------------------------------------------

/// The `ConditionalFormat2ColorScale` struct represents a 2 Color Scale
/// conditional format.
///
/// `ConditionalFormat2ColorScale` is used to represent a Cell style conditional
/// format in Excel. A 2 Color Scale Cell conditional format shows a per cell
/// color gradient from the minimum value to the maximum value.
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_2color_intro.png">
///
/// For more information see [Working with Conditional
/// Formats](crate::conditional_format).
///
/// # Examples
///
/// Example of adding a 2 color scale type conditional formatting to a worksheet.
/// Note, the colors in the fifth example (yellow to green) are the default
/// colors and could be omitted.
///
/// ```
/// # // This code is available in examples/doc_conditional_format_2color.rs
/// #
/// # use rust_xlsxwriter::{ConditionalFormat2ColorScale, Workbook, XlsxError};
/// #
/// # fn main() -> Result<(), XlsxError> {
/// #     // Create a new Excel file object.
/// #     let mut workbook = Workbook::new();
/// #     let worksheet = workbook.add_worksheet();
/// #
/// #     // Write the worksheet data.
/// #     let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// #     worksheet.write_column(2, 1, data)?;
/// #     worksheet.write_column(2, 3, data)?;
/// #     worksheet.write_column(2, 5, data)?;
/// #     worksheet.write_column(2, 7, data)?;
/// #     worksheet.write_column(2, 9, data)?;
/// #     worksheet.write_column(2, 11, data)?;
/// #
/// #     // Set the column widths for clarity.
/// #     for col_num in 0..=12u16 {
/// #         worksheet.set_column_width(col_num, 6)?;
/// #     }
/// #
///     // Write 2 color scale formats with standard Excel colors.
///     let conditional_format = ConditionalFormat2ColorScale::new()
///         .set_minimum_color("F8696B")
///         .set_maximum_color("FCFCFF");
///
///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
///
///     let conditional_format = ConditionalFormat2ColorScale::new()
///         .set_minimum_color("FCFCFF")
///         .set_maximum_color("F8696B");
///
///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
///
///     let conditional_format = ConditionalFormat2ColorScale::new()
///         .set_minimum_color("FCFCFF")
///         .set_maximum_color("63BE7B");
///
///     worksheet.add_conditional_format(2, 5, 11, 5, &conditional_format)?;
///
///     let conditional_format = ConditionalFormat2ColorScale::new()
///         .set_minimum_color("63BE7B")
///         .set_maximum_color("FCFCFF");
///
///     worksheet.add_conditional_format(2, 7, 11, 7, &conditional_format)?;
///
///     let conditional_format = ConditionalFormat2ColorScale::new()
///         .set_minimum_color("FFEF9C")
///         .set_maximum_color("63BE7B");
///
///     worksheet.add_conditional_format(2, 9, 11, 9, &conditional_format)?;
///
///     let conditional_format = ConditionalFormat2ColorScale::new()
///         .set_minimum_color("63BE7B")
///         .set_maximum_color("FFEF9C");
///
///     worksheet.add_conditional_format(2, 11, 11, 11, &conditional_format)?;
/// #
/// #     // Save the file.
/// #     workbook.save("conditional_format.xlsx")?;
/// #
/// #     Ok(())
/// # }
/// ```
///
/// This creates conditional format rules like this:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_2color_rules.png">
///
/// And the following output file:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_2color.png">
///
///
#[derive(Clone)]
pub struct ConditionalFormat2ColorScale {
    min_type: ConditionalFormatType,
    max_type: ConditionalFormatType,
    min_value: ConditionalFormatValue,
    max_value: ConditionalFormatValue,
    min_color: Color,
    max_color: Color,

    multi_range: String,
    stop_if_true: bool,
    has_x14_extensions: bool,
    pub(crate) format: Option<Format>,
}

/// **Section 1**: The following methods are specific to `ConditionalFormat2ColorScale`.
impl ConditionalFormat2ColorScale {
    /// Create a new Cell conditional format struct.
    #[allow(clippy::new_without_default)]
    #[allow(clippy::unreadable_literal)] // For RGB colors.
    pub fn new() -> ConditionalFormat2ColorScale {
        ConditionalFormat2ColorScale {
            min_type: ConditionalFormatType::Lowest,
            max_type: ConditionalFormatType::Highest,
            min_value: 0.into(),
            max_value: 0.into(),
            min_color: Color::RGB(0xFFEF9C),
            max_color: Color::RGB(0x63BE7B),
            multi_range: String::new(),
            stop_if_true: false,
            has_x14_extensions: false,
            format: None,
        }
    }

    /// Set the type and value of the minimum in the 2 color scale.
    ///
    /// Set the minimum type (number, percent, formula or percentile) and value
    /// for a 2 color scale type of conditional format. By default the minimum
    /// is the lowest value in the conditional formatting range.
    ///
    /// # Parameters
    ///
    /// * `rule_type`- A [`ConditionalFormatType`] enum value.
    /// * `value` - Any type that can convert into a [`ConditionalFormatValue`]
    ///   such as numbers, dates, times and formula ranges. String values are
    ///   ignored in this type of conditional format.
    ///
    /// # Examples
    ///
    /// Example of adding a 2 color scale type conditional formatting to a worksheet
    /// with user defined minimum and maximum values.
    ///
    /// ```
    /// # // This code is available in examples/doc_conditional_format_2color_set_minimum.rs
    /// #
    /// # use rust_xlsxwriter::{
    /// #     ConditionalFormat2ColorScale, ConditionalFormatType, Workbook, XlsxError,
    /// # };
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #     let worksheet = workbook.add_worksheet();
    /// #
    /// #     // Write the worksheet data.
    /// #     let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// #     worksheet.write_column(2, 1, data)?;
    /// #     worksheet.write_column(2, 3, data)?;
    /// #
    ///     // Write a 2 color scale formats with standard Excel colors. The conditional
    ///     // format is applied from the lowest to the highest value.
    ///     let conditional_format = ConditionalFormat2ColorScale::new();
    ///
    ///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
    ///
    ///     // Write a 2 color scale formats with standard Excel colors but a user
    ///     // defined range. Values <= 3 will be shown with the minimum color while
    ///     // values >= 7 will be shown with the maximum color.
    ///     let conditional_format = ConditionalFormat2ColorScale::new()
    ///         .set_minimum(ConditionalFormatType::Number, 3)
    ///         .set_maximum(ConditionalFormatType::Number, 7);
    ///
    ///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
    /// #
    /// #     // Save the file.
    /// #     workbook.save("conditional_format.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img src="https://rustxlsxwriter.github.io/images/conditional_format_2color_set_minimum.png">
    ///
    pub fn set_minimum(
        mut self,
        rule_type: ConditionalFormatType,
        value: impl Into<ConditionalFormatValue>,
    ) -> ConditionalFormat2ColorScale {
        let value = value.into();

        // This type of rule doesn't support strings.
        if value.is_string {
            return self;
        }

        // Check that percent and percentile are in range 0..100.
        if rule_type == ConditionalFormatType::Percent
            || rule_type == ConditionalFormatType::Percentile
        {
            if let Ok(num) = value.value.parse::<f64>() {
                if !(0.0..=100.0).contains(&num) {
                    eprintln!("Percent/percentile '{num}' must be in Excel range: 0..100.");
                    return self;
                }
            }
        }

        // The highest and lowest options cannot be set by the user.
        if rule_type != ConditionalFormatType::Lowest
            && rule_type != ConditionalFormatType::Highest
            && rule_type != ConditionalFormatType::Automatic
        {
            self.min_type = rule_type;
            self.min_value = value;
        }

        self
    }

    /// Set the type and value of the maximum in the 2 color scale.
    ///
    /// Set the maximum type (number, percent, formula or percentile) and value
    /// for a 2 color scale type of conditional format. By default the maximum
    /// is the highest value in the conditional formatting range.
    ///
    /// # Parameters
    ///
    /// * `rule_type`- A [`ConditionalFormatType`] enum value.
    /// * `value` - Any type that can convert into a [`ConditionalFormatValue`]
    ///   such as numbers, dates, times and formula ranges. String values are
    ///   ignored in this type of conditional format.
    ///
    pub fn set_maximum(
        mut self,
        rule_type: ConditionalFormatType,
        value: impl Into<ConditionalFormatValue>,
    ) -> ConditionalFormat2ColorScale {
        let value = value.into();

        // This type of rule doesn't support strings.
        if value.is_string {
            return self;
        }

        // Check that percent and percentile are in range 0..100.
        if rule_type == ConditionalFormatType::Percent
            || rule_type == ConditionalFormatType::Percentile
        {
            if let Ok(num) = value.value.parse::<f64>() {
                if !(0.0..=100.0).contains(&num) {
                    eprintln!("Percent/percentile '{num}' must be in Excel range: 0..100.");
                    return self;
                }
            }
        }

        // The highest and lowest options cannot be set by the user.
        if rule_type != ConditionalFormatType::Lowest
            && rule_type != ConditionalFormatType::Highest
            && rule_type != ConditionalFormatType::Automatic
        {
            self.max_type = rule_type;
            self.max_value = value;
        }

        self
    }

    /// Set the color of the minimum in the 2 color scale.
    ///
    /// Set the minimum color value for a 2 color scale type of conditional
    /// format. By default the minimum color is `#FFEF9C` (yellow).
    ///
    /// # Parameters
    ///
    /// * `color` - The color property defined by a [`Color`] enum value or a
    ///   type that implements the [`IntoColor`] trait.
    ///
    /// # Examples
    ///
    /// Example of adding a 2 color scale type conditional formatting to a worksheet
    /// with user defined minimum and maximum colors.
    ///
    /// ```
    /// # // This code is available in examples/doc_conditional_format_2color_set_color.rs
    /// #
    /// # use rust_xlsxwriter::{ConditionalFormat2ColorScale, Workbook, XlsxError};
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #     let worksheet = workbook.add_worksheet();
    /// #
    /// #     // Write the worksheet data.
    /// #     let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// #     worksheet.write(1, 1, "Default colors")?;
    /// #     worksheet.write(1, 3, "User colors")?;
    /// #     worksheet.write_column(2, 1, data)?;
    /// #     worksheet.write_column(2, 3, data)?;
    /// #
    ///     // Write a 2 color scale formats with standard Excel colors.
    ///     let conditional_format = ConditionalFormat2ColorScale::new();
    ///
    ///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
    ///
    ///     // Write a 2 color scale formats with user defined colors. This reverses the
    ///     // default colors.
    ///     let conditional_format = ConditionalFormat2ColorScale::new()
    ///         .set_minimum_color("63BE7B")
    ///         .set_maximum_color("FFEF9C");
    ///
    ///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
    /// #
    /// #     // Save the file.
    /// #     workbook.save("conditional_format.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img src="https://rustxlsxwriter.github.io/images/conditional_format_2color_set_color.png">
    ///
    pub fn set_minimum_color<T>(mut self, color: T) -> ConditionalFormat2ColorScale
    where
        T: IntoColor,
    {
        let color = color.new_color();
        if color.is_valid() {
            self.min_color = color;
        }

        self
    }

    /// Set the color of the maximum in the 2 color scale.
    ///
    /// Set the maximum color value for a 2 color scale type of conditional
    /// format. By default the maximum color is `#63BE7B` (green).
    ///
    /// # Parameters
    ///
    /// * `color` - The color property defined by a [`Color`] enum value or a
    ///   type that implements the [`IntoColor`] trait.
    ///
    pub fn set_maximum_color<T>(mut self, color: T) -> ConditionalFormat2ColorScale
    where
        T: IntoColor,
    {
        let color = color.new_color();
        if color.is_valid() {
            self.max_color = color;
        }

        self
    }

    // Validate the conditional format.
    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::unused_self)]
    pub(crate) fn validate(&self) -> Result<(), XlsxError> {
        Ok(())
    }

    //  Return the conditional format rule as an XML string.
    pub(crate) fn rule(
        &self,
        dxf_index: Option<u32>,
        priority: u32,
        _range: &str,
        _guid: &str,
    ) -> String {
        let mut writer = XMLWriter::new();
        let mut attributes = vec![("type", "colorScale".to_string())];

        // Set the format index if present.
        if let Some(dxf_index) = dxf_index {
            attributes.push(("dxfId", dxf_index.to_string()));
        }

        // Set the rule priority order.
        attributes.push(("priority", priority.to_string()));

        // Set the "Stop if True" property.
        if self.stop_if_true {
            attributes.push(("stopIfTrue", "1".to_string()));
        }

        // Write the rule.
        writer.xml_start_tag("cfRule", &attributes);
        writer.xml_start_tag_only("colorScale");

        Self::write_type(&mut writer, self.min_type, &self.min_value.value);
        Self::write_type(&mut writer, self.max_type, &self.max_value.value);

        Self::write_color(&mut writer, self.min_color);
        Self::write_color(&mut writer, self.max_color);

        writer.xml_end_tag("colorScale");

        writer.xml_end_tag("cfRule");

        writer.read_to_string()
    }

    // Write the <cfvo> element.
    fn write_type(writer: &mut XMLWriter, rule_type: ConditionalFormatType, value: &str) {
        let mut attributes = vec![];

        match rule_type {
            ConditionalFormatType::Lowest => {
                attributes.push(("type", "min".to_string()));
            }
            ConditionalFormatType::Number => {
                attributes.push(("type", "num".to_string()));
            }
            ConditionalFormatType::Percent => {
                attributes.push(("type", "percent".to_string()));
            }
            ConditionalFormatType::Formula => {
                attributes.push(("type", "formula".to_string()));
            }
            ConditionalFormatType::Percentile => {
                attributes.push(("type", "percentile".to_string()));
            }
            ConditionalFormatType::Highest => {
                attributes.push(("type", "max".to_string()));
            }
            ConditionalFormatType::Automatic => {}
        }

        attributes.push(("val", value.to_string()));

        writer.xml_empty_tag("cfvo", &attributes);
    }

    // Write the <color> element.
    fn write_color(writer: &mut XMLWriter, color: Color) {
        let attributes = [("rgb", color.argb_hex_value())];

        writer.xml_empty_tag("color", &attributes);
    }

    // Return an extended x14 rule for conditional formats that support it.
    #[allow(clippy::unused_self)]
    pub(crate) fn x14_rule(&self, _guid: &str) -> String {
        String::new()
    }
}

// -----------------------------------------------------------------------
// ConditionalFormat3ColorScale
// -----------------------------------------------------------------------

/// The `ConditionalFormat3ColorScale` struct represents a 3 Color Scale
/// conditional format.
///
/// `ConditionalFormat3ColorScale` is used to represent a Cell style conditional
/// format in Excel. A 3 Color Scale Cell conditional format shows a per cell
/// color gradient from the minimum value to the maximum value.
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_3color_intro.png">
///
/// For more information see [Working with Conditional
/// Formats](crate::conditional_format).
///
/// # Examples
///
/// Example of adding 3 color scale type conditional formatting to a worksheet.
/// Note, the colors in the first example (red to yellow to green) are the
/// default colors and could be omitted.
///
/// ```
/// # // This code is available in examples/doc_conditional_format_3color.rs
/// #
/// # use rust_xlsxwriter::{ConditionalFormat3ColorScale, Workbook, XlsxError};
/// #
/// # fn main() -> Result<(), XlsxError> {
/// #     // Create a new Excel file object.
/// #     let mut workbook = Workbook::new();
/// #     let worksheet = workbook.add_worksheet();
/// #
/// #     // Write the worksheet data.
/// #     let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// #     worksheet.write_column(2, 1, data)?;
/// #     worksheet.write_column(2, 3, data)?;
/// #     worksheet.write_column(2, 5, data)?;
/// #     worksheet.write_column(2, 7, data)?;
/// #     worksheet.write_column(2, 9, data)?;
/// #     worksheet.write_column(2, 11, data)?;
/// #
/// #     // Set the column widths for clarity.
/// #     for col_num in 0..=12u16 {
/// #         worksheet.set_column_width(col_num, 6)?;
/// #     }
/// #
///     // Write 3 color scale formats with standard Excel colors.
///     let conditional_format = ConditionalFormat3ColorScale::new()
///         .set_minimum_color("F8696B")
///         .set_midpoint_color("FFEB84")
///         .set_maximum_color("63BE7B");
///
///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
///
///     let conditional_format = ConditionalFormat3ColorScale::new()
///         .set_minimum_color("63BE7B")
///         .set_midpoint_color("FFEB84")
///         .set_maximum_color("F8696B");
///
///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
///
///     let conditional_format = ConditionalFormat3ColorScale::new()
///         .set_minimum_color("F8696B")
///         .set_midpoint_color("FCFCFF")
///         .set_maximum_color("63BE7B");
///
///     worksheet.add_conditional_format(2, 5, 11, 5, &conditional_format)?;
///
///     let conditional_format = ConditionalFormat3ColorScale::new()
///         .set_minimum_color("63BE7B")
///         .set_midpoint_color("FCFCFF")
///         .set_maximum_color("F8696B");
///
///     worksheet.add_conditional_format(2, 7, 11, 7, &conditional_format)?;
///
///     let conditional_format = ConditionalFormat3ColorScale::new()
///         .set_minimum_color("F8696B")
///         .set_midpoint_color("FCFCFF")
///         .set_maximum_color("5A8AC6");
///
///     worksheet.add_conditional_format(2, 9, 11, 9, &conditional_format)?;
///
///     let conditional_format = ConditionalFormat3ColorScale::new()
///         .set_minimum_color("5A8AC6")
///         .set_midpoint_color("FCFCFF")
///         .set_maximum_color("F8696B");
///
///     worksheet.add_conditional_format(2, 11, 11, 11, &conditional_format)?;
/// #
/// #     // Save the file.
/// #     workbook.save("conditional_format.xlsx")?;
/// #
/// #     Ok(())
/// # }
/// ```
///
/// This creates conditional format rules like this:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_3color_rules.png">
///
/// And the following output file:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_3color.png">
///
///
#[derive(Clone)]
pub struct ConditionalFormat3ColorScale {
    min_type: ConditionalFormatType,
    mid_type: ConditionalFormatType,
    max_type: ConditionalFormatType,
    min_value: ConditionalFormatValue,
    mid_value: ConditionalFormatValue,
    max_value: ConditionalFormatValue,
    min_color: Color,
    mid_color: Color,
    max_color: Color,

    multi_range: String,
    stop_if_true: bool,
    has_x14_extensions: bool,
    pub(crate) format: Option<Format>,
}

/// **Section 1**: The following methods are specific to `ConditionalFormat3ColorScale`.
impl ConditionalFormat3ColorScale {
    /// Create a new Cell conditional format struct.
    #[allow(clippy::new_without_default)]
    #[allow(clippy::unreadable_literal)] // For RGB colors.
    pub fn new() -> ConditionalFormat3ColorScale {
        ConditionalFormat3ColorScale {
            min_type: ConditionalFormatType::Lowest,
            mid_type: ConditionalFormatType::Percentile,
            max_type: ConditionalFormatType::Highest,
            min_value: 0.into(),
            mid_value: 50.into(),
            max_value: 0.into(),
            min_color: Color::RGB(0xF8696B),
            mid_color: Color::RGB(0xFFEB84),
            max_color: Color::RGB(0x63BE7B),
            multi_range: String::new(),
            stop_if_true: false,
            has_x14_extensions: false,
            format: None,
        }
    }

    /// Set the type and value of the minimum in the 3 color scale.
    ///
    /// Set the minimum type (number, percent, formula or percentile) and value
    /// for a 3 color scale type of conditional format. By default the minimum
    /// is the lowest value in the conditional formatting range.
    ///
    /// # Parameters
    ///
    /// * `rule_type`- A [`ConditionalFormatType`] enum value.
    /// * `value` - Any type that can convert into a [`ConditionalFormatValue`]
    ///   such as numbers, dates, times and formula ranges. String values are
    ///   ignored in this type of conditional format.
    ///
    /// # Examples
    ///
    /// Example of adding 3 color scale type conditional formatting to a worksheet
    /// with user defined minimum and maximum values.
    ///
    /// ```
    /// # // This code is available in examples/doc_conditional_format_3color_set_minimum.rs
    /// #
    /// # use rust_xlsxwriter::{
    /// #     ConditionalFormat3ColorScale, ConditionalFormatType, Workbook, XlsxError,
    /// # };
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #     let worksheet = workbook.add_worksheet();
    /// #
    /// #     // Write the worksheet data.
    /// #     let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// #     worksheet.write_column(2, 1, data)?;
    /// #     worksheet.write_column(2, 3, data)?;
    /// #
    ///     // Write a 3 color scale formats with standard Excel colors. The conditional
    ///     // format is applied from the lowest to the highest value.
    ///     let conditional_format = ConditionalFormat3ColorScale::new();
    ///
    ///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
    ///
    ///     // Write a 3 color scale formats with standard Excel colors but a user
    ///     // defined range. Values <= 3 will be shown with the minimum color while
    ///     // values >= 7 will be shown with the maximum color.
    ///     let conditional_format = ConditionalFormat3ColorScale::new()
    ///         .set_minimum(ConditionalFormatType::Number, 3)
    ///         .set_maximum(ConditionalFormatType::Number, 7);
    ///
    ///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
    /// #
    /// #     // Save the file.
    /// #     workbook.save("conditional_format.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img src="https://rustxlsxwriter.github.io/images/conditional_format_3color_set_minimum.png">
    ///
    pub fn set_minimum(
        mut self,
        rule_type: ConditionalFormatType,
        value: impl Into<ConditionalFormatValue>,
    ) -> ConditionalFormat3ColorScale {
        let value = value.into();

        // This type of rule doesn't support strings.
        if value.is_string {
            return self;
        }

        // Check that percent and percentile are in range 0..100.
        if rule_type == ConditionalFormatType::Percent
            || rule_type == ConditionalFormatType::Percentile
        {
            if let Ok(num) = value.value.parse::<f64>() {
                if !(0.0..=100.0).contains(&num) {
                    eprintln!("Percent/percentile '{num}' must be in Excel range: 0..100.");
                    return self;
                }
            }
        }

        // The highest and lowest options cannot be set by the user.
        if rule_type != ConditionalFormatType::Lowest
            && rule_type != ConditionalFormatType::Highest
            && rule_type != ConditionalFormatType::Automatic
        {
            self.min_type = rule_type;
            self.min_value = value;
        }

        self
    }

    /// Set the type and value of the midpoint in the 3 color scale.
    ///
    /// Set the midpoint type (number, percent, formula or percentile) and value
    /// for a 3 color scale type of conditional format. By default the midpoint
    /// is set as 50 percentile.
    ///
    /// # Parameters
    ///
    /// * `rule_type`- A [`ConditionalFormatType`] enum value.
    /// * `value` - Any type that can convert into a [`ConditionalFormatValue`]
    ///   such as numbers, dates, times and formula ranges. String values are
    ///   ignored in this type of conditional format.
    ///
    pub fn set_midpoint(
        mut self,
        rule_type: ConditionalFormatType,
        value: impl Into<ConditionalFormatValue>,
    ) -> ConditionalFormat3ColorScale {
        let value = value.into();

        // This type of rule doesn't support strings.
        if value.is_string {
            return self;
        }

        // Check that percent and percentile are in range 0..100.
        if rule_type == ConditionalFormatType::Percent
            || rule_type == ConditionalFormatType::Percentile
        {
            if let Ok(num) = value.value.parse::<f64>() {
                if !(0.0..=100.0).contains(&num) {
                    eprintln!("Percent/percentile '{num}' must be in Excel range: 0..100.");
                    return self;
                }
            }
        }

        // The highest and lowest options cannot be set by the user.
        if rule_type != ConditionalFormatType::Lowest
            && rule_type != ConditionalFormatType::Highest
            && rule_type != ConditionalFormatType::Automatic
        {
            self.mid_type = rule_type;
            self.mid_value = value;
        }

        self
    }

    /// Set the type and value of the maximum in the 3 color scale.
    ///
    /// Set the maximum type (number, percent, formula or percentile) and value
    /// for a 3 color scale type of conditional format. By default the maximum
    /// is the highest value in the conditional formatting range.
    ///
    /// # Parameters
    ///
    /// * `rule_type`- A [`ConditionalFormatType`] enum value.
    /// * `value` - Any type that can convert into a [`ConditionalFormatValue`]
    ///   such as numbers, dates, times and formula ranges. String values are
    ///   ignored in this type of conditional format.
    ///
    pub fn set_maximum(
        mut self,
        rule_type: ConditionalFormatType,
        value: impl Into<ConditionalFormatValue>,
    ) -> ConditionalFormat3ColorScale {
        let value = value.into();

        // This type of rule doesn't support strings.
        if value.is_string {
            return self;
        }

        // Check that percent and percentile are in range 0..100.
        if rule_type == ConditionalFormatType::Percent
            || rule_type == ConditionalFormatType::Percentile
        {
            if let Ok(num) = value.value.parse::<f64>() {
                if !(0.0..=100.0).contains(&num) {
                    eprintln!("Percent/percentile '{num}' must be in Excel range: 0..100.");
                    return self;
                }
            }
        }

        // The highest and lowest options cannot be set by the user.
        if rule_type != ConditionalFormatType::Lowest
            && rule_type != ConditionalFormatType::Highest
            && rule_type != ConditionalFormatType::Automatic
        {
            self.max_type = rule_type;
            self.max_value = value;
        }

        self
    }

    /// Set the color of the minimum in the 3 color scale.
    ///
    /// Set the minimum color value for a 3 color scale type of conditional
    /// format. By default the minimum color is `#FCFCFF` (white).
    ///
    /// # Parameters
    ///
    /// * `color` - The color property defined by a [`Color`] enum value or a
    ///   type that implements the [`IntoColor`] trait.
    ///
    /// # Examples
    ///
    /// Example of adding 3 color scale type conditional formatting to a worksheet
    /// with user defined minimum, midpoint and maximum colors.
    ///
    /// ```
    /// # // This code is available in examples/doc_conditional_format_3color_set_color.rs
    /// #
    /// # use rust_xlsxwriter::{ConditionalFormat3ColorScale, Workbook, XlsxError};
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #     let worksheet = workbook.add_worksheet();
    /// #
    /// #     // Write the worksheet data.
    /// #     let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// #     worksheet.write(1, 1, "Default colors")?;
    /// #     worksheet.write(1, 3, "User colors")?;
    /// #     worksheet.write_column(2, 1, data)?;
    /// #     worksheet.write_column(2, 3, data)?;
    /// #
    ///     // Write a 3 color scale formats with standard Excel colors.
    ///     let conditional_format = ConditionalFormat3ColorScale::new();
    ///
    ///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
    ///
    ///     // Write a 3 color scale formats with user defined colors. This reverses the
    ///     // default colors.
    ///     let conditional_format = ConditionalFormat3ColorScale::new()
    ///         .set_minimum_color("63BE7B")
    ///         .set_midpoint_color("FFEB84")
    ///         .set_maximum_color("F8696B");
    ///
    ///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
    /// #
    /// #     // Save the file.
    /// #     workbook.save("conditional_format.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img src="https://rustxlsxwriter.github.io/images/conditional_format_3color_set_color.png">
    ///
    pub fn set_minimum_color<T>(mut self, color: T) -> ConditionalFormat3ColorScale
    where
        T: IntoColor,
    {
        let color = color.new_color();
        if color.is_valid() {
            self.min_color = color;
        }

        self
    }

    /// Set the color of the midpoint in the 3 color scale.
    ///
    /// Set the midpoint color value for a 3 color scale type of conditional
    /// format. By default the midpoint color is `#FFEB84` (Yellow).
    ///
    /// # Parameters
    ///
    /// * `color` - The color property defined by a [`Color`] enum value or a
    ///   type that implements the [`IntoColor`] trait.
    ///
    pub fn set_midpoint_color<T>(mut self, color: T) -> ConditionalFormat3ColorScale
    where
        T: IntoColor,
    {
        let color = color.new_color();
        if color.is_valid() {
            self.mid_color = color;
        }

        self
    }

    /// Set the color of the maximum in the 3 color scale.
    ///
    /// Set the maximum color value for a 3 color scale type of conditional
    /// format. By default the maximum color is `#63BE7B` (green).
    ///
    /// # Parameters
    ///
    /// * `color` - The color property defined by a [`Color`] enum value or a
    ///   type that implements the [`IntoColor`] trait.
    ///
    pub fn set_maximum_color<T>(mut self, color: T) -> ConditionalFormat3ColorScale
    where
        T: IntoColor,
    {
        let color = color.new_color();
        if color.is_valid() {
            self.max_color = color;
        }

        self
    }

    // Validate the conditional format.
    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::unused_self)]
    pub(crate) fn validate(&self) -> Result<(), XlsxError> {
        Ok(())
    }

    //  Return the conditional format rule as an XML string.
    pub(crate) fn rule(
        &self,
        dxf_index: Option<u32>,
        priority: u32,
        _range: &str,
        _guid: &str,
    ) -> String {
        let mut writer = XMLWriter::new();
        let mut attributes = vec![("type", "colorScale".to_string())];

        // Set the format index if present.
        if let Some(dxf_index) = dxf_index {
            attributes.push(("dxfId", dxf_index.to_string()));
        }

        // Set the rule priority order.
        attributes.push(("priority", priority.to_string()));

        // Set the "Stop if True" property.
        if self.stop_if_true {
            attributes.push(("stopIfTrue", "1".to_string()));
        }

        // Write the rule.
        writer.xml_start_tag("cfRule", &attributes);
        writer.xml_start_tag_only("colorScale");

        Self::write_type(&mut writer, self.min_type, &self.min_value.value);
        Self::write_type(&mut writer, self.mid_type, &self.mid_value.value);
        Self::write_type(&mut writer, self.max_type, &self.max_value.value);

        Self::write_color(&mut writer, self.min_color);
        Self::write_color(&mut writer, self.mid_color);
        Self::write_color(&mut writer, self.max_color);

        writer.xml_end_tag("colorScale");

        writer.xml_end_tag("cfRule");

        writer.read_to_string()
    }

    // Write the <cfvo> element.
    fn write_type(writer: &mut XMLWriter, rule_type: ConditionalFormatType, value: &str) {
        let mut attributes = vec![];

        match rule_type {
            ConditionalFormatType::Lowest => {
                attributes.push(("type", "min".to_string()));
            }
            ConditionalFormatType::Number => {
                attributes.push(("type", "num".to_string()));
            }
            ConditionalFormatType::Percent => {
                attributes.push(("type", "percent".to_string()));
            }
            ConditionalFormatType::Formula => {
                attributes.push(("type", "formula".to_string()));
            }
            ConditionalFormatType::Percentile => {
                attributes.push(("type", "percentile".to_string()));
            }
            ConditionalFormatType::Highest => {
                attributes.push(("type", "max".to_string()));
            }
            ConditionalFormatType::Automatic => {}
        }

        attributes.push(("val", value.to_string()));

        writer.xml_empty_tag("cfvo", &attributes);
    }

    // Write the <color> element.
    fn write_color(writer: &mut XMLWriter, color: Color) {
        let attributes = [("rgb", color.argb_hex_value())];

        writer.xml_empty_tag("color", &attributes);
    }

    // Return an extended x14 rule for conditional formats that support it.
    #[allow(clippy::unused_self)]
    pub(crate) fn x14_rule(&self, _guid: &str) -> String {
        String::new()
    }
}

// -----------------------------------------------------------------------
// ConditionalFormatDataBar
// -----------------------------------------------------------------------

/// The `ConditionalFormatDataBar` struct represents a Data Bar conditional
/// format.
///
/// `ConditionalFormatDataBar` is used to represent a Cell style conditional
/// format in Excel. A Data Bar Cell conditional format shows a per cell color
/// gradient from the minimum value to the maximum value.
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_databar_intro.png">
///
/// The options that can be applied to a data bar conditional format in Excel
/// are shown in the image below.
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_databar_intro2.png">
///
/// The methods to replicate these options are explained in the following
/// documentation.
///
/// For more information see [Working with Conditional
/// Formats](crate::conditional_format).
///
/// # Examples
///
/// Example of adding data bar type conditional formatting to a worksheet.
///
/// ```
/// # // This code is available in examples/doc_conditional_format_databar.rs
/// #
/// # use rust_xlsxwriter::{
/// #     ConditionalFormatDataBar, ConditionalFormatDataBarDirection, Workbook, XlsxError,
/// # };
/// #
/// # fn main() -> Result<(), XlsxError> {
/// #     // Create a new Excel file object.
/// #     let mut workbook = Workbook::new();
/// #     let worksheet = workbook.add_worksheet();
/// #
/// #     // Write some captions.
/// #     worksheet.write(1, 1, "Default")?;
/// #     worksheet.write(1, 3, "Default negative")?;
/// #     worksheet.write(1, 5, "User color")?;
/// #     worksheet.write(1, 7, "Changed direction")?;
/// #
/// #     // Write the worksheet data.
/// #     let data1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// #     let data2 = [6, 4, 2, -2, -4, -6, -4, -2, 2, 4];
/// #     worksheet.write_column(2, 1, data1)?;
/// #     worksheet.write_column(2, 3, data2)?;
/// #     worksheet.write_column(2, 5, data1)?;
/// #     worksheet.write_column(2, 7, data1)?;
/// #
///     // Write a standard Excel data bar.
///     let conditional_format = ConditionalFormatDataBar::new();
///
///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
///
///     // Write a standard Excel data bar with negative data
///     let conditional_format = ConditionalFormatDataBar::new();
///
///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
///
///     // Write a data bar with a user defined fill color.
///     let conditional_format = ConditionalFormatDataBar::new().set_fill_color("009933");
///
///     worksheet.add_conditional_format(2, 5, 11, 5, &conditional_format)?;
///
///     // Write a data bar with the direction changed.
///     let conditional_format = ConditionalFormatDataBar::new()
///         .set_direction(ConditionalFormatDataBarDirection::RightToLeft);
///
///     worksheet.add_conditional_format(2, 7, 11, 7, &conditional_format)?;
///
/// #     // Save the file.
/// #     workbook.save("conditional_format.xlsx")?;
/// #
/// #     Ok(())
/// # }
/// ```
///
///
/// This creates conditional format rules like this:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_databar_rules.png">
///
/// And the following output file:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_databar.png">
///
///
#[derive(Clone)]
pub struct ConditionalFormatDataBar {
    min_type: ConditionalFormatType,
    max_type: ConditionalFormatType,
    min_value: ConditionalFormatValue,
    max_value: ConditionalFormatValue,
    fill_color: Color,
    border_color: Color,
    negative_fill_color: Color,
    negative_border_color: Color,
    axis_color: Color,
    border_off: bool,
    solid_bar: bool,
    bar_only: bool,
    direction: ConditionalFormatDataBarDirection,
    axis_position: ConditionalFormatDataBarAxisPosition,

    multi_range: String,
    stop_if_true: bool,
    has_x14_extensions: bool,
    pub(crate) format: Option<Format>,
}

/// **Section 1**: The following methods are specific to `ConditionalFormatDataBar`.
impl ConditionalFormatDataBar {
    /// Create a new Cell conditional format struct.
    #[allow(clippy::new_without_default)]
    #[allow(clippy::unreadable_literal)] // For RGB colors.
    pub fn new() -> ConditionalFormatDataBar {
        ConditionalFormatDataBar {
            min_type: ConditionalFormatType::Automatic,
            max_type: ConditionalFormatType::Automatic,
            min_value: 0.into(),
            max_value: 0.into(),
            fill_color: Color::RGB(0x638EC6),
            border_color: Color::RGB(0x638EC6),
            negative_fill_color: Color::RGB(0xFF0000),
            negative_border_color: Color::RGB(0xFF0000),
            axis_color: Color::RGB(0x000000),
            border_off: false,
            solid_bar: false,
            bar_only: false,
            direction: ConditionalFormatDataBarDirection::Context,
            axis_position: ConditionalFormatDataBarAxisPosition::Automatic,

            multi_range: String::new(),
            stop_if_true: false,
            has_x14_extensions: true,
            format: None,
        }
    }

    /// Set the type and value of the minimum in the data bar.
    ///
    /// Set the minimum type (number, percent, formula or percentile) and value
    /// for a data bar conditional format. By default the minimum is set
    /// automatically.
    ///
    /// # Parameters
    ///
    /// * `rule_type`- A [`ConditionalFormatType`] enum value.
    /// * `value` - Any type that can convert into a [`ConditionalFormatValue`]
    ///   such as numbers, dates, times and formula ranges. String values are
    ///   ignored in this type of conditional format.
    ///
    ///
    /// # Examples
    ///
    /// Example of adding a data bar type conditional formatting to a worksheet
    /// with user defined minimum and maximum values.
    ///
    /// ```
    /// # // This code is available in examples/doc_conditional_format_databar_set_minimum.rs
    /// #
    /// # use rust_xlsxwriter::{ConditionalFormatDataBar, ConditionalFormatType, Workbook, XlsxError};
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #     let worksheet = workbook.add_worksheet();
    /// #
    /// #     // Write the worksheet data.
    /// #     let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// #     worksheet.write_column(2, 1, data)?;
    /// #     worksheet.write_column(2, 3, data)?;
    /// #
    ///     // Write a standard Excel data bar. The conditional format is applied over
    ///     // the full range of values from minimum to maximum.
    ///     let conditional_format = ConditionalFormatDataBar::new();
    ///
    ///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
    ///
    ///     // Write a data bar a user defined range. Values <= 3 are shown with zero
    ///     // bar width while values >= 7 are shown with the maximum bar width.
    ///     let conditional_format = ConditionalFormatDataBar::new()
    ///         .set_minimum(ConditionalFormatType::Number, 3)
    ///         .set_maximum(ConditionalFormatType::Number, 7);
    ///
    ///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
    /// #
    /// #     // Save the file.
    /// #     workbook.save("conditional_format.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img
    /// src="https://rustxlsxwriter.github.io/images/conditional_format_databar_set_minimum.png">
    ///
    pub fn set_minimum(
        mut self,
        rule_type: ConditionalFormatType,
        value: impl Into<ConditionalFormatValue>,
    ) -> ConditionalFormatDataBar {
        let value = value.into();

        // This type of rule doesn't support strings.
        if value.is_string {
            return self;
        }

        // Check that percent and percentile are in range 0..100.
        if rule_type == ConditionalFormatType::Percent
            || rule_type == ConditionalFormatType::Percentile
        {
            if let Ok(num) = value.value.parse::<f64>() {
                if !(0.0..=100.0).contains(&num) {
                    eprintln!("Percent/percentile '{num}' must be in Excel range: 0..100.");
                    return self;
                }
            }
        }
        // The highest option cannot be set for the minimum.
        if rule_type != ConditionalFormatType::Highest {
            self.min_type = rule_type;
            self.min_value = value;
        }

        self
    }

    /// Set the type and value of the maximum in the data bar.
    ///
    /// Set the maximum type (number, percent, formula or percentile) and value
    /// for a data bar conditional format. By default the maximum is set
    /// automatically.
    ///
    /// See the example above.
    ///
    /// # Parameters
    ///
    /// * `rule_type`- A [`ConditionalFormatType`] enum value.
    /// * `value` - Any type that can convert into a [`ConditionalFormatValue`]
    ///   such as numbers, dates, times and formula ranges. String values are
    ///   ignored in this type of conditional format.
    ///
    pub fn set_maximum(
        mut self,
        rule_type: ConditionalFormatType,
        value: impl Into<ConditionalFormatValue>,
    ) -> ConditionalFormatDataBar {
        let value = value.into();

        // This type of rule doesn't support strings.
        if value.is_string {
            return self;
        }

        // Check that percent and percentile are in range 0..100.
        if rule_type == ConditionalFormatType::Percent
            || rule_type == ConditionalFormatType::Percentile
        {
            if let Ok(num) = value.value.parse::<f64>() {
                if !(0.0..=100.0).contains(&num) {
                    eprintln!("Percent/percentile '{num}' must be in Excel range: 0..100.");
                    return self;
                }
            }
        }

        // The lowest option cannot be set for the maximum.
        if rule_type != ConditionalFormatType::Lowest {
            self.max_type = rule_type;
            self.max_value = value;
        }

        self
    }

    /// Set the color of the fill in the data bar.
    ///
    /// Set the fill color for a data bar conditional format. By default the
    /// data bar color is `#638EC6` (blue).
    ///
    /// # Parameters
    ///
    /// * `color` - The color property defined by a [`Color`] enum value or a
    ///   type that implements the [`IntoColor`] trait.
    ///
    /// # Examples
    ///
    /// Example of adding a data bar type conditional formatting to a worksheet
    /// with user defined fill color.
    ///
    /// ```
    /// # // This code is available in examples/doc_conditional_format_databar_set_fill_color.rs
    /// #
    /// # use rust_xlsxwriter::{ConditionalFormatDataBar, Workbook, XlsxError};
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #     let worksheet = workbook.add_worksheet();
    /// #
    /// #     // Write the worksheet data.
    /// #     let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// #     worksheet.write_column(2, 1, data)?;
    /// #     worksheet.write_column(2, 3, data)?;
    /// #
    ///     // Write a standard Excel data bar.
    ///     let conditional_format = ConditionalFormatDataBar::new();
    ///
    ///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
    ///
    ///     // Write a data bar with a user defined fill color.
    ///     let conditional_format = ConditionalFormatDataBar::new().set_fill_color("009933");
    ///
    ///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
    ///
    /// #     // Save the file.
    /// #     workbook.save("conditional_format.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img
    /// src="https://rustxlsxwriter.github.io/images/conditional_format_databar_set_fill_color.png">
    ///
    pub fn set_fill_color<T>(mut self, color: T) -> ConditionalFormatDataBar
    where
        T: IntoColor,
    {
        let color = color.new_color();
        if color.is_valid() {
            self.fill_color = color;
            self.border_color = color;
        }

        self
    }

    /// Set the color of the border in the data bar.
    ///
    /// Set the border color for a data bar conditional format. By default the
    /// border is the same color as the data bar: `#638EC6` (blue).
    ///
    /// # Parameters
    ///
    /// * `color` - The color property defined by a [`Color`] enum value or a
    ///   type that implements the [`IntoColor`] trait.
    ///
    /// # Examples
    ///
    /// Example of adding a data bar type conditional formatting to a worksheet
    /// with user defined border color.
    ///
    /// ```
    /// # // This code is available in examples/doc_conditional_format_databar_set_border_color.rs
    /// #
    /// # use rust_xlsxwriter::{ConditionalFormatDataBar, Workbook, XlsxError};
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #     let worksheet = workbook.add_worksheet();
    /// #
    /// #     // Write the worksheet data.
    /// #     let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// #     worksheet.write_column(2, 1, data)?;
    /// #     worksheet.write_column(2, 3, data)?;
    /// #
    ///     // Write a standard Excel data bar.
    ///     let conditional_format = ConditionalFormatDataBar::new();
    ///
    ///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
    ///
    ///     // Write a data bar with a user defined border color.
    ///     let conditional_format = ConditionalFormatDataBar::new().set_border_color("FF0000");
    ///
    ///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
    ///
    /// #     // Save the file.
    /// #     workbook.save("conditional_format.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img
    /// src="https://rustxlsxwriter.github.io/images/conditional_format_databar_set_border_color.png">
    ///
    pub fn set_border_color<T>(mut self, color: T) -> ConditionalFormatDataBar
    where
        T: IntoColor,
    {
        let color = color.new_color();
        if color.is_valid() {
            self.border_color = color;
        }

        self
    }

    /// Set the color of the fill for negative values in the data bar.
    ///
    /// Set the fill color for negative values in a data bar conditional format.
    /// By default the negative data bar color is `#FF0000` (red).
    ///
    /// # Parameters
    ///
    /// * `color` - The color property defined by a [`Color`] enum value or a
    ///   type that implements the [`IntoColor`] trait.
    ///
    /// # Examples
    ///
    /// Example of adding a data bar type conditional formatting to a worksheet
    /// with user defined negative fill color.
    ///
    /// ```
    /// # // This code is available in examples/doc_conditional_format_databar_set_negative_fill_color.rs
    /// #
    /// # use rust_xlsxwriter::{ConditionalFormatDataBar, Workbook, XlsxError};
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #     let worksheet = workbook.add_worksheet();
    /// #
    /// #     // Write the worksheet data.
    /// #     let data = [6, 4, 2, -2, -4, -6, -4, -2, 2, 4];
    /// #     worksheet.write_column(2, 1, data)?;
    /// #     worksheet.write_column(2, 3, data)?;
    /// #
    ///     // Write a standard Excel data bar.
    ///     let conditional_format = ConditionalFormatDataBar::new();
    ///
    ///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
    ///
    ///     // Write a data bar with a user defined negative fill color.
    ///     let conditional_format = ConditionalFormatDataBar::new().set_negative_fill_color("009933");
    ///
    ///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
    ///
    /// #     // Save the file.
    /// #     workbook.save("conditional_format.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img
    /// src="https://rustxlsxwriter.github.io/images/conditional_format_databar_set_negative_fill_color.png">
    ///
    pub fn set_negative_fill_color<T>(mut self, color: T) -> ConditionalFormatDataBar
    where
        T: IntoColor,
    {
        let color = color.new_color();
        if color.is_valid() {
            self.negative_fill_color = color;
            self.negative_border_color = color;
        }

        self
    }

    /// Set the color of the border for negative values in the data bar.
    ///
    /// Set the border color for negative values in a data bar conditional
    /// format. By default the border is the same color as the data bar:
    /// is `#FF0000` (red).
    ///
    /// # Parameters
    ///
    /// * `color` - The color property defined by a [`Color`] enum value or a
    ///   type that implements the [`IntoColor`] trait.
    ///
    /// # Examples
    ///
    /// Example of adding a data bar type conditional formatting to a worksheet with
    /// user defined negative border color.
    ///
    /// ```
    /// # // This code is available in examples/doc_conditional_format_databar_set_negative_border_color.rs
    /// #
    /// # use rust_xlsxwriter::{ConditionalFormatDataBar, Workbook, XlsxError};
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #     let worksheet = workbook.add_worksheet();
    /// #
    /// #     // Write the worksheet data.
    /// #     let data = [6, 4, 2, -2, -4, -6, -4, -2, 2, 4];
    /// #     worksheet.write_column(2, 1, data)?;
    /// #     worksheet.write_column(2, 3, data)?;
    /// #
    ///     // Write a standard Excel data bar.
    ///     let conditional_format = ConditionalFormatDataBar::new();
    ///
    ///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
    ///
    ///     // Write a data bar with a user defined negative border color.
    ///     let conditional_format = ConditionalFormatDataBar::new()
    ///         .set_negative_border_color("000000");
    ///
    ///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
    ///
    /// #     // Save the file.
    /// #     workbook.save("conditional_format.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img src="https://rustxlsxwriter.github.io/images/conditional_format_databar_set_negative_border_color.png">
    ///
    pub fn set_negative_border_color<T>(mut self, color: T) -> ConditionalFormatDataBar
    where
        T: IntoColor,
    {
        let color = color.new_color();
        if color.is_valid() {
            self.negative_border_color = color;
        }

        self
    }

    /// Set the data bar fill to solid.
    ///
    /// By default Excel uses a gradient fill for data bar conditional formats.
    /// This option can be used to turn on a solid fill.
    ///
    /// # Parameters
    ///
    /// * `enable` - Turn the property on/off. It is off by default.
    ///
    /// # Examples
    ///
    /// Example of adding a data bar type conditional formatting to a worksheet
    /// with a solid (non-gradient) style bar.
    ///
    /// ```
    /// # // This code is available in examples/doc_conditional_format_databar_set_solid_fill.rs
    /// #
    /// # use rust_xlsxwriter::{ConditionalFormatDataBar, Workbook, XlsxError};
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #     let worksheet = workbook.add_worksheet();
    /// #
    /// #     // Write the worksheet data.
    /// #     let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// #     worksheet.write_column(2, 1, data)?;
    /// #     worksheet.write_column(2, 3, data)?;
    /// #
    ///     // Write a standard Excel data bar.
    ///     let conditional_format = ConditionalFormatDataBar::new();
    ///
    ///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
    ///
    ///     // Write a data bar with a solid fill.
    ///     let conditional_format = ConditionalFormatDataBar::new().set_solid_fill(true);
    ///
    ///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
    ///
    /// #     // Save the file.
    /// #     workbook.save("conditional_format.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img
    /// src="https://rustxlsxwriter.github.io/images/conditional_format_databar_set_solid_fill.png">
    ///
    pub fn set_solid_fill(mut self, enable: bool) -> ConditionalFormatDataBar {
        self.solid_bar = enable;
        self
    }

    /// Turn off the border for a data bar conditional format.
    ///
    /// # Parameters
    ///
    /// * `enable` - Turn the property on/off. It is on by default.
    ///
    /// # Examples
    ///
    /// Example of adding a data bar type conditional formatting to a worksheet
    /// without a border.
    ///
    /// ```
    /// # // This code is available in examples/doc_conditional_format_databar_set_border_off.rs
    /// #
    /// # use rust_xlsxwriter::{ConditionalFormatDataBar, Workbook, XlsxError};
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #     let worksheet = workbook.add_worksheet();
    /// #
    /// #     // Write the worksheet data.
    /// #     let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// #     worksheet.write_column(2, 1, data)?;
    /// #     worksheet.write_column(2, 3, data)?;
    /// #
    ///     // Write a standard Excel data bar.
    ///     let conditional_format = ConditionalFormatDataBar::new();
    ///
    ///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
    ///
    ///     // Write a data bar without a border.
    ///     let conditional_format = ConditionalFormatDataBar::new().set_border_off(true);
    ///
    ///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
    ///
    /// #     // Save the file.
    /// #     workbook.save("conditional_format.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img src="https://rustxlsxwriter.github.io/images/conditional_format_databar_set_border_off.png">
    ///
    pub fn set_border_off(mut self, enable: bool) -> ConditionalFormatDataBar {
        self.border_off = enable;
        self
    }

    /// Set the direction of the data bar conditional format.
    ///
    /// Set the data bar conditional format direction to "Right to left", "Left
    /// to right" or "Context" (the default).
    ///
    /// # Parameters
    ///
    /// * `direction`- A [`ConditionalFormatDataBarDirection`] enum value.
    ///
    /// # Examples
    ///
    /// Example of adding a data bar type conditional formatting to a worksheet
    /// without a border
    ///
    /// ```
    /// # // This code is available in examples/doc_conditional_format_databar_set_direction.rs
    /// #
    /// # use rust_xlsxwriter::{
    /// #     ConditionalFormatDataBar, ConditionalFormatDataBarDirection, Workbook, XlsxError,
    /// # };
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #     let worksheet = workbook.add_worksheet();
    /// #
    /// #     // Write the worksheet data.
    /// #     let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// #     worksheet.write_column(2, 1, data)?;
    /// #     worksheet.write_column(2, 3, data)?;
    /// #
    ///     // Write a standard Excel data bar.
    ///     let conditional_format = ConditionalFormatDataBar::new();
    ///
    ///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
    ///
    ///     // Write a data bar with the direction changed.
    ///     let conditional_format = ConditionalFormatDataBar::new()
    ///         .set_direction(ConditionalFormatDataBarDirection::RightToLeft);
    ///
    ///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
    ///
    /// #     // Save the file.
    /// #     workbook.save("conditional_format.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img src="https://rustxlsxwriter.github.io/images/conditional_format_databar_set_direction.png">
    ///
    pub fn set_direction(
        mut self,
        direction: ConditionalFormatDataBarDirection,
    ) -> ConditionalFormatDataBar {
        self.direction = direction;

        self
    }

    /// Hide the values and only show the data bars.
    ///
    /// # Parameters
    ///
    /// * `enable` - Turn the property on/off. It is off by default.
    ///
    /// # Examples
    ///
    /// Example of adding a data bar type conditional formatting to a worksheet with
    /// the bar only and with the data hidden.
    ///
    /// ```
    /// # // This code is available in examples/doc_conditional_format_databar_set_bar_only.rs
    /// #
    /// # use rust_xlsxwriter::{ConditionalFormatDataBar, Workbook, XlsxError};
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #     let worksheet = workbook.add_worksheet();
    /// #
    /// #     // Write the worksheet data.
    /// #     let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// #     worksheet.write_column(2, 1, data)?;
    /// #     worksheet.write_column(2, 3, data)?;
    /// #
    ///     // Write a standard Excel data bar.
    ///     let conditional_format = ConditionalFormatDataBar::new();
    ///
    ///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
    ///
    ///     // Write a data bar with the data hidden and bars shown only.
    ///     let conditional_format = ConditionalFormatDataBar::new().set_bar_only(true);
    ///
    ///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
    ///
    /// #     // Save the file.
    /// #     workbook.save("conditional_format.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img src="https://rustxlsxwriter.github.io/images/conditional_format_databar_set_bar_only.png">
    ///
    ///
    pub fn set_bar_only(mut self, enable: bool) -> ConditionalFormatDataBar {
        self.bar_only = enable;

        self
    }

    /// Set the position of the axis in a data bar.
    ///
    /// The position can be set to midpoint or turned off.
    ///
    /// # Parameters
    ///
    /// * `position`- A [`ConditionalFormatDataBarAxisPosition`] enum value.
    ///
    /// # Examples
    ///
    /// Example of adding a data bar type conditional formatting to a worksheet
    /// with different axis positions.
    ///
    /// ```
    /// # // This code is available in examples/doc_conditional_format_databar_set_axis_position.rs
    /// #
    /// # use rust_xlsxwriter::{
    /// #     ConditionalFormatDataBar, ConditionalFormatDataBarAxisPosition, Workbook, XlsxError,
    /// # };
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #     let worksheet = workbook.add_worksheet();
    /// #
    /// #     // Write the worksheet data.
    /// #     let data1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// #     let data2 = [6, 4, 2, -2, -4, -6, -4, -2, 2, 4];
    /// #     worksheet.write_column(2, 1, data1)?;
    /// #     worksheet.write_column(2, 3, data1)?;
    /// #     worksheet.write_column(2, 5, data2)?;
    /// #     worksheet.write_column(2, 7, data2)?;
    /// #
    ///     // Write a standard Excel data bar.
    ///     let conditional_format = ConditionalFormatDataBar::new();
    ///
    ///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
    ///
    ///     // Write a data bar with a midpoint axis.
    ///     let conditional_format = ConditionalFormatDataBar::new()
    ///         .set_axis_position(ConditionalFormatDataBarAxisPosition::Midpoint);
    ///
    ///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
    ///
    ///     // Write a standard Excel data bar with negative data
    ///     let conditional_format = ConditionalFormatDataBar::new();
    ///
    ///     worksheet.add_conditional_format(2, 5, 11, 5, &conditional_format)?;
    ///
    ///     // Write a data bar without an axis.
    ///     let conditional_format = ConditionalFormatDataBar::new()
    ///         .set_axis_position(ConditionalFormatDataBarAxisPosition::None);
    ///
    ///     worksheet.add_conditional_format(2, 7, 11, 7, &conditional_format)?;
    ///
    /// #     // Save the file.
    /// #     workbook.save("conditional_format.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img src="https://rustxlsxwriter.github.io/images/conditional_format_databar_set_axis_position.png">
    ///
    pub fn set_axis_position(
        mut self,
        position: ConditionalFormatDataBarAxisPosition,
    ) -> ConditionalFormatDataBar {
        self.axis_position = position;

        self
    }

    /// Set the color of the axis in the data bar.
    ///
    /// Set the axis color for a data bar conditional format. By default the
    /// axis color is `#000000` (black).
    ///
    /// # Parameters
    ///
    /// * `color` - The color property defined by a [`Color`] enum value or a
    ///   type that implements the [`IntoColor`] trait.
    ///
    ///
    /// # Examples
    ///
    /// Example of adding a data bar type conditional formatting to a worksheet with
    /// a user defined axis color.
    ///
    /// ```
    /// # // This code is available in examples/doc_conditional_format_databar_set_axis_color.rs
    /// #
    /// # use rust_xlsxwriter::{ConditionalFormatDataBar, Workbook, XlsxError};
    /// #
    /// # fn main() -> Result<(), XlsxError> {
    /// #     // Create a new Excel file object.
    /// #     let mut workbook = Workbook::new();
    /// #     let worksheet = workbook.add_worksheet();
    /// #
    /// #     // Write the worksheet data.
    /// #     let data = [6, 4, 2, -2, -4, -6, -4, -2, 2, 4];
    /// #     worksheet.write_column(2, 1, data)?;
    /// #     worksheet.write_column(2, 3, data)?;
    /// #
    ///     // Write a standard Excel data bar.
    ///     let conditional_format = ConditionalFormatDataBar::new();
    ///
    ///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
    ///
    ///     // Write a data bar with a user defined axis color.
    ///     let conditional_format = ConditionalFormatDataBar::new().set_axis_color("FF0000");
    ///
    ///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
    ///
    /// #     // Save the file.
    /// #     workbook.save("conditional_format.xlsx")?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img src="https://rustxlsxwriter.github.io/images/conditional_format_databar_set_axis_color.png">
    ///
    pub fn set_axis_color<T>(mut self, color: T) -> ConditionalFormatDataBar
    where
        T: IntoColor,
    {
        let color = color.new_color();
        if color.is_valid() {
            self.axis_color = color;
        }

        self
    }

    /// Set the data bar format to the original Excel 2007 style.
    ///
    /// The original Excel 2007 style was simpler than the post Excel 2010 style
    /// and had very limited configuration options.
    ///
    /// This is implemented for backward compatibility and for testing but is
    /// unlikely be be required by the end user.
    ///
    #[doc(hidden)]
    pub fn use_classic_style(mut self) -> ConditionalFormatDataBar {
        self.has_x14_extensions = false;

        if self.min_type == ConditionalFormatType::Automatic {
            self.min_type = ConditionalFormatType::Lowest;
        }
        if self.max_type == ConditionalFormatType::Automatic {
            self.max_type = ConditionalFormatType::Highest;
        }

        self
    }

    // Validate the conditional format.
    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::unused_self)]
    pub(crate) fn validate(&self) -> Result<(), XlsxError> {
        Ok(())
    }

    //  Return the conditional format rule as an XML string.
    pub(crate) fn rule(
        &self,
        dxf_index: Option<u32>,
        priority: u32,
        _range: &str,
        guid: &str,
    ) -> String {
        let mut writer = XMLWriter::new();
        let mut attributes = vec![("type", "dataBar".to_string())];

        // Set the format index if present.
        if let Some(dxf_index) = dxf_index {
            attributes.push(("dxfId", dxf_index.to_string()));
        }

        // Set the rule priority order.
        attributes.push(("priority", priority.to_string()));

        // Set the "Stop if True" property.
        if self.stop_if_true {
            attributes.push(("stopIfTrue", "1".to_string()));
        }

        // Write the rule.
        writer.xml_start_tag("cfRule", &attributes);

        let mut attributes = vec![];

        if self.bar_only {
            attributes.push(("showValue", "0".to_string()));
        }

        writer.xml_start_tag("dataBar", &attributes);

        Self::write_type(
            &mut writer,
            self.min_type,
            &self.min_value.value,
            self.has_x14_extensions,
            false,
        );

        Self::write_type(
            &mut writer,
            self.max_type,
            &self.max_value.value,
            self.has_x14_extensions,
            true,
        );

        Self::write_color(&mut writer, "color", self.fill_color);

        writer.xml_end_tag("dataBar");

        if self.has_x14_extensions {
            // Write the extLst element.
            Self::write_extension_list(&mut writer, guid);
        }

        writer.xml_end_tag("cfRule");

        writer.read_to_string()
    }

    // Write the <cfvo> element.
    fn write_type(
        writer: &mut XMLWriter,
        rule_type: ConditionalFormatType,
        value: &str,
        has_x14_extensions: bool,
        is_max: bool,
    ) {
        let mut attributes = vec![];

        match rule_type {
            ConditionalFormatType::Automatic => {
                if is_max {
                    attributes.push(("type", "max".to_string()));
                } else {
                    attributes.push(("type", "min".to_string()));
                }
            }
            ConditionalFormatType::Lowest => {
                attributes.push(("type", "min".to_string()));
                if !has_x14_extensions {
                    attributes.push(("val", value.to_string()));
                }
            }
            ConditionalFormatType::Number => {
                attributes.push(("type", "num".to_string()));
                attributes.push(("val", value.to_string()));
            }
            ConditionalFormatType::Percent => {
                attributes.push(("type", "percent".to_string()));
                attributes.push(("val", value.to_string()));
            }
            ConditionalFormatType::Formula => {
                attributes.push(("type", "formula".to_string()));
                attributes.push(("val", value.to_string()));
            }
            ConditionalFormatType::Percentile => {
                attributes.push(("type", "percentile".to_string()));
                attributes.push(("val", value.to_string()));
            }
            ConditionalFormatType::Highest => {
                attributes.push(("type", "max".to_string()));
                if !has_x14_extensions {
                    attributes.push(("val", value.to_string()));
                }
            }
        }

        writer.xml_empty_tag("cfvo", &attributes);
    }

    // Write the <color> element.
    fn write_color(writer: &mut XMLWriter, color_tag: &str, color: Color) {
        let attributes = [("rgb", color.argb_hex_value())];

        writer.xml_empty_tag(color_tag, &attributes);
    }

    // Write the <extLst> element.
    fn write_extension_list(writer: &mut XMLWriter, guid: &str) {
        writer.xml_start_tag_only("extLst");

        let attributes = [
            (
                "xmlns:x14",
                "http://schemas.microsoft.com/office/spreadsheetml/2009/9/main",
            ),
            ("uri", "{B025F937-C7B1-47D3-B67F-A62EFF666E3E}"),
        ];

        writer.xml_start_tag("ext", &attributes);
        writer.xml_data_element_only("x14:id", guid);
        writer.xml_end_tag("ext");
        writer.xml_end_tag("extLst");
    }

    // Return an extended x14 rule for conditional formats that support it.
    pub(crate) fn x14_rule(&self, guid: &str) -> String {
        let mut writer = XMLWriter::new();
        let attributes = [("type", "dataBar".to_string()), ("id", guid.to_string())];

        // Write the rule.
        writer.xml_start_tag("x14:cfRule", &attributes);
        Self::write_data_bar(&mut writer, self.clone());

        Self::write_x14_type(&mut writer, self.min_type, &self.min_value.value, false);
        Self::write_x14_type(&mut writer, self.max_type, &self.max_value.value, true);

        // Write the color elements.
        if !self.border_off {
            Self::write_color(&mut writer, "x14:borderColor", self.border_color);
        }

        if self.fill_color != self.negative_fill_color {
            Self::write_color(
                &mut writer,
                "x14:negativeFillColor",
                self.negative_fill_color,
            );
        }

        if !self.border_off && self.fill_color != self.negative_border_color {
            Self::write_color(
                &mut writer,
                "x14:negativeBorderColor",
                self.negative_border_color,
            );
        }

        if self.axis_position != ConditionalFormatDataBarAxisPosition::None {
            Self::write_color(&mut writer, "x14:axisColor", self.axis_color);
        }

        writer.xml_end_tag("x14:dataBar");
        writer.xml_end_tag("x14:cfRule");

        writer.read_to_string()
    }

    // Write the <x14:dataBar> element.
    fn write_data_bar(writer: &mut XMLWriter, data_bar: ConditionalFormatDataBar) {
        let mut attributes = vec![
            ("minLength", "0".to_string()),
            ("maxLength", "100".to_string()),
        ];

        if !data_bar.border_off {
            attributes.push(("border", "1".to_string()));
        }

        if data_bar.solid_bar {
            attributes.push(("gradient", "0".to_string()));
        }

        match data_bar.direction {
            ConditionalFormatDataBarDirection::LeftToRight => {
                attributes.push(("direction", "leftToRight".to_string()));
            }
            ConditionalFormatDataBarDirection::RightToLeft => {
                attributes.push(("direction", "rightToLeft".to_string()));
            }
            ConditionalFormatDataBarDirection::Context => {}
        }

        if data_bar.fill_color == data_bar.negative_fill_color {
            attributes.push(("negativeBarColorSameAsPositive", "1".to_string()));
        }

        if !data_bar.border_off && data_bar.fill_color != data_bar.negative_border_color {
            attributes.push(("negativeBarBorderColorSameAsPositive", "0".to_string()));
        }

        match data_bar.axis_position {
            ConditionalFormatDataBarAxisPosition::Midpoint => {
                attributes.push(("axisPosition", "middle".to_string()));
            }
            ConditionalFormatDataBarAxisPosition::None => {
                attributes.push(("axisPosition", "none".to_string()));
            }
            ConditionalFormatDataBarAxisPosition::Automatic => {}
        }

        writer.xml_start_tag("x14:dataBar", &attributes);
    }

    // Write the <x14:cfvo> element.
    fn write_x14_type(
        writer: &mut XMLWriter,
        rule_type: ConditionalFormatType,
        value: &str,
        is_max: bool,
    ) {
        let mut attributes = vec![];

        match rule_type {
            ConditionalFormatType::Automatic => {
                if is_max {
                    attributes.push(("type", "autoMax".to_string()));
                } else {
                    attributes.push(("type", "autoMin".to_string()));
                }
                writer.xml_empty_tag("x14:cfvo", &attributes);
            }
            ConditionalFormatType::Lowest => {
                attributes.push(("type", "min".to_string()));
                writer.xml_empty_tag("x14:cfvo", &attributes);
            }
            ConditionalFormatType::Highest => {
                attributes.push(("type", "max".to_string()));
                writer.xml_empty_tag("x14:cfvo", &attributes);
            }
            ConditionalFormatType::Number => {
                attributes.push(("type", "num".to_string()));
                writer.xml_start_tag("x14:cfvo", &attributes);
                writer.xml_data_element_only("xm:f", value);
                writer.xml_end_tag("x14:cfvo");
            }
            ConditionalFormatType::Percent => {
                attributes.push(("type", "percent".to_string()));
                writer.xml_start_tag("x14:cfvo", &attributes);
                writer.xml_data_element_only("xm:f", value);
                writer.xml_end_tag("x14:cfvo");
            }
            ConditionalFormatType::Formula => {
                attributes.push(("type", "formula".to_string()));
                writer.xml_start_tag("x14:cfvo", &attributes);
                writer.xml_data_element_only("xm:f", value);
                writer.xml_end_tag("x14:cfvo");
            }
            ConditionalFormatType::Percentile => {
                attributes.push(("type", "percentile".to_string()));
                writer.xml_start_tag("x14:cfvo", &attributes);
                writer.xml_data_element_only("xm:f", value);
                writer.xml_end_tag("x14:cfvo");
            }
        }
    }
}

// -----------------------------------------------------------------------
// ConditionalFormatValue
// -----------------------------------------------------------------------

/// The `ConditionalFormatValue` struct represents conditional format
/// value types.
///
/// TODO - Explain `ConditionalFormatValue`
///
#[derive(Clone)]
pub struct ConditionalFormatValue {
    value: String,
    pub(crate) is_string: bool,
}

impl ConditionalFormatValue {
    pub(crate) fn new_from_string(value: impl Into<String>) -> ConditionalFormatValue {
        ConditionalFormatValue {
            value: value.into(),
            is_string: false,
        }
    }

    // Helper method to account for the fact that Excel requires that strings in
    // Cell formats are quoted.
    pub(crate) fn quote_string(&mut self) {
        // Only quote string values.
        if !self.is_string || self.value.is_empty() {
            return;
        }

        // Ignore already quoted strings.
        if self.value.starts_with('"') && self.value.ends_with('"') {
            return;
        }

        // Excel requires that double quotes are doubly quoted.
        self.value = self.value.replace('"', "\"\"");

        // Double quote the remaining string.
        self.value = format!("\"{}\"", self.value);
    }
}

macro_rules! conditional_format_value_from_string {
    ($($t:ty)*) => ($(
        impl From<$t> for ConditionalFormatValue {
            fn from(value: $t) -> ConditionalFormatValue {
                let mut value = ConditionalFormatValue::new_from_string(value);
                value.is_string = true;
                value
            }
        }
    )*)
}
conditional_format_value_from_string!(&str &String String Cow<'_, str>);

macro_rules! conditional_format_value_from_number {
    ($($t:ty)*) => ($(
        impl From<$t> for ConditionalFormatValue {
            fn from(value: $t) -> ConditionalFormatValue {
                ConditionalFormatValue::new_from_string(value.to_string())
            }
        }
    )*)
}
conditional_format_value_from_number!(u8 i8 u16 i16 u32 i32 f32 f64);

impl From<Formula> for ConditionalFormatValue {
    fn from(value: Formula) -> ConditionalFormatValue {
        ConditionalFormatValue::new_from_string(value.expand_formula(true))
    }
}

impl From<ExcelDateTime> for ConditionalFormatValue {
    fn from(value: ExcelDateTime) -> ConditionalFormatValue {
        let value = value.to_excel().to_string();
        ConditionalFormatValue::new_from_string(value)
    }
}

impl From<&ExcelDateTime> for ConditionalFormatValue {
    fn from(value: &ExcelDateTime) -> ConditionalFormatValue {
        let value = value.to_excel().to_string();
        ConditionalFormatValue::new_from_string(value)
    }
}

#[cfg(feature = "chrono")]
impl From<&NaiveDate> for ConditionalFormatValue {
    fn from(value: &NaiveDate) -> ConditionalFormatValue {
        let value = ExcelDateTime::chrono_date_to_excel(value).to_string();
        ConditionalFormatValue::new_from_string(value)
    }
}

#[cfg(feature = "chrono")]
impl From<&NaiveDateTime> for ConditionalFormatValue {
    fn from(value: &NaiveDateTime) -> ConditionalFormatValue {
        let value = ExcelDateTime::chrono_datetime_to_excel(value).to_string();
        ConditionalFormatValue::new_from_string(value)
    }
}

#[cfg(feature = "chrono")]
impl From<&NaiveTime> for ConditionalFormatValue {
    fn from(value: &NaiveTime) -> ConditionalFormatValue {
        let value = ExcelDateTime::chrono_time_to_excel(value).to_string();
        ConditionalFormatValue::new_from_string(value)
    }
}

// -----------------------------------------------------------------------
// ConditionalFormatCellCriteria
// -----------------------------------------------------------------------

/// The `ConditionalFormatCellCriteria` enum defines the conditional format
/// criteria for [`ConditionalFormatCell`].
///
///
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConditionalFormatCellCriteria {
    /// The cell conditional format criteria isn't set.
    #[doc(hidden)]
    None,

    /// Show the conditional format for cells that are equal to the target value.
    EqualTo,

    /// Show the conditional format for cells that are not equal to the target value.
    NotEqualTo,

    /// Show the conditional format for cells that are greater than the target value.
    GreaterThan,

    /// Show the conditional format for cells that are greater than or equal to the target value.
    GreaterThanOrEqualTo,

    /// Show the conditional format for cells that are less than the target value.
    LessThan,

    /// Show the conditional format for cells that are less than or equal to the target value.
    LessThanOrEqualTo,

    /// Show the conditional format for cells that are between the target values.
    Between,

    /// Show the conditional format for cells that are not between the target values.
    NotBetween,
}

impl fmt::Display for ConditionalFormatCellCriteria {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConditionalFormatCellCriteria::None => write!(f, ""),
            ConditionalFormatCellCriteria::EqualTo => write!(f, "equal"),
            ConditionalFormatCellCriteria::Between => write!(f, "between"),
            ConditionalFormatCellCriteria::LessThan => write!(f, "lessThan"),
            ConditionalFormatCellCriteria::NotEqualTo => write!(f, "notEqual"),
            ConditionalFormatCellCriteria::NotBetween => write!(f, "notBetween"),
            ConditionalFormatCellCriteria::GreaterThan => write!(f, "greaterThan"),
            ConditionalFormatCellCriteria::LessThanOrEqualTo => write!(f, "lessThanOrEqual"),
            ConditionalFormatCellCriteria::GreaterThanOrEqualTo => write!(f, "greaterThanOrEqual"),
        }
    }
}

// -----------------------------------------------------------------------
// ConditionalFormatAverageCriteria
// -----------------------------------------------------------------------

/// The `ConditionalFormatAverageCriteria` enum defines the conditional format
/// criteria for [`ConditionalFormatCell`].
///
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConditionalFormatAverageCriteria {
    /// Show the conditional format for cells above the average for the range.
    /// This is the default.
    AboveAverage,

    /// Show the conditional format for cells below the average for the range.
    BelowAverage,

    /// Show the conditional format for cells above or equal to the average for
    /// the range.
    EqualOrAboveAverage,

    /// Show the conditional format for cells below or equal to the average for
    /// the range.
    EqualOrBelowAverage,

    /// Show the conditional format for cells 1 standard deviation above the
    /// average for the range.
    OneStandardDeviationAbove,

    /// Show the conditional format for cells 1 standard deviation below the
    /// average for the range.
    OneStandardDeviationBelow,

    /// Show the conditional format for cells 2 standard deviation above the
    /// average for the range.
    TwoStandardDeviationsAbove,

    /// Show the conditional format for cells 2 standard deviation below the
    /// average for the range.
    TwoStandardDeviationsBelow,

    /// Show the conditional format for cells 3 standard deviation above the
    /// average for the range.
    ThreeStandardDeviationsAbove,

    /// Show the conditional format for cells 3 standard deviation below the
    /// average for the range.
    ThreeStandardDeviationsBelow,
}

// -----------------------------------------------------------------------
// ConditionalFormatTextCriteria
// -----------------------------------------------------------------------

/// The `ConditionalFormatTextCriteria` enum defines the conditional format
/// criteria for [`ConditionalFormatText`].
///
///
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConditionalFormatTextCriteria {
    /// Show the conditional format for text that contains to the target string.
    Contains,

    /// Show the conditional format for text that do not contain to the target string.
    DoesNotContain,

    /// Show the conditional format for text that begins with the target string.
    BeginsWith,

    /// Show the conditional format for text that ends with the target string.
    EndsWith,
}

impl fmt::Display for ConditionalFormatTextCriteria {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConditionalFormatTextCriteria::Contains => write!(f, "containsText"),
            ConditionalFormatTextCriteria::DoesNotContain => write!(f, "notContains"),
            ConditionalFormatTextCriteria::BeginsWith => write!(f, "beginsWith"),
            ConditionalFormatTextCriteria::EndsWith => write!(f, "endsWith"),
        }
    }
}

// -----------------------------------------------------------------------
// ConditionalFormatDateCriteria
// -----------------------------------------------------------------------

/// The `ConditionalFormatDateCriteria` enum defines the conditional format
/// criteria for [`ConditionalFormatDate`].
///
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConditionalFormatDateCriteria {
    /// Show the conditional format for dates occurring yesterday. This is the
    /// default.
    Yesterday,

    /// Show the conditional format for dates occurring today, relative to when
    /// the file is opened.
    Today,

    /// Show the conditional format for dates occurring tomorrow, relative to
    /// when the file is opened.
    Tomorrow,

    /// Show the conditional format for dates occurring in the last 7 days,
    /// relative to when the file is opened.
    Last7Days,

    /// Show the conditional format for dates occurring in the last week,
    /// relative to when the file is opened.
    LastWeek,

    /// Show the conditional format for dates occurring this week, relative to
    /// when the file is opened.
    ThisWeek,

    /// Show the conditional format for dates occurring in the next week,
    /// relative to when the file is opened.
    NextWeek,

    /// Show the conditional format for dates occurring in the last month,
    /// relative to when the file is opened.
    LastMonth,

    /// Show the conditional format for dates occurring this month, relative to
    /// when the file is opened.
    ThisMonth,

    /// Show the conditional format for dates occurring in the next month,
    /// relative to when the file is opened.
    NextMonth,
}

impl fmt::Display for ConditionalFormatDateCriteria {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConditionalFormatDateCriteria::Today => write!(f, "today"),
            ConditionalFormatDateCriteria::Tomorrow => write!(f, "tomorrow"),
            ConditionalFormatDateCriteria::LastWeek => write!(f, "lastWeek"),
            ConditionalFormatDateCriteria::NextWeek => write!(f, "nextWeek"),
            ConditionalFormatDateCriteria::ThisWeek => write!(f, "thisWeek"),
            ConditionalFormatDateCriteria::Last7Days => write!(f, "last7Days"),
            ConditionalFormatDateCriteria::LastMonth => write!(f, "lastMonth"),
            ConditionalFormatDateCriteria::NextMonth => write!(f, "nextMonth"),
            ConditionalFormatDateCriteria::ThisMonth => write!(f, "thisMonth"),
            ConditionalFormatDateCriteria::Yesterday => write!(f, "yesterday"),
        }
    }
}

// -----------------------------------------------------------------------
// ConditionalFormatType
// -----------------------------------------------------------------------

/// The `ConditionalFormatType` enum defines the conditional format type
/// for [`ConditionalFormat2ColorScale`], [`ConditionalFormat3ColorScale`] and
/// [`ConditionalFormatDataBar`].
///
/// # Examples
///
/// Example of adding a 2 color scale type conditional formatting to a worksheet
/// with user defined minimum and maximum values.
///
/// ```
/// # // This code is available in examples/doc_conditional_format_2color_set_minimum.rs
/// #
/// # use rust_xlsxwriter::{
/// #     ConditionalFormat2ColorScale, ConditionalFormatType, Workbook, XlsxError,
/// # };
/// #
/// # fn main() -> Result<(), XlsxError> {
/// #     // Create a new Excel file object.
/// #     let mut workbook = Workbook::new();
/// #     let worksheet = workbook.add_worksheet();
/// #
/// #     // Write the worksheet data.
/// #     let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// #     worksheet.write_column(2, 1, data)?;
/// #     worksheet.write_column(2, 3, data)?;
/// #
///     // Write a 2 color scale formats with standard Excel colors. The conditional
///     // format is applied from the lowest to the highest value.
///     let conditional_format = ConditionalFormat2ColorScale::new();
///
///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
///
///     // Write a 2 color scale formats with standard Excel colors but a user
///     // defined range. Values <= 3 will be shown with the minimum color while
///     // values >= 7 will be shown with the maximum color.
///     let conditional_format = ConditionalFormat2ColorScale::new()
///         .set_minimum(ConditionalFormatType::Number, 3)
///         .set_maximum(ConditionalFormatType::Number, 7);
///
///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
/// #
/// #     // Save the file.
/// #     workbook.save("conditional_format.xlsx")?;
/// #
/// #     Ok(())
/// # }
/// ```
///
/// Output file:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_2color_set_minimum.png">
///
///
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConditionalFormatType {
    /// Set the color scale/data bar to use the minimum or maximum value in the range. This is the
    /// default for data bars.
    Automatic,

    /// Set the color scale/data bar to use the minimum value in the range. This is the
    /// default for the minimum value in color scales.
    Lowest,

    /// Set the color scale/data bar to use a number value other than the
    /// maximum/minimum.
    Number,

    /// Set the color scale/data bar to use a percentage. This must be in the range
    /// 0-100.
    Percent,

    /// Set the color scale/data bar to use a formula value.
    Formula,

    /// Set the color scale/data bar to use a percentile. This must be in the range
    /// 0-100.
    Percentile,

    /// Set the color scale/data bar to use the maximum value in the range. This is the
    /// default for the maximum value in value in color scales.
    Highest,
}

// -----------------------------------------------------------------------
// ConditionalFormatDataBarDirection
// -----------------------------------------------------------------------

/// The `ConditionalFormatDataBarDirection` enum defines the conditional format
/// directions for [`ConditionalFormatDataBar`]. This is used to set the data
/// bar conditional format direction to "Right to left", "Left to right" or
/// "Context" (the default) in conjunction with
/// [`ConditionalFormatDataBar::set_direction()`].
///
/// # Parameters
///
/// * `direction`- A [`ConditionalFormatDataBarDirection`] enum value.
///
/// # Examples
///
/// Example of adding a data bar type conditional formatting to a worksheet
/// without a border
///
/// ```
/// # // This code is available in examples/doc_conditional_format_databar_set_direction.rs
/// #
/// # use rust_xlsxwriter::{
/// #     ConditionalFormatDataBar, ConditionalFormatDataBarDirection, Workbook, XlsxError,
/// # };
/// #
/// # fn main() -> Result<(), XlsxError> {
/// #     // Create a new Excel file object.
/// #     let mut workbook = Workbook::new();
/// #     let worksheet = workbook.add_worksheet();
/// #
/// #     // Write the worksheet data.
/// #     let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// #     worksheet.write_column(2, 1, data)?;
/// #     worksheet.write_column(2, 3, data)?;
/// #
///     // Write a standard Excel data bar.
///     let conditional_format = ConditionalFormatDataBar::new();
///
///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
///
///     // Write a data bar without a border.
///     let conditional_format = ConditionalFormatDataBar::new()
///         .set_direction(ConditionalFormatDataBarDirection::RightToLeft);
///
///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
///
/// #     // Save the file.
/// #     workbook.save("conditional_format.xlsx")?;
/// #
/// #     Ok(())
/// # }
/// ```
///
/// Output file:
///
/// <img
/// src="https://rustxlsxwriter.github.io/images/conditional_format_databar_set_direction.png">
///
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConditionalFormatDataBarDirection {
    /// The bars go "Right to left" or "Left to right" depending on the context.
    /// This is the default.
    Context,

    /// The bars go "Left to right".
    LeftToRight,

    /// The bars go "Right to left".
    RightToLeft,
}

// -----------------------------------------------------------------------
// ConditionalFormatDataBarAxisPosition
// -----------------------------------------------------------------------

/// The `ConditionalFormatDataBarAxisPosition` enum defines the conditional
/// format axis positions for [`ConditionalFormatDataBar`].
///
///
/// # Examples
///
/// Example of adding a data bar type conditional formatting to a worksheet
/// with different axis positions.
///
/// ```
/// # // This code is available in examples/doc_conditional_format_databar_set_axis_position.rs
/// #
/// # use rust_xlsxwriter::{
/// #     ConditionalFormatDataBar, ConditionalFormatDataBarAxisPosition, Workbook, XlsxError,
/// # };
/// #
/// # fn main() -> Result<(), XlsxError> {
/// #     // Create a new Excel file object.
/// #     let mut workbook = Workbook::new();
/// #     let worksheet = workbook.add_worksheet();
/// #
/// #     // Write the worksheet data.
/// #     let data1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// #     let data2 = [6, 4, 2, -2, -4, -6, -4, -2, 2, 4];
/// #     worksheet.write_column(2, 1, data1)?;
/// #     worksheet.write_column(2, 3, data1)?;
/// #     worksheet.write_column(2, 5, data2)?;
/// #     worksheet.write_column(2, 7, data2)?;
/// #
///     // Write a standard Excel data bar.
///     let conditional_format = ConditionalFormatDataBar::new();
///
///     worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;
///
///     // Write a data bar with a midpoint axis.
///     let conditional_format = ConditionalFormatDataBar::new()
///         .set_axis_position(ConditionalFormatDataBarAxisPosition::Midpoint);
///
///     worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;
///
///     // Write a standard Excel data bar with negative data
///     let conditional_format = ConditionalFormatDataBar::new();
///
///     worksheet.add_conditional_format(2, 5, 11, 5, &conditional_format)?;
///
///     // Write a data bar without an axis.
///     let conditional_format = ConditionalFormatDataBar::new()
///         .set_axis_position(ConditionalFormatDataBarAxisPosition::None);
///
///     worksheet.add_conditional_format(2, 7, 11, 7, &conditional_format)?;
///
/// #     // Save the file.
/// #     workbook.save("conditional_format.xlsx")?;
/// #
/// #     Ok(())
/// # }
/// ```
///
/// Output file:
///
/// <img src="https://rustxlsxwriter.github.io/images/conditional_format_databar_set_axis_position.png">
///
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConditionalFormatDataBarAxisPosition {
    /// The axis is set automatically depending on whether the data contains
    /// negative values. This is the default.
    Automatic,

    /// The axis is set at the midpoint. This is the automatic option for ranges
    /// with negative values.
    Midpoint,

    /// Turn the axis off.
    None,
}

// -----------------------------------------------------------------------
// Generate common methods.
// -----------------------------------------------------------------------
macro_rules! generate_conditional_common_methods {
    ($($t:ty)*) => ($(

    /// **Section 2**: The following methods are common to all conditional
    /// formatting variants.
    impl $t {
        /// Set the [`Format`] of the conditional format rule.
        ///
        /// Set the [`Format`] that will be applied to the cell range if the conditional
        /// format rule applies. Not all cell format properties can be set in a
        /// conditional format. See [Excel's limitations on conditional format
        /// properties](crate::conditional_format#excels-limitations-on-conditional-format-properties) for
        /// more information.
        ///
        /// See the examples above.
        ///
        /// # Parameters
        ///
        /// * `format` - The [`Format`] property for the conditional format.
        ///
        pub fn set_format(mut self, format: impl Into<Format>) -> $t {
            self.format = Some(format.into());
            self
        }

        /// Set an additional multi-cell range for the conditional format.
        ///
        /// The `set_multi_range()` method is used to extend a conditional
        /// format over non-contiguous ranges like `"B3:D6 I3:K6 B9:D12
        /// I9:K12"`.
        ///
        /// See [Selecting a non-contiguous
        /// range](crate::conditional_format#selecting-a-non-contiguous-range)
        /// for more information.
        ///
        /// # Parameters
        ///
        /// * `range` - A string like type representing an Excel range.
        ///
        ///   Note, you can use an Excel range like `"$B$3:$D$6,$I$3:$K$6"` or
        ///   omit the `$` anchors and replace the commas with spaces to have a
        ///   clearer range like `"B3:D6 I3:K6"`. The documentation and examples
        ///   use the latter format for clarity but it you are copying and
        ///   pasting from Excel you can use the first format.
        ///
        pub fn set_multi_range(mut self, range: impl Into<String>) -> $t {
            self.multi_range = range.into().replace('$', "").replace(',', " ");
            self
        }

        /// Set the "Stop if True" option for the conditional format rule.
        ///
        /// The `set_stop_if_true()` method can be used to set the “Stop if true”
        /// feature of a conditional formatting rule when more than one rule is
        /// applied to a cell or a range of cells. When this parameter is set then
        /// subsequent rules are not evaluated if the current rule is true.
        ///
        /// # Parameters
        ///
        /// * `enable` - Turn the property on/off. It is off by default.
        ///
        pub fn set_stop_if_true(mut self, enable: bool) -> $t {
            self.stop_if_true = enable;
            self
        }

        // Get the index of the format object in the conditional format.
        pub(crate) fn format_index(&self) -> Option<u32> {
            self.format.as_ref().map(|format| format.dxf_index)
        }

        // Get a reference to the format object in the conditional format.
        pub(crate) fn format_as_mut(&mut self) -> Option<&mut Format> {
            self.format.as_mut()
        }

        // Get the multi-cell range for the conditional format, if present.
        pub(crate) fn multi_range(&self) -> String {
            self.multi_range.clone()
        }

        /// Check if the conditional format uses Excel 2010+ extensions.
        pub(crate) fn has_x14_extensions(&self) -> bool {
            self.has_x14_extensions
        }
    }
    )*)
}
generate_conditional_common_methods!(
    ConditionalFormatAverage
    ConditionalFormatBlank
    ConditionalFormatCell
    ConditionalFormatDate
    ConditionalFormatDuplicate
    ConditionalFormatError
    ConditionalFormatFormula
    ConditionalFormatText
    ConditionalFormatTop
    ConditionalFormat2ColorScale
    ConditionalFormat3ColorScale
    ConditionalFormatDataBar
);

// -----------------------------------------------------------------------
// Common methods.
// -----------------------------------------------------------------------

// TODO COW
fn range_to_anchor(range: &str) -> String {
    let mut anchor = range;

    // Extract cell/range from optional space separated list.
    if let Some(position) = anchor.find(' ') {
        anchor = &anchor[0..position];
    }

    // Extract cell from optional : separated range.
    if let Some(position) = anchor.find(':') {
        anchor = &anchor[0..position];
    }

    anchor.to_string()
}