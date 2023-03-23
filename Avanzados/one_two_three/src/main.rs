#![allow(clippy::cast_sign_loss)]

use plotters::prelude::*;
use std::collections::VecDeque;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = (0, 0);
    let goal = (8, 5);

    if start == goal {
        println!("Start and goal are the same");
        return Ok(());
    }

    let path = bfs(start, goal);

    if path.is_empty() {
        println!("No path found");
        return Ok(());
    }
    println!("Path: {:?}", &path);

    let min_x = path.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_x = path.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;

    let min_y = path.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let max_y = path.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    let mut padding_x: i32 = ((max_x - min_x) as f32 * 0.25).floor() as i32;
    let mut padding_y: i32 = ((max_y - min_y) as f32 * 0.25).floor() as i32;

    if padding_x == 0 {
        padding_x += 1;
    } else if padding_y == 0 {
        padding_y += 1;
    }

    let root = SVGBackend::new("path.svg", (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Shortest Path", ("sans-serif", 40).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            (min_x - padding_x)..(max_x + padding_x),
            (min_y - padding_y)..(max_y + padding_y),
        )?;

    chart.configure_mesh().draw()?;

    chart.draw_series(PointSeries::of_element(
        path.into_iter().enumerate(),
        3,
        ShapeStyle::from(&RED).filled(),
        &|coord, size, style| {
            return EmptyElement::at(coord.1)
                + Circle::new((0, 0), size, style)
                + Text::new(
                    format!("{}", coord.0),
                    (0, -15),
                    ("sans-serif", 15).into_font(),
                );
        },
    ))?;

    Ok(())
}

fn bfs(start: (i32, i32), goal: (i32, i32)) -> Vec<(i32, i32)> {
    let size = (start.0)
        .abs()
        .max(start.1.abs())
        .max(goal.0.abs())
        .max(goal.1.abs())
        + 1000;

    // Make size 1 bigger than the max value of the coordinates and multiply by 2 for negative values
    let grid_size = (size * 2 + 1) as usize;

    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; grid_size]; grid_size];
    let mut prev = vec![vec![None; grid_size]; grid_size];

    queue.push_back((start, 0));
    // Add size to the coordinates to make them positive and center it in the grid
    visited[(start.0 + size) as usize][(start.1 + size) as usize] = true;
    // for i in &visited {
    //     println!("{i:?}");
    // }

    while let Some((current, n)) = queue.pop_front() {
        if current == goal {
            //  println!("Found goal");
            // create a vector with the final position
            let mut path = vec![current];
            // Get the previous node of the current node
            // which is located at the same position in the prev grid
            let mut prev_node = prev[(current.0 + size) as usize][(current.1 + size) as usize];

            // While there is a previous node, add it to the path
            while let Some(node) = prev_node {
                path.push(node);
                prev_node = prev[(node.0 + size) as usize][(node.1 + size) as usize];
            }

            // Reverse the path to get the correct order from start to finish
            path.reverse();
            return path;
        }

        // Get the neighbors of the current node
        for neighbor in get_neighbors(current, n + 1, size) {
            // If the neighbor is not visited, add it to the queue
            if !visited[(neighbor.0 + size) as usize][(neighbor.1 + size) as usize] {
                queue.push_back((neighbor, n + 1));

                // Mark the neighbor as visited and set the previous node
                visited[(neighbor.0 + size) as usize][(neighbor.1 + size) as usize] = true;
                prev[(neighbor.0 + size) as usize][(neighbor.1 + size) as usize] = Some(current);
            }
        }
    }

    vec![]
}

fn get_neighbors(point: (i32, i32), i: i32, size: i32) -> Vec<(i32, i32)> {
    let mut neighbors = Vec::new();

    // if neighbor is not out of bounds, add it to the neighbors vector
    if point.0 + i <= size {
        neighbors.push((point.0 + i, point.1));
    }
    if point.1 + i <= size {
        neighbors.push((point.0, point.1 + i));
    }
    if point.0 - i >= -size {
        neighbors.push((point.0 - i, point.1));
    }
    if point.1 - i >= -size {
        neighbors.push((point.0, point.1 - i));
    }

    neighbors
}
