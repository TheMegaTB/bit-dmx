
use meval::Expr;

#[derive(Debug, Clone)]
pub enum FadeCurve {
    Linear,
    Squared,
    SquareRoot,
    Custom(String)
}

fn linear(x: f64) -> f64 {
    x
}

fn squared(x: f64) -> f64 {
    x * x
}

fn square_root(x: f64) -> f64 {
    x.sqrt()
}

impl FadeCurve {
    pub fn to_function(self) -> Box<Fn(f64) -> f64> {
        match self {
            FadeCurve::Linear => Box::new(linear),
            FadeCurve::Squared => Box::new(squared),
            FadeCurve::SquareRoot => Box::new(square_root),
            FadeCurve::Custom(e) => Expr::from_str(e).unwrap().bind("x").unwrap()
        }

    }
}
