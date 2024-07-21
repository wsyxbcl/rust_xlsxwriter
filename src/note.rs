// note - A module to represent Excel cell notes.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

#![warn(missing_docs)]

use crate::drawing::{DrawingObject, DrawingType};
use crate::vml::VmlInfo;
use crate::{ColNum, ObjectMovement, RowNum, COL_MAX, ROW_MAX};

#[derive(Clone)]
/// The `Note` struct represents an worksheet note object.
///
/// TODO
///
pub struct Note {
    height: f64,
    width: f64,
    row: Option<RowNum>,
    col: Option<ColNum>,
    x_offset: Option<u32>,
    y_offset: Option<u32>,

    pub(crate) author: Option<String>,
    pub(crate) author_id: Option<usize>,
    pub(crate) cell_row: RowNum,
    pub(crate) cell_col: ColNum,
    pub(crate) text: String,
    pub(crate) alt_text: String,
    pub(crate) object_movement: ObjectMovement,
    pub(crate) decorative: bool,
}

impl Note {
    // -----------------------------------------------------------------------
    // Public (and crate public) methods.
    // -----------------------------------------------------------------------

    /// Create a new Note object to represent an Excel Form Control note.
    ///
    pub fn new(text: impl Into<String>) -> Note {
        Note {
            row: None,
            col: None,
            x_offset: None,
            y_offset: None,

            cell_row: 0,
            cell_col: 0,

            author: None,
            author_id: None,
            width: 128.0,
            height: 74.0,
            text: text.into(),
            alt_text: String::new(),
            object_movement: ObjectMovement::DontMoveOrSizeWithCells,
            decorative: false,
        }
    }

    /// Set the width of the note in pixels.
    ///
    /// # Parameters
    ///
    /// - `width`: The note width in pixels.
    ///
    pub fn set_width(mut self, width: u32) -> Note {
        if width == 0 {
            return self;
        }

        self.width = f64::from(width);
        self
    }

    /// Set the height of the note in pixels.
    ///
    /// # Parameters
    ///
    /// - `height`: The note height in pixels.
    ///
    pub fn set_height(mut self, height: u32) -> Note {
        if height == 0 {
            return self;
        }
        self.height = f64::from(height);
        self
    }

    /// Set the TODO
    ///
    /// # Parameters
    ///
    /// - `name`: The aTODO
    ///
    pub fn set_author(mut self, name: impl Into<String>) -> Note {
        let author = name.into();
        if author.chars().count() > 54 {
            eprintln!("Author name is greater than Excel's limit of 54 characters.");
            return self;
        }

        self.author = Some(author);
        self
    }

    /// Set the alt text for the note to help accessibility.
    ///
    /// The alt text is used with screen readers to help people with visual
    /// disabilities.
    ///
    /// See the following Microsoft documentation on [Everything you need to
    /// know to write effective alt
    /// text](https://support.microsoft.com/en-us/office/everything-you-need-to-know-to-write-effective-alt-text-df98f884-ca3d-456c-807b-1a1fa82f5dc2).
    ///
    /// # Parameters
    ///
    /// - `alt_text`: The alt text string to add to the note.
    ///
    pub fn set_alt_text(mut self, alt_text: impl Into<String>) -> Note {
        let alt_text = alt_text.into();
        if alt_text.chars().count() > 255 {
            eprintln!("Alternative text is greater than Excel's limit of 255 characters.");
            return self;
        }

        self.alt_text = alt_text;
        self
    }

    /// Set the object movement options for a worksheet note.
    ///
    /// Set the option to define how an note will behave in Excel if the cells
    /// under the note are moved, deleted, or have their size changed. In
    /// Excel the options are:
    ///
    /// 1. Move and size with cells.
    /// 2. Move but don't size with cells.
    /// 3. Don't move or size with cells.
    ///
    /// <img src="https://rustxlsxwriter.github.io/images/object_movement.png">
    ///
    /// These values are defined in the [`ObjectMovement`] enum.
    ///
    /// The [`ObjectMovement`] enum also provides an additional option to "Move
    /// and size with cells - after the note is inserted" to allow notes to
    /// be hidden in rows or columns. In Excel this equates to option 1 above
    /// but the internal note position calculations are handled differently.
    ///
    /// # Parameters
    ///
    /// - `option`: An note/object positioning behavior defined by the
    ///   [`ObjectMovement`] enum.
    pub fn set_object_movement(mut self, option: ObjectMovement) -> Note {
        self.object_movement = option;
        self
    }

    // Notes are stored in a vmlDrawing file. We create a struct to store the
    // required image information in that format.
    pub(crate) fn vml_info(&self) -> VmlInfo {
        VmlInfo {
            width: self.width,
            height: self.height,
            text: self.text.clone(),
            alt_text: self.alt_text.clone(),
            ..Default::default()
        }
    }

    //TODO
    pub(crate) fn row(&self) -> RowNum {
        match self.row {
            Some(row) => row,
            None => {
                if self.cell_row == 0 {
                    0
                } else if self.cell_row == ROW_MAX - 3 {
                    ROW_MAX - 7
                } else if self.cell_row == ROW_MAX - 2 {
                    ROW_MAX - 6
                } else if self.cell_row == ROW_MAX - 1 {
                    ROW_MAX - 5
                } else {
                    self.cell_row - 1
                }
            }
        }
    }

    //TODO
    pub(crate) fn col(&self) -> ColNum {
        match self.col {
            Some(col) => col,
            None => {
                if self.cell_col == COL_MAX - 3 {
                    COL_MAX - 6
                } else if self.cell_col == COL_MAX - 2 {
                    COL_MAX - 5
                } else if self.cell_col == COL_MAX - 1 {
                    COL_MAX - 4
                } else {
                    self.cell_col + 1
                }
            }
        }
    }
}

// Trait for objects that have a component stored in the drawing.xml file.
impl DrawingObject for Note {
    #[allow(clippy::if_same_then_else)]
    fn x_offset(&self) -> u32 {
        match self.x_offset {
            Some(offset) => offset,
            None => {
                if self.cell_col == COL_MAX - 3 {
                    49
                } else if self.cell_col == COL_MAX - 2 {
                    49
                } else if self.cell_col == COL_MAX - 1 {
                    49
                } else {
                    15
                }
            }
        }
    }

    #[allow(clippy::if_same_then_else)]
    fn y_offset(&self) -> u32 {
        match self.y_offset {
            Some(offset) => offset,
            None => {
                if self.cell_row == 0 {
                    2
                } else if self.cell_row == ROW_MAX - 3 {
                    16
                } else if self.cell_row == ROW_MAX - 2 {
                    16
                } else if self.cell_row == ROW_MAX - 1 {
                    14
                } else {
                    10
                }
            }
        }
    }

    fn width_scaled(&self) -> f64 {
        self.width
    }

    fn height_scaled(&self) -> f64 {
        self.height
    }

    fn object_movement(&self) -> ObjectMovement {
        self.object_movement
    }

    fn name(&self) -> String {
        self.text.clone()
    }

    fn alt_text(&self) -> String {
        self.alt_text.clone()
    }

    fn decorative(&self) -> bool {
        self.decorative
    }

    fn drawing_type(&self) -> DrawingType {
        DrawingType::Vml
    }
}
