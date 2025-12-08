mod point_3d;
mod union_find;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use point_3d::Point3D;
use union_find::UnionFind;

struct Edge {
    a: usize,
    b: usize,
    weight: u64,
}

impl Edge {
    const fn new(a: usize, b: usize, weight: u64) -> Self {
        Self { a, b, weight }
    }
}

fn build_distance_matrix(points: &[Point3D]) -> Vec<Edge> {
    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let dx: u64 = (points[i].x - points[j].x).unsigned_abs() as u64;
            let dy: u64 = (points[i].y - points[j].y).unsigned_abs() as u64;
            let dz: u64 = (points[i].z - points[j].z).unsigned_abs() as u64;

            edges.push(Edge::new(i, j, dx * dx + dy * dy + dz * dz));
        }
    }
    edges
}

fn largest_3_components_multiplied(points: &[Point3D], edges: &[Edge]) -> usize {
    let size: usize = points.len();
    let mut union_find: UnionFind = UnionFind::new(size);

    for edge in edges.iter().take(1000) {
        union_find.union(edge.a, edge.b);
    }

    let mut component_sizes: Vec<usize> = union_find.all_sizes();
    component_sizes.sort_unstable_by(|a: &usize, b: &usize| b.cmp(a));
    component_sizes.iter().take(3).product()
}

fn multiply_last_x_elements(points: &[Point3D], edges: &[Edge]) -> isize {
    let size: usize = points.len();
    let mut union_find: UnionFind = UnionFind::new(points.len());
    let mut remaining_components: usize = size;

    for edge in edges {
        if union_find.union(edge.a, edge.b) {
            remaining_components -= 1;

            if remaining_components == 1 {
                return points[edge.a].x * points[edge.b].x;
            }
        }
    }

    0
}

fn main() {
    let mut points: Vec<Point3D> = Vec::new();

    for line in BufReader::new(File::open("src/Day 8/input.txt").expect("Can't open file")).lines()
    {
        match line {
            Ok(content) => {
                let numbers: Vec<isize> = content
                    .split(',')
                    .map(|s: &str| s.trim().parse::<isize>().expect("Failure to parse integer"))
                    .collect();
                if numbers.len() == 3 {
                    points.push(Point3D::new(numbers[0], numbers[1], numbers[2]));
                } else {
                    eprintln!("Expected 3 numbers per line, got {numbers:?}");
                }
            }
            Err(e) => eprintln!("Error reading line: {e}"),
        }
    }

    let mut edges: Vec<Edge> = build_distance_matrix(&points);
    edges.sort_unstable_by_key(|edge: &Edge| edge.weight);

    println!(
        "The product of the sizes of the three largest components is {}",
        largest_3_components_multiplied(&points, &edges)
    );

    println!(
        "The product of the x coordinates of the first two points is {}",
        multiply_last_x_elements(&points, &edges)
    );
}
