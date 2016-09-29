use graphics::{OutColor, OutDepth, Encoder};

pub enum ToRender {
    GraphicsData(OutColor, OutDepth),
    Encoder(Encoder),
}
