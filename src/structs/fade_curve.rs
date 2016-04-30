use meval::Expr;

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
pub enum FadeCurve {
    Linear,
    Squared,
    SquareRoot,
    Sin(u8),
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
            FadeCurve::Sin(i) => Expr::from_str("-cos(".to_string() + &i.to_string() + &".5*6.28318530718*x)*0.5+0.5".to_string()).unwrap().bind("x").unwrap(),
            FadeCurve::Custom(e) => Expr::from_str(e).unwrap().bind("x").unwrap()
        }

    }
}
