use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;
type Image = Vec<Vec<char>>;

fn expand(image: &Image, round: i32) -> Image {
    let mut expanded = image.clone();
    let expanded_cols = image[0].len();
    let elm = if round % 2 == 0 {
        '#'
    } else {
        '.'
    };

    expanded.insert(0, vec![elm; expanded_cols + 4]);
    expanded.insert(0, vec![elm; expanded_cols + 4]);
    for i in 2..expanded.len() {
        expanded[i].insert(0, elm);
        expanded[i].insert(0, elm);
        expanded[i].push(elm);
        expanded[i].push(elm);
    }
    expanded.push(vec![elm; expanded_cols + 4]);
    expanded.push(vec![elm; expanded_cols + 4]);

    expanded
}

fn enhance(image: &Image, algorithm: &Vec<char>, round: i32) -> Image {
    let image = expand(image, round);
    let mut enhanced = image.clone();
    for i in 1..image.len() - 1 {
        for j in 1..image[0].len() - 1 {
            let mut index: usize = 0;

            for k_i in -1i32..=1 {
                for k_j in -1i32..=1 {
                    let ii = i as i32 + k_i;
                    let jj = j as i32 + k_j;
                    index = index << 1;
                    index = index | if image[ii as usize][jj as usize] == '.' {
                        0
                    } else {
                        1
                    };
                }
            }

            enhanced[i][j] = algorithm[index];
        }
    }

    enhanced.remove(0);
    enhanced.pop();

    for i in 0..enhanced.len() {
        enhanced[i].remove(0);
        enhanced[i].pop();
    }

    enhanced
}

fn print(image: &Image) {
    for row in image {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

fn part1(image: &Image, algorithm: &Vec<char>) -> usize {
    let image = enhance(image, algorithm, 1);
    print(&image);

    println!();
    println!();

    let image = enhance(&image, algorithm, 2);
    print(&image);

    image
        .iter()
        .flat_map(|x| x.iter())
        .filter(|x| **x == '#')
        .count()
}

fn part2(image: &Image, algorithm: &Vec<char>) -> usize {
    let mut image = image.clone();
    for i in 1..=50 {
        image = enhance(&image, algorithm, i);
    }
    print(&image);
    image
        .iter()
        .flat_map(|x| x.iter())
        .filter(|x| **x == '#')
        .count()
}

fn parse(path: &Path) -> Result<(Vec<char>, Image)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let algorithm = lines.next().unwrap()?.chars().collect();
    lines.next();

    let mut image = Image::new();
    for line in lines {
        let line = line?;
        image.push(line.chars().collect());
    }

    Ok((algorithm, image))
}

fn main() -> Result<()> {
    let (algo, image) = parse(&Path::new("input.txt"))?;
    let p1 = part1(&image, &algo);
    println!("Part 1 {}", p1);

    let p2 = part2(&image, &algo);
    println!("Part 2 {}", p2);
    Ok(())
}
