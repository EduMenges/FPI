use basic_ops::histogram::{calculate_histogram, cumulative_histogram, normalize_histogram};
use egui::{
    plot::{Bar, BarChart, Plot},
    Context, Window, Id,
};

use crate::image_wrapper::ImageWrapper;

#[derive(Clone)]
pub struct HistogramGraph {
    pub open: bool,
    title: String,
    name: String,
    raw_values: [u32; 256],
}

impl Default for HistogramGraph {
    fn default() -> Self {
        Self {
            open: false,
            title: Default::default(),
            name: Default::default(),
            raw_values: [Default::default(); 256],
        }
    }
}

impl HistogramGraph {
    pub const fn new(title: String, name: String, histogram: [u32; 256]) -> Self {
        Self {
            open: false,
            title,
            name,
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

        Window::new(&self.title).id(Id::new(&self.name))
            .open(&mut self.open)
            .show(ctx, |ui| {
                Plot::new(&self.name)
                    .show(ui, |plot_ui| plot_ui.bar_chart(chart))
            });
    }

    pub fn all_histograms(img: &ImageWrapper) -> [Self; 3] {
        let regular = Self::new(
            "Histogram".to_owned(),
            format!("{}-histogram", img.name()),
            calculate_histogram(&img.img.to_luma_alpha8()),
        );

        let cumulative = Self::new(
            "Cumulative".to_owned(),
            format!("{}-cumulative", img.name()),
            cumulative_histogram(regular.raw_values),
        );

        let normalized_cumulative = Self::new(
            "Normalized cumulative".to_owned(),
            format!("{}-normalized", img.name()),
            normalize_histogram(cumulative.raw_values),
        );

        [regular, cumulative, normalized_cumulative]
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}
