//! Draw for visualizing backtest results and candle charts.

use crate::engine::{Backtest, Candle};
use crate::errors::{Error, Result};

use plotters::backend::{BitMapBackend, DrawingBackend, SVGBackend};
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters::style::WHITE;

/// Output formats for the generated charts.
#[derive(Debug, Default)]
pub enum DrawOutput {
    /// SVG vector format (default)
    #[default]
    Svg,
    /// PNG raster format
    Png,
    /// HTML interactive format (not yet implemented)
    Html,
}

/// Configuration options for chart generation.
#[derive(Debug, Default)]
pub struct DrawOptions {
    title: Option<String>,
    output: DrawOutput,
    show_volume: bool,
}

impl DrawOptions {
    /// Creates a new DrawOptions with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the chart title.
    ///
    /// # Arguments
    /// * `title` - The title to display on the chart
    ///
    /// # Returns
    /// Self for method chaining
    pub fn title(mut self, title: impl ToString) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Sets the output format.
    ///
    /// # Arguments
    /// * `o` - The output format (Svg, Png, or Html)
    ///
    /// # Returns
    /// Self for method chaining
    pub fn output(mut self, o: DrawOutput) -> Self {
        self.output = o;
        self
    }

    /// Sets whether to show volume bars.
    ///
    /// # Arguments
    /// * `show` - Boolean indicating whether to show volume
    ///
    /// # Returns
    /// Self for method chaining
    pub fn show_volume(mut self, show: bool) -> Self {
        self.show_volume = show;
        self
    }
}

/// Chart drawing utility for backtest visualization.
#[derive(Debug, Default)]
pub struct Draw<'d> {
    backtest: Option<&'d Backtest>,
    options: DrawOptions,
}

impl<'d> Draw<'d> {
    /// Creates a new Draw instance with the given backtest.
    ///
    /// # Arguments
    /// * `backtest` - Reference to the backtest to visualize
    ///
    /// # Returns
    /// A new Draw instance
    pub fn with_backtest(backtest: &'d Backtest) -> Self {
        Self {
            backtest: Some(backtest),
            options: DrawOptions::default(),
        }
    }

    /// Sets the drawing options.
    ///
    /// # Arguments
    /// * `options` - The drawing options to use
    ///
    /// # Returns
    /// Self for method chaining
    pub fn with_options(mut self, options: DrawOptions) -> Self {
        self.options = options;
        self
    }

    /// Generates and saves the chart based on the configured options.
    ///
    /// Automatically calculates chart dimensions based on the number of candles.
    /// Minimum width is 500px plus 10px per candle, with a 16:9 aspect ratio.
    ///
    /// # Returns
    /// Result indicating success or failure
    pub fn plot(&self) -> Result<()> {
        let backtest = self.backtest.ok_or(Error::Msg("No backtest provided".to_string()))?;
        let candles = backtest.candles().collect::<Vec<_>>();

        if candles.is_empty() {
            return Err(Error::CandleDataEmpty);
        }

        let title = self.options.title.as_deref().unwrap_or("BTS Chart");

        let candle_count = candles.len() as u32;
        let width = 500.max(50 + candle_count * 10);
        let height = ((width as f64 * 0.5625) as u32).min(900);

        let show_volume = self.options.show_volume;

        match self.options.output {
            DrawOutput::Svg => self.plot_svg(&candles, width, height, title, show_volume),
            DrawOutput::Png => self.plot_png(&candles, width, height, title, show_volume),
            DrawOutput::Html => Ok(()), //todo use plotters-canvas crate
        }
    }

    /// Generates an SVG chart.
    ///
    /// # Arguments
    /// * `candles` - Slice of candle references to plot
    /// * `width` - Chart width in pixels
    /// * `height` - Chart height in pixels
    /// * `title` - Chart title
    /// * `show_volume` - Whether to show volume bars
    ///
    /// # Returns
    /// Result indicating success or failure
    fn plot_svg(&self, candles: &[&Candle], width: u32, height: u32, title: &str, show_volume: bool) -> Result<()> {
        let root = SVGBackend::new("bts.svg", (width, height)).into_drawing_area();
        root.fill(&WHITE).map_err(|e| Error::Plotters(e.to_string()))?;
        self.draw_chart(&root, candles, title, show_volume)
    }

    /// Generates a PNG chart.
    ///
    /// # Returns
    /// Result indicating success or failure
    fn plot_png(&self, candles: &[&Candle], width: u32, height: u32, title: &str, show_volume: bool) -> Result<()> {
        let root = BitMapBackend::new("bts.png", (width, height)).into_drawing_area();
        root.fill(&WHITE).map_err(|e| Error::Plotters(e.to_string()))?;
        self.draw_chart(&root, candles, title, show_volume)
    }

    /// Draws the chart on the given drawing area.
    ///
    /// # Arguments
    /// * `drawing_area` - The drawing area to render to
    /// * `candles` - Slice of candle references to plot
    /// * `title` - Chart title
    /// * `show_volume` - Whether to show volume bars
    ///
    /// # Returns
    /// Result indicating success or failure
    fn draw_chart<DB: DrawingBackend>(
        &self,
        drawing_area: &DrawingArea<DB, Shift>,
        candles: &[&Candle],
        title: &str,
        _show_volume: bool,
    ) -> Result<()> {
        let min_price = candles.iter().map(|c| c.low()).fold(f64::INFINITY, f64::min);
        let max_price = candles.iter().map(|c| c.high()).fold(f64::NEG_INFINITY, f64::max);
        let price_range = max_price - min_price;
        let price_padding = price_range * 0.1; // 10% margin

        let first_time = candles.first().ok_or(Error::CandleNotFound)?.open_time();
        let last_time = candles.last().ok_or(Error::CandleNotFound)?.close_time();

        let x_label_size = 20;
        let y_label_size = 20;
        let caption_size = 30;

        let mut chart = ChartBuilder::on(drawing_area)
            .caption(title, ("sans-serif", caption_size).into_font())
            .margin(x_label_size + 10)
            .margin_top(caption_size + 10)
            .x_label_area_size(x_label_size)
            .y_label_area_size(y_label_size)
            .build_cartesian_2d(
                first_time..last_time,
                min_price - price_padding..max_price + price_padding,
            )
            .map_err(|e| Error::Plotters(e.to_string()))?;

        chart
            .configure_mesh()
            .x_desc("Time")
            .y_desc("Price")
            .x_label_style(("sans-serif", x_label_size))
            .y_label_style(("sans-serif", y_label_size))
            .x_labels(5)
            .y_labels(5)
            .draw()
            .map_err(|e| Error::Plotters(e.to_string()))?;

        let candle_width = {
            let total_width = drawing_area.dim_in_pixel().0 as f64;
            let available_width = total_width - (x_label_size * 2) as f64;
            let candles_count = candles.len() as f64;
            (available_width / candles_count).max(5.0) as u32
        };

        let candlesticks = candles.iter().map(|c| {
            let x = c.open_time();
            let open = c.open();
            let high = c.high();
            let low = c.low();
            let close = c.close();
            CandleStick::new(x, open, high, low, close, GREEN.filled(), RED.filled(), candle_width)
        });

        chart
            .draw_series(candlesticks)
            .map_err(|e| Error::Plotters(e.to_string()))?;

        chart
            .configure_series_labels()
            .background_style(WHITE.mix(0.8))
            .border_style(BLACK)
            .draw()
            .map_err(|e| Error::Plotters(e.to_string()))?;

        drawing_area.present().map_err(|e| Error::Plotters(e.to_string()))
    }
}
