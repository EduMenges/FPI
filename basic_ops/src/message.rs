#[derive(Copy, Clone)]
pub enum Message {
    Open,
    SaveAs,
    Quit,
    Copy,
    FlipVertical,
    FlipHorizontal,
    GrayScale,
    Quantize,
    About,
}
