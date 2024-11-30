use std::{collections::HashSet, sync::LazyLock};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Coordinate {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) z: i32,
}

impl Coordinate {
    pub(crate) fn shift(&self, shift: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + shift.x,
            y: self.y + shift.y,
            z: self.z + shift.z,
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
pub(crate) struct Rotation([[i32; 3]; 3]);

pub(crate) static ALL_ROTATIONS: LazyLock<Vec<Rotation>> = LazyLock::new(|| {
    let mut all: HashSet<Rotation> = HashSet::new();

    let mut x = Rotation::empty();
    for _ in 0..4 {
        x = x.multiply(&Rotation::x90());

        let mut y = Rotation::empty();
        for _ in 0..4 {
            y = y.multiply(&Rotation::y90());

            let mut z = Rotation::empty();
            for _ in 0..4 {
                z = z.multiply(&Rotation::z90());
                let rotation = x.multiply(&y).multiply(&z);
                all.insert(rotation);
            }
        }
    }

    return all.into_iter().collect();
});

impl Rotation {
    fn from_matrix(matrix: [[i32; 3]; 3]) -> Self {
        Self(matrix)
    }

    fn empty() -> Rotation {
        Self::from_matrix([
            // The identity matrix.
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1],
        ])
    }

    fn x90() -> Rotation {
        Self::from_matrix([
            // Rotate 90 degrees about x-axis.
            [1, 0, 0],
            [0, 0, -1],
            [0, 1, 0],
        ])
    }

    fn y90() -> Rotation {
        Self::from_matrix([
            // Rotate 90 degrees about y-axis.
            [0, 0, 1],
            [0, 1, 0],
            [-1, 0, 0],
        ])
    }

    fn z90() -> Rotation {
        Self::from_matrix([
            // Rotate 90 degrees about z-axis.
            [0, -1, 0],
            [1, 0, 0],
            [0, 0, 1],
        ])
    }

    fn multiply(&self, o: &Rotation) -> Rotation {
        Rotation([
            [
                self.0[0][0] * o.0[0][0] + self.0[0][1] * o.0[1][0] + self.0[0][2] * o.0[2][0],
                self.0[0][0] * o.0[0][1] + self.0[0][1] * o.0[1][1] + self.0[0][2] * o.0[2][1],
                self.0[0][0] * o.0[0][2] + self.0[0][1] * o.0[1][2] + self.0[0][2] * o.0[2][2],
            ],
            [
                self.0[1][0] * o.0[0][0] + self.0[1][1] * o.0[1][0] + self.0[1][2] * o.0[2][0],
                self.0[1][0] * o.0[0][1] + self.0[1][1] * o.0[1][1] + self.0[1][2] * o.0[2][1],
                self.0[1][0] * o.0[0][2] + self.0[1][1] * o.0[1][2] + self.0[1][2] * o.0[2][2],
            ],
            [
                self.0[2][0] * o.0[0][0] + self.0[2][1] * o.0[1][0] + self.0[2][2] * o.0[2][0],
                self.0[2][0] * o.0[0][1] + self.0[2][1] * o.0[1][1] + self.0[2][2] * o.0[2][1],
                self.0[2][0] * o.0[0][2] + self.0[2][1] * o.0[1][2] + self.0[2][2] * o.0[2][2],
            ],
        ])
    }

    pub(crate) fn rotate(&self, coord: Coordinate) -> Coordinate {
        Coordinate {
            x: self.0[0][0] * coord.x + self.0[0][1] * coord.y + self.0[0][2] * coord.z,
            y: self.0[1][0] * coord.x + self.0[1][1] * coord.y + self.0[1][2] * coord.z,
            z: self.0[2][0] * coord.x + self.0[2][1] * coord.y + self.0[2][2] * coord.z,
        }
    }
}
