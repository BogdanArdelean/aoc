use std::collections::{BTreeMap, HashSet};
use std::collections::Bound::Excluded;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;

enum Fold {
    ByX(i32),
    ByY(i32)
}

#[derive(Debug, Clone)]
struct Origami {
    by_x: BTreeMap<i32, Vec<i32>>,
    by_y: BTreeMap<i32, Vec<i32>>,
    paper: HashSet<(i32, i32)>,
}

impl Origami {
    fn new(points: Vec<(i32, i32)>) -> Self {
        let mut by_x: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
        let mut by_y: BTreeMap<i32, Vec<i32>> = BTreeMap::new();

        for (x, y) in &points {
            by_x.entry(*x).or_default().push(*y);
            by_y.entry(*y).or_default().push(*x);
        }

        let paper = HashSet::from_iter(points.into_iter());

        Self { by_x, by_y, paper }
    }

    fn count(&self) -> usize {
        self.paper.len()
    }

    fn fold_internal(map: &BTreeMap<i32, Vec<i32>>, line: i32) -> Vec<(i32, i32)> {
        let mut points = vec![];

        let mx = *map.keys().max().unwrap();
        let offset = if mx - line > line {
            mx - 2*line + 1
        } else {
            0
        };
        for (p1, p2s) in map.range(0..=line) {
            for p2 in p2s {
                points.push((p1 + offset, *p2));
            }
        }

        for (p1, p2s) in map.range((Excluded(line), std::collections::Bound::Unbounded)) {
            for p2 in p2s {
                points.push((line - (p1 - line) - offset, *p2));
            }
        }

        points
    }

    fn fold(&self, fold: &Fold) -> Self {
        match fold {
            Fold::ByX(line) => {
                Origami::new(Origami::fold_internal(&self.by_x, *line))
            }
            Fold::ByY(line) => {
                Origami::new(Origami::fold_internal(&self.by_y, *line).iter().map(|(a, b)| (*b, *a)).collect())
            }
        }
    }
}

fn parse(path: &Path) -> Result<(Origami, Vec<Fold>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut points = vec![];
    loop {
        let line = lines.next().unwrap()?;
        if line.is_empty() {
            break;
        }
        let point = scan_fmt::scan_fmt!(&line, "{},{}", i32, i32)?;
        points.push(point);
    }
    let origami = Origami::new(points);

    let mut folds = vec![];
    for line in lines {
        let line = line?;
        let (tp, line) = scan_fmt::scan_fmt!(&line, "fold along {}={}", char, i32)?;
        if tp == 'x' {
            folds.push(Fold::ByX(line));
        } else {
            folds.push(Fold::ByY(line));
        }
    }

    Ok((origami, folds))
}

fn part1(origami: &Origami, fold: &Fold) -> usize {
    let origami = origami.fold(fold);
    origami.count()
}

fn part2(origami: &Origami, fold: &Vec<Fold>) -> Origami {
    let mut o = origami.clone();

    for f in fold {
        o = o.fold(f);
    }

    o
}

fn display(o: &Origami) {
    let cols = *o.by_x.keys().max().unwrap();
    let rows = *o.by_y.keys().max().unwrap();

    for i in (0..=rows) {
        for j in (0..=cols) {
            if o.paper.contains(&(j,i)) {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!();
    }
}

fn main() -> Result<()> {
    let (origami, folds) = parse(&Path::new("input.txt"))?;
    let p1 = part1(&origami, folds.first().unwrap());
    println!("Part 1 {}", p1);

    let p2 = part2(&origami, &folds);
    println!("Part 2 {}", p2.count());
    display(&p2);
    Ok(())
}
