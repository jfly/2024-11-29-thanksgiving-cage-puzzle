mod coordinates;

use std::collections::HashSet;

use coordinates::Coordinate;
use coordinates::Rotation;
use coordinates::ALL_ROTATIONS;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Hitmap(i32);

impl Hitmap {
    fn from_coordinates(coords: Vec<Coordinate>) -> Self {
        let mut hitmap = Hitmap(0);
        for coord in coords {
            hitmap = hitmap.add(coord);
        }

        return hitmap;
    }

    fn add(self, coord: Coordinate) -> Self {
        let index = Self::coordinate_to_index(&coord);
        Hitmap(self.0 | (1 << index))
    }

    fn empty() -> Self {
        return Self::from_coordinates(Vec::new());
    }

    fn rotate(&self, rotation: &Rotation) -> Hitmap {
        let rotated_coords: Vec<Coordinate> = self
            .coordinates()
            .into_iter()
            .map(|coord| rotation.rotate(coord))
            .collect();

        return Self::from_coordinates(rotated_coords);
    }

    fn shift(&self, shift: Coordinate) -> Hitmap {
        let shifted_coords: Vec<Coordinate> = self
            .coordinates()
            .into_iter()
            .map(|coord| coord.shift(shift))
            .collect();

        return Self::from_coordinates(shifted_coords);
    }

    fn coordinates(&self) -> Vec<Coordinate> {
        let mut coords = Vec::new();

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let coordinate = Coordinate { x, y, z };
                    let index = Self::coordinate_to_index(&coordinate);

                    if self.0 & (1 << index) != 0 {
                        coords.push(coordinate);
                    }
                }
            }
        }

        coords
    }

    fn coordinate_to_index(coord: &Coordinate) -> i32 {
        assert!(coord.x == -1 || coord.x == 0 || coord.x == 1);
        assert!(coord.y == -1 || coord.y == 0 || coord.y == 1);
        assert!(coord.z == -1 || coord.z == 0 || coord.z == 1);

        coord.x + 1 + 3 * (coord.y + 1) + 9 * (coord.z + 1)
    }
}

struct HitmapBuilder {
    hitmap: Hitmap,
    coordinate: Coordinate,
}

impl HitmapBuilder {
    fn new(coordinate: Coordinate) -> Self {
        Self {
            hitmap: Hitmap::empty(),
            coordinate,
        }
        .teleport(coordinate)
    }

    fn teleport(mut self, coordinate: Coordinate) -> Self {
        self.coordinate = coordinate;
        self.hitmap = self.hitmap.add(coordinate);
        self
    }

    fn shift(self, amount: Coordinate) -> Self {
        let new_coordinate = self.coordinate.shift(amount);
        self.teleport(new_coordinate)
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Cage {
    hitmap: Hitmap,
    pieces: Vec<Hitmap>,
}

struct PieceDoesNotFit;

impl Cage {
    fn new() -> Self {
        Cage {
            hitmap: Hitmap::empty(),
            pieces: Vec::new(),
        }
    }

    fn add(&self, piece: Hitmap) -> Result<Cage, PieceDoesNotFit> {
        // If this piece intersects with the stuff already in the
        // cage, then it can't fit!
        if self.hitmap.0 & piece.0 != 0 {
            return Err(PieceDoesNotFit);
        }

        let mut new_pieces = self.pieces.clone();
        new_pieces.push(piece);
        new_pieces.sort();

        Ok(Cage {
            hitmap: Hitmap(self.hitmap.0 | piece.0),
            pieces: new_pieces,
        })
    }

    fn canonicalize(&self) -> Cage {
        let mut canon_cage = self.clone();
        for rotation in &*ALL_ROTATIONS {
            let new_hitmap = self.hitmap.rotate(rotation);
            if new_hitmap <= canon_cage.hitmap {
                let new_pieces = self
                    .pieces
                    .clone()
                    .into_iter()
                    .map(|piece| piece.rotate(rotation))
                    .collect();

                if new_hitmap == canon_cage.hitmap {
                    if new_hitmap >= canon_cage.hitmap {
                        continue;
                    }
                }

                canon_cage.hitmap = new_hitmap;
                canon_cage.pieces = new_pieces;
            }
        }
        return canon_cage;
    }
}

struct Search {
    all_pieces: HashSet<Hitmap>,
}

impl Search {
    fn new() -> Self {
        let mut all_pieces = HashSet::new();

        let x = Coordinate { x: 1, y: 0, z: 0 };
        let y = Coordinate { x: 0, y: 1, z: 0 };
        let z = Coordinate { x: 0, y: 0, z: 1 };
        let corner = Coordinate {
            x: -1,
            y: -1,
            z: -1,
        };
        let mut builder = HitmapBuilder::new(corner);
        builder = builder.shift(x);
        builder = builder.shift(x);
        builder = builder.shift(z);
        builder = builder.teleport(corner);
        builder = builder.shift(z);
        builder = builder.teleport(corner);
        builder = builder.shift(y);
        builder = builder.shift(y);

        let piece1 = builder.hitmap;

        // There's only 1 shift we can do to the piece that lets it still fit. Everything else is
        // rotations.
        let piece2 = piece1.shift(z);

        for rotation in &*ALL_ROTATIONS {
            all_pieces.insert(piece1.rotate(rotation));
            all_pieces.insert(piece2.rotate(rotation));
        }

        Self { all_pieces }
    }

    fn search(self) -> HashSet<Cage> {
        let mut fringe: Vec<Cage> = vec![Cage::new()];

        let mut canonical_end_states = HashSet::new();

        loop {
            let cage = match fringe.pop() {
                None => break,
                Some(cage) => cage,
            };

            let mut is_end_state = true;
            for piece in &self.all_pieces {
                match cage.add(*piece) {
                    Err(PieceDoesNotFit) => continue,
                    Ok(new_cage) => {
                        is_end_state = false;
                        fringe.push(new_cage);
                    }
                }
            }

            if is_end_state {
                let cage = cage.canonicalize();
                canonical_end_states.insert(cage.clone());
            }
        }

        canonical_end_states
    }
}

fn main() {
    let search = Search::new();

    let solutions = search.search();
    for solution in solutions {
        if solution.pieces.len() == 3 {
            println!("Found a solution!");
            for piece in solution.pieces {
                println!("{:?}", piece.coordinates());
            }
        }
    }
}
