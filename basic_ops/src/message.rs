#[derive(Copy, Clone)]
pub enum Message {
    Changed,
    Open,
    SaveAs,
    Quit,
    Copy,
    MirrorVertical,
    MirorrHorizontal,
    GrayScale,
    Quantize,
    About,
}
