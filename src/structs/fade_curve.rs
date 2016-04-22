
use meval::{Expr, Error};
use FadeTime;
use DmxValue;

#[derive(Debug, Clone)]
pub enum FadeCurve {
    Linear,
    Exponential,
    AntiExponential,
    Logarithmic,
    Custom(String)
}

impl FadeCurve {
    pub fn to_expression(self, fade_time: FadeTime, delta: DmxValue) -> Result<Box<Fn(f64) -> f64>, Error> {
        let expression = match self {
            FadeCurve::Linear => "m * x".to_string(),
            FadeCurve::Exponential => "m * (x^2)".to_string(),
            FadeCurve::AntiExponential => "m * ln(x)".to_string(),
            FadeCurve::Logarithmic => "m * log(x)".to_string(),
            FadeCurve::Custom(e) => e
        };
        let factor_func = Expr::from_str(&format!("{} / ({})", delta, expression)).unwrap().bind_with_context(("m", 1.), "x").unwrap();
        let factor = ("m", Some(fade_time as f64).map(&*factor_func).unwrap());
        Expr::from_str(expression).unwrap().bind_with_context(factor, "x")
    }
}
