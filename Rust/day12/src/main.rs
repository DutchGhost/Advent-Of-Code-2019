static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day12.txt");

#[derive(Debug)]
struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug)]
struct Moon {
    location: Vec3,
    velocity: Vec3,
}

fn parse(s: &str) -> Vec<Moon> {
    let mut v = Vec::new();
    for line in s.lines() {
        let stringified = line.chars().filter(|c| c.is_digit(10) || c == &'-' || c == &',')
        .collect::<String>();

        let mut it = stringified.split(",");

        let x = it.next().unwrap().parse::<isize>().unwrap();
        let y = it.next().unwrap().parse::<isize>().unwrap();
        let z = it.next().unwrap().parse::<isize>().unwrap();

        let location = Vec3 {
            x,
            y,
            z
        };

        let velocity = Vec3 {
            x: 0,
            y: 0,
            z: 0,
        };

        v.push(Moon { location, velocity});
    }

    v
}

macro_rules! update_velocity {
    ($fst:expr, $scnd:expr, $moons:expr, [$axe:ident $(,$axis:ident)*]) => {
        if $moons[$fst].location.$axe > $moons[$scnd].location.$axe {
            $moons[$fst].velocity.$axe -= 1;
            $moons[$scnd].velocity.$axe += 1;
        }

        if $moons[$fst].location.$axe < $moons[$scnd].location.$axe {
            $moons[$fst].velocity.$axe += 1;
            $moons[$fst].velocity.$axe -= 1;
        }

        update_velocity!($fst, $scnd, $moons, [$($axis),*])
    };
    ($fst:expr, $scnd:expr, $moons:expr, []) => {()}
}

fn update_velocity(v: &mut [Moon]) {
    for i in 0..v.len() {
        for j in 0..v.len() {
            if i == j { continue }
            update_velocity!(i, j, v, [x, y, z]);
        }
    }
}

fn update_position(v: &mut [Moon]) {
    for moon in v.iter_mut() {
        moon.location.x += moon.velocity.x;
        moon.location.y += moon.velocity.y;
        moon.location.z += moon.velocity.z;
    }
}

fn kinetic_enery(v: &mut [Moon]) -> usize {
    v.iter().map(|moon| {
        let potential = moon.location.x.abs() + moon.location.y.abs() + moon.location.z.abs();
        let kinetic = moon.velocity.x.abs() + moon.velocity.y.abs() + moon.velocity.z.abs();

        (potential * kinetic) as usize
    }).sum::<usize>()
}

fn main() {
    let mut v = parse(PUZZLE);

    for _ in 0..1000 {
        update_velocity(&mut v);
        update_position(&mut v);
    }

    let e = kinetic_enery(&mut v);
    dbg!(e);
}
