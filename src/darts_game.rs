struct Darts {
    height: f64,
    speed: f64,
    angle: f64,
}

impl Darts {
    fn new(height: f64, speed: f64, angle: f64) -> Darts {
        Darts {
            height,
            speed,
            angle,
        }
    }
}

struct Target {
    distance: f64,
    height: f64,
    diameter: f64,
}

impl Target {
    fn new(distance: f64, height: f64, diameter: f64) -> Target {
        Target {
            distance,
            height,
            diameter,
        }
    }
}

struct Game {
    darts: Darts,
    target: Target,
}

const gravity: f64 = 9.8;

impl Game {
    fn new(darts: Darts, target: Target) -> Game {
        Game { darts, target }
    }

    fn solve(&self) -> Option<f64> {
        let arrive = self.darts.height + self.target.distance * tan(self.darts.angle) - (gravity * self.target.distance * self.target.distance / (2.0 * self.darts.speed * self.darts.speed * cos(self.darts.angle) * cos(self.darts.angle)));
        let diff = (arrive - self.target.height).abs();
        if diff < self.target.diameter / 2.0 {
            Some(diff)
        } else {
            None
        }
    }
}

fn cos(angle: f64) -> f64 {
    let rad = radian(angle);
    rad.cos()
}

fn tan(angle: f64) -> f64 {
    let rad = radian(angle);
    rad.tan()
}

fn radian(angle: f64) -> f64 {
    2.0 * std::f64::consts::PI * angle / 360.0
}

#[cfg(test)]
mod tests {
    use super::{Darts, Game, Target};

    #[test]
    fn test0() {
        let darts = Darts::new(10.0, 10.0, 10.0);
        let target = Target::new(10.0, 10.0, 10.0);
        let game = Game::new(darts, target);
        let want = 3.3;
        let got = game.solve().unwrap();
        let diff = (want-got).abs();
        assert!(diff < 0.1);
    }

    #[test]
    fn test1(){
        let darts = Darts::new(10.0, 15.0, 45.0);
        let target = Target::new(10.0, 10.0, 10.0);
        let game = Game::new(darts, target);
        assert_eq!(None, game.solve());
    }
}
