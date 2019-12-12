static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day12.txt");

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Moon {
    location: Vec3,
    velocity: Vec3,
}

impl Moon {
    fn kinetic_energy(&self) -> isize {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }

    fn potential_energy(&self) -> isize {
        self.location.x.abs() + self.location.y.abs() + self.location.z.abs()
    }

    fn total_energy(&self) -> isize {
        self.kinetic_energy() * self.potential_energy()
    }
}

fn parse(s: &str) -> Vec<Moon> {
    let mut v = Vec::new();
    for line in s.lines() {
        let stringified = line
            .chars()
            .filter(|c| c.is_digit(10) || c == &'-' || c == &',')
            .collect::<String>();

        let mut it = stringified.split(",");

        let x = it.next().unwrap().parse::<isize>().unwrap();
        let y = it.next().unwrap().parse::<isize>().unwrap();
        let z = it.next().unwrap().parse::<isize>().unwrap();

        let location = Vec3 { x, y, z };

        let velocity = Vec3 { x: 0, y: 0, z: 0 };

        v.push(Moon { location, velocity });
    }

    v
}

macro_rules! update_velocities {
    ($moons:expr, [$($axis:ident),*]) => {
        for i in 0..$moons.len() {
            for j in 0..$moons.len() {
                if i == j { continue }
                update_velocities!(@INNER i, j, $moons, [$($axis),*]);
            }
        }
    };
    (@INNER $fst:expr, $scnd:expr, $moons:expr, [$axis:ident $(,$axes:ident)*]) => {
        if $moons[$fst].location.$axis > $moons[$scnd].location.$axis {
            $moons[$fst].velocity.$axis -= 1;
            $moons[$scnd].velocity.$axis += 1;
        }

        if $moons[$fst].location.$axis < $moons[$scnd].location.$axis {
            $moons[$fst].velocity.$axis += 1;
            $moons[$fst].velocity.$axis -= 1;
        }

        update_velocities!(@INNER $fst, $scnd, $moons, [$($axes),*])
    };
    (@INNER $fst:expr, $scnd:expr, $moons:expr, []) => {()}
}

macro_rules! update_positions {
    ($moons:expr, [$($axis:ident),*] ) => {
        for moon in $moons.iter_mut() {
            $(moon.location.$axis += moon.velocity.$axis;)*
        }
    };
}

macro_rules! cycle {
    ($moons:expr, $axis:ident) => {{
        let start = [
            $moons[0].location.$axis,
            $moons[1].location.$axis,
            $moons[2].location.$axis,
            $moons[3].location.$axis,
        ];

        let mut idx = 1;
        loop {
            update_velocities!($moons, [$axis]);
            update_positions!($moons, [$axis]);

            idx += 1;
            if start
                == [
                    $moons[0].location.$axis,
                    $moons[1].location.$axis,
                    $moons[2].location.$axis,
                    $moons[3].location.$axis,
                ]
            {
                break;
            }
        }
        idx
    }};
}

fn part1(mut moons: Vec<Moon>) -> isize {
    for _ in 0..1000 {
        update_velocities!(&mut moons, [x, y, z]);
        update_positions!(&mut moons, [x, y, z]);
    }

    moons.iter().map(|moon| moon.total_energy()).sum()
}

fn part2(mut moons: Vec<Moon>) -> isize {
    let x = cycle!(&mut moons, x);
    let y = cycle!(&mut moons, y);
    let z = cycle!(&mut moons, z);

    let answer = lcm(lcm(x, y), z);
    answer
}

fn main() {
    let moons = parse(PUZZLE);

    let p1 = part1(moons.clone());
    let p2 = part2(moons);

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

#[inline]
fn gcd(s: isize, other: isize) -> isize {
    // Use Stein's algorithm
    let mut m = s;
    let mut n = other;
    if m == 0 || n == 0 {
        return (m | n).abs();
    }

    // find common factors of 2
    let shift: u32 = (m | n).trailing_zeros();

    // The algorithm needs positive numbers, but the minimum value
    // can't be represented as a positive one.
    // It's also a power of two, so the gcd can be
    // calculated by bitshifting in that case

    // Assuming two's complement, the number created by the shift
    // is positive for all numbers except gcd = abs(min value)
    // The call to .abs() causes a panic in debug mode
    if m == isize::min_value() || n == isize::min_value() {
        return 1 << shift;
    }

    // guaranteed to be positive now, rest like unsigned algorithm
    m = m.abs();
    n = n.abs();

    // divide n and m by 2 until odd
    m >>= m.trailing_zeros();
    n >>= n.trailing_zeros();

    while m != n {
        if m > n {
            m -= n;
            m >>= m.trailing_zeros();
        } else {
            n -= m;
            n >>= n.trailing_zeros();
        }
    }
    m << shift
}

#[inline]
fn lcm(s: isize, other: isize) -> isize {
    if s == 0 && other == 0 {
        return 0;
    }
    let gcd = gcd(s, other);
    // should not have to recalculate abs
    let lcm = (s * (other / gcd)).abs();
    lcm
}
