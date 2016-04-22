
extern crate meval;
pub use meval::*;

pub type FadeTime = u16;
pub type DmxChannel = usize;
pub type DmxValue = u8;

pub mod fixture;
pub use fixture::*;

pub mod stage;
pub use stage::*;

pub mod fade_curve;
pub use fade_curve::*;

#[test]
fn test_fade_curve() {
    let curve = FadeCurve::Custom("m * sin(x)".to_string());
    let timeframe = 1000.0; //ms
    for i in 1..255 {
        let expr = curve.clone().to_expression(timeframe as FadeTime, i).unwrap();
        let value = Some(timeframe).map(&*expr).unwrap();
        assert_eq!(value as DmxValue, i as DmxValue);
    }
}
