use std::{error::Error, fs, path::Path};

use cairo::{Context, ImageSurface};

use crate::{Color, State};

pub fn render_to<P: AsRef<Path>>(
    dst: P,
    cols: i32,
    rows: i32,
    cell_size: i32,
    theme: &[Color],
    iter: impl Iterator<Item = State>,
) -> Result<(), Box<dyn Error>> {
    let width = cols * cell_size + cols + 1;
    let height = rows * cell_size + rows + 1;
    let img = ImageSurface::create(cairo::Format::ARgb32, width, height)?;
    let ctx = Context::new(&img)?;

    theme[0].set(&ctx);
    ctx.rectangle(0.0, 0.0, width as f64, height as f64);
    ctx.fill()?;

    for (j, state) in iter.take(rows as usize).enumerate() {
        let y = j as i32 * (cell_size + 1) + 1;
        for (i, v) in state.iter().enumerate() {
            if v == 0 {
                continue;
            }
            let x = i as i32 * (cell_size + 1) + 1;
            theme[v as usize].set(&ctx);
            ctx.rectangle(x as f64, y as f64, cell_size as f64, cell_size as f64);
            ctx.fill()?;
        }
    }

    img.write_to_png(&mut fs::File::create(dst)?)?;

    Ok(())
}
