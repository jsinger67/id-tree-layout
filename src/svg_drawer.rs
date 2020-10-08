//! The SvgDrawer type provides the tranformation of the embedding information
//! into the Svg format.

use crate::Drawer;
use std::path::Path;
use xml_writer::XmlWriter;

use std::fs::File;

use super::embedder::PlacedTreeItem;

pub type Result = std::io::Result<()>;

const X_MARGIN: f32 = 10.0;
const Y_MARGIN: f32 = 25.0;
const Y_FACTOR: f32 = 3.5;
const FONT_X_SIZE: f32 = 10.0;
const FONT_Y_SIZE: f32 = 10.0;

#[derive(Debug, Default)]
pub struct SvgDrawer;

impl SvgDrawer {
    pub fn new() -> Self {
        Self
    }

    fn scale_y(y: usize) -> f32 {
        y as f32 * FONT_Y_SIZE * Y_FACTOR + Y_MARGIN
    }

    fn scale_x(x: usize) -> f32 {
        x as f32 * FONT_X_SIZE + X_MARGIN
    }

    fn measure_string(str: &str) -> f32 {
        str.len() as f32 * FONT_X_SIZE
    }
}

impl Drawer for SvgDrawer {
    fn draw(&self, file_name: &Path, embedding: &[PlacedTreeItem]) -> Result {
        let file = File::create(file_name)?;
        let mut xml = XmlWriter::new(file);

        xml.dtd("UTF-8")?;
        xml.begin_elem("svg")?;
        xml.attr("xmlns", "http://www.w3.org/2000/svg")?;
        xml.attr("version", "1.1")?;
        xml.attr("lang", "en")?;

        const STRING_FONT: &str = "font-family: 'Courier'; font-style: normal";
        const TERMINAL_FONT: &str = "font-family: 'Courier'; font-weight: bold; font-style: normal";

        let tree_depth = embedding
            .iter()
            .fold(0, |acc, e| if e.y_order > acc { e.y_order } else { acc });
        let tree_width = embedding.iter().fold(0, |acc, e| {
            if e.x_extend_children > acc {
                e.x_extend_children
            } else {
                acc
            }
        });

        let img_width = Self::scale_x(tree_width);
        let img_height = Self::scale_y(tree_depth + 1);

        xml.attr("width", format!("{}", img_width).as_str())?;
        xml.attr("height", format!("{}", img_height).as_str())?;

        // Draw on a white rectangle to be visible also on black backgrounds.
        xml.begin_elem("rect")?;
        xml.attr("x", "0")?;
        xml.attr("y", "0")?;
        xml.attr("width", format!("{}", img_width).as_str())?;
        xml.attr("height", format!("{}", img_height).as_str())?;
        xml.attr("fill", "white")?;
        xml.end_elem()?;

        for data in embedding {
            let font = if data.is_empasized {
                TERMINAL_FONT
            } else {
                STRING_FONT
            };
            let szx = Self::measure_string(&data.name);
            let x = Self::scale_x(data.x_center) - szx / 2.0;
            let y = Self::scale_y(data.y_order);
            xml.begin_elem("text")?;
            xml.attr("x", format!("{}", x).as_str())?;
            xml.attr("y", format!("{}", y).as_str())?;
            xml.attr("style", font)?;
            xml.text(data.name.as_str())?;
            xml.end_elem()?;

            if let Some(parent_index) = data.parent {
                let parent_data = embedding.iter().find(|e| e.id == parent_index).unwrap();

                // Draw a line from the nodes parent down to this node
                xml.begin_elem("line")?;
                xml.attr(
                    "x1",
                    format!("{}", (Self::scale_x(parent_data.x_center))).as_str(),
                )?;
                xml.attr(
                    "y1",
                    format!("{}", (Self::scale_y(parent_data.y_order) + FONT_Y_SIZE)).as_str(),
                )?;
                xml.attr("x2", format!("{}", (Self::scale_x(data.x_center))).as_str())?;
                xml.attr("y2", format!("{}", (y - FONT_Y_SIZE)).as_str())?;
                xml.attr("stroke", "black")?;
                xml.end_elem()?;
            }
        }

        xml.end_elem()?;
        xml.close()?;
        xml.flush()?;

        Ok(())
    }
}
