#[derive(Debug, Clone, Hash, Eq)]
pub enum Calculation {
    Add(Box<Calculation>, Box<Calculation>),
    Sub(Box<Calculation>, Box<Calculation>),
    Mul(Box<Calculation>, Box<Calculation>),
    Div(Box<Calculation>, Box<Calculation>),
    Cube(usize, u64),
}

impl Calculation {
    pub fn score(&self) -> u32 {
        match self {
            Calculation::Add(a, b) => 20 + a.score() + b.score(),
            Calculation::Sub(a, b) => 21 + a.score() + b.score(),
            Calculation::Mul(a, b) => 30 + a.score() + b.score(),
            Calculation::Div(a, b) => 34 + a.score() + b.score(),
            Calculation::Cube(_, v) => 10 + 2 * v.ilog10(),
        }
    }
}

impl PartialOrd for Calculation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.score() <= other.score() {
            return Some(std::cmp::Ordering::Less);
        } else {
            return Some(std::cmp::Ordering::Greater);
        }
    }
}

impl Ord for Calculation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.score() <= other.score() {
            return std::cmp::Ordering::Less;
        } else {
            return std::cmp::Ordering::Greater;
        }
    }
}

impl PartialEq for Calculation {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Calculation::Cube(_, v) => {
                if let Calculation::Cube(_, w) = other {
                    return v == w;
                }
            }

            Calculation::Add(a, b) => {
                if let Calculation::Add(c, d) = other {
                    return (*a == *c && *b == *d) || (*a == *d && *b == *c);
                }
            }

            Calculation::Sub(a, b) => {
                if let Calculation::Sub(c, d) = other {
                    return *a == *c && *b == *d;
                }
            }

            Calculation::Mul(a, b) => {
                if let Calculation::Mul(c, d) = other {
                    return (*a == *c && *b == *d) || (*a == *d && *b == *c);
                }
            }

            Calculation::Div(a, b) => {
                if let Calculation::Div(c, d) = other {
                    return *a == *c && *b == *d;
                }
            }
        }
        return false;
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

impl std::fmt::Display for Calculation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Calculation::Add(a, b) => write!(f, "({} + {})", a, b),
            Calculation::Sub(a, b) => write!(f, "({} - {})", a, b),
            Calculation::Mul(a, b) => write!(f, "({} * {})", a, b),
            Calculation::Div(a, b) => write!(f, "({} / {})", a, b),
            Calculation::Cube(_, c) => write!(f, "{}", c),
        }
    }
}
