//! Generate `image-rs` images of `Position`s.

use super::{Color, InvalidThemeError, Position};
use image::{self, imageops::FilterType, DynamicImage, GenericImage, GenericImageView};
use std::path::PathBuf;

/// Represents a theme, i.e. board theme and piece style.
/// Currently, all of Chess.com's board themes are available, but
/// as for piece sets, only one option— "normal", is available.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Theme<'a> {
    pub board_theme: &'a str,
    pub piece_set: &'a str,
}

impl<'a> Default for Theme<'a> {
    fn default() -> Self {
        Self {
            board_theme: "brown",
            piece_set: "normal",
        }
    }
}

/// Creates an image of a `Position`, from the perspective of the side `perspective`.
pub fn position_to_image<'a>(position: &Position, theme: Theme<'a>, perspective: Color) -> Result<DynamicImage, InvalidThemeError<'a>> {
    let assets_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    let mut content = position.content;
    let ranks: Vec<_> = if perspective.is_white() {
        content.chunks(8).rev().enumerate().collect()
    } else {
        content.reverse();
        content.chunks(8).rev().enumerate().collect()
    };
    let mut board_image = image::open(
        assets_path
            .join("boards")
            .join(format!("{perspective}"))
            .join(format!("{}.png", theme.board_theme.replace(' ', "-").to_lowercase())),
    )
    .map_err(|_| InvalidThemeError(theme))?;
    let piece_size = board_image.width() / 8;
    for (ranki, rank) in ranks {
        for (sqi, sq) in rank.iter().enumerate() {
            if let Some(piece) = sq {
                let piece_image = image::open(
                    assets_path
                        .join("pieces")
                        .join(format!("{}", theme.piece_set))
                        .join(format!("{}", piece.color()))
                        .join(format!("{}.png", char::from(*piece))),
                )
                .map_err(|_| InvalidThemeError(theme))?
                .resize(piece_size, piece_size, FilterType::Nearest);
                let piece_x = sqi as u32 * piece_size;
                let piece_y = ranki as u32 * piece_size;
                for y in 0..piece_size {
                    for x in 0..piece_size {
                        let px = piece_image.get_pixel(x, y);
                        if px.0[3] != 0 {
                            board_image.put_pixel(piece_x + x, piece_y + y, px);
                        }
                    }
                }
            }
        }
    }
    Ok(board_image)
}
