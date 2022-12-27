use basic_ops::histogram::{calculate_histogram, cumulative_histogram, normalize_histogram};
use egui::{
    plot::{Bar, BarChart, Legend, Plot},
    Context, Window,
};
use image::DynamicImage;

#[derive(Clone)]
pub struct HistogramGraph {
    open: bool,
    title: String,
    raw_values: [u32; 256],
}

impl Default for HistogramGraph {
    fn default() -> Self {
        Self {
            open: false,
            title: Default::default(),
            raw_values: [0; 256],
        }
    }
}

impl HistogramGraph {
    pub fn new(title: String, histogram: [u32; 256]) -> Self {
        Self {
            open: true,
            title,
            raw_values: histogram,
        }
    }

    pub fn plot_histogram(&mut self, ctx: &Context) {
        let chart = BarChart::new(
            (0..=255_usize)
                .map(|x| Bar::new(x as f64 + 0.5, self.raw_values[x].into()).width(1.0))
                .collect(),
        )
        .width(1.0);

        Window::new(&self.title)
            .open(&mut self.open)
            .show(ctx, |ui| {
                Plot::new("Normal Distribution Demo")
                    .legend(Legend::default())
                    .show(ui, |plot_ui| plot_ui.bar_chart(chart))
            });
    }

    pub fn all_histograms(img: &DynamicImage) -> [Self; 3] {
        let regular = Self::new("Histogram".to_owned(), calculate_histogram(img));

        let cumulative = Self::new(
            "Cumulative".to_owned(),
            cumulative_histogram(&regular.raw_values),
        );

        let normalized_cumulative = Self::new(
            "Normalized cumulative".to_owned(),
            normalize_histogram(cumulative.raw_values),
        );

        [regular, cumulative, normalized_cumulative]
    }
}
