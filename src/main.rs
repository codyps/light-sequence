use rand::Rng;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Color {
    Red,
    Blue,
    Yellow,
    White,
    Orange,
    Pink,
    Teal,
    Green,
}

impl Color {
    fn is_allowed_adjacent(self, other: Color) -> bool {
        use Color::*;
        if other == self {
            return false;
        }

        let forbidden_pairs = [(Red, Orange), (Pink, Red), (Green, Teal), (Yellow, White)];

        for fp in forbidden_pairs {
            if fp.0 == self && fp.1 == other || fp.1 == self && fp.0 == other {
                return false;
            }
        }

        true
    }
}

fn main() {
    use Color::*;
    let mut slots = 100;
    let mut lights = [(Red, 25), (Blue, 25), (Green, 25), (Orange, 25)];
    let mut prev_light: Option<Color> = None;
    let mut rng = rand::thread_rng();

    while slots != 0 {
        let mut rem_lights = 0;
        for l in lights {
            rem_lights += l.1;
        }

        // 1. pick randomly from the full set of remaining lights
        let which_light: usize = rng.gen_range(0..rem_lights);

        let group = {
            let mut c = 0;
            let mut group = None;

            for l in &mut lights {
                let n = c + l.1;
                if which_light >= n {
                    c = n;
                    continue;
                }

                group = Some(l);
                break;
            }

            group.unwrap()
        };

        // 2. discard pick if it is not allowed adjacent to previous
        if let Some(prev) = prev_light {
            if !prev.is_allowed_adjacent(group.0) {
                continue;
            }
        }

        prev_light = Some(group.0);
        group.1 -= 1;

        println!("{:?}", group.0);

        slots -= 1;
    }
}
