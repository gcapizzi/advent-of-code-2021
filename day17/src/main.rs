use anyhow::Result;
use std::ops::Range;

fn main() -> Result<()> {
    let dx = 155..216;
    let dy = -132..-71;

    let mut count = 0;
    // ranges were manually tweaked after the fact 😬
    for y in -132..132 {
        for x in 18..216 {
            if let Some(t) = Trajectory::new(x, y).to_range(&dx, &dy) {
                count += 1;
                dbg!(count);
                dbg!(x, y);
                dbg!(t.iter().max_by_key(|p| p.1));
            }
        }
    }

    Ok(())
}

struct Trajectory {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Trajectory {
    fn new(vx: i32, vy: i32) -> Trajectory {
        Trajectory {
            pos: (0, 0),
            vel: (vx, vy),
        }
    }

    fn to_range(self, dx: &Range<i32>, dy: &Range<i32>) -> Option<Vec<(i32, i32)>> {
        let ps = self
            .take_while(|(x, y)| if &dx.end < &0 {
                x >= &dx.start
            } else {
                x < &dx.end
            } && if &dy.end < &0 {
                y >= &dy.start
            } else {
                y < &dy.end
            }).collect::<Vec<(i32, i32)>>();

        let last = ps.last()?;

        if dx.contains(&last.0) && dy.contains(&last.1) {
            Some(ps)
        } else {
            None
        }
    }
}

impl Iterator for Trajectory {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.pos;
        let (vx, vy) = self.vel;
        self.pos = (x + vx, y + vy);

        let new_vx = if vx > 0 {
            vx - 1
        } else if vx < 0 {
            vx + 1
        } else {
            vx
        };
        let new_vy = vy - 1;
        self.vel = (new_vx, new_vy);

        Some((x, y))
    }
}

#[cfg(test)]
mod tests {
    use crate::Trajectory;

    #[test]
    fn test_trajectory() {
        assert_eq!(
            Trajectory::new(7, 2).to_range(&(20..31), &(-10..-4)),
            Some(vec![
                (0, 0),
                (7, 2),
                (13, 3),
                (18, 3),
                (22, 2),
                (25, 0),
                (27, -3),
                (28, -7)
            ])
        );
    }
}
