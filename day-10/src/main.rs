use std::{
    collections::HashMap,
    fmt::{Debug, Write},
};

use petgraph::Undirected;

const INPUT: &str = include_str!("input.txt");

enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
}

impl Pipe {
    fn connects(&self) -> ((i32, i32), (i32, i32)) {
        match self {
            Pipe::NorthSouth => ((0, -1), (0, 1)),
            Pipe::EastWest => ((1, 0), (-1, 0)),
            Pipe::NorthEast => ((0, -1), (1, 0)),
            Pipe::NorthWest => ((0, -1), (-1, 0)),
            Pipe::SouthWest => ((0, 1), (-1, 0)),
            Pipe::SouthEast => ((0, 1), (1, 0)),
            Pipe::Start => unreachable!("start has no connection info"),
        }
    }

    fn connects_relative(&self, coords: (i32, i32)) -> ((i32, i32), (i32, i32)) {
        let (one, two) = self.connects();

        (
            (one.0 + coords.0, one.1 + coords.1),
            (two.0 + coords.0, two.1 + coords.1),
        )
    }
}

impl TryFrom<char> for Pipe {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Pipe::NorthSouth),
            '-' => Ok(Pipe::EastWest),
            'L' => Ok(Pipe::NorthEast),
            'J' => Ok(Pipe::NorthWest),
            '7' => Ok(Pipe::SouthWest),
            'F' => Ok(Pipe::SouthEast),
            'S' => Ok(Pipe::Start),
            _ => Err("Invalid pipe character"),
        }
    }
}

impl Debug for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Pipe::NorthSouth => '│',
            Pipe::EastWest => '─',
            Pipe::NorthEast => '└',
            Pipe::NorthWest => '┘',
            Pipe::SouthWest => '┐',
            Pipe::SouthEast => '┌',
            Pipe::Start => 'S',
        };

        f.write_char(char)
    }
}

struct Maze {
    pipes: petgraph::Graph<Pipe, usize, Undirected>,
    start: petgraph::graph::NodeIndex,
}

impl TryFrom<&str> for Maze {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut graph = petgraph::Graph::new_undirected();
        let mut start: Option<petgraph::graph::NodeIndex> = None;

        let mut pipes = HashMap::<(i32, i32), petgraph::graph::NodeIndex>::new();

        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Ok(pipe) = Pipe::try_from(c) {
                    match pipe {
                        Pipe::Start => {
                            if start.is_some() {
                                return Err("Multiple starts found");
                            }
                            let start_index = graph.add_node(pipe);
                            pipes.insert((x as i32, y as i32), start_index);
                            start = Some(start_index);
                        }
                        _ => {
                            let index = graph.add_node(pipe);
                            pipes.insert((x as i32, y as i32), index);
                        }
                    }
                }
            }
        }

        for (coords, node_index) in pipes.clone().into_iter() {
            let pipe = &graph[node_index];

            match pipe {
                Pipe::Start => continue,
                _ => {}
            }

            let get_connection = |graph: &petgraph::prelude::Graph<Pipe, usize, Undirected>,
                                  coords: (i32, i32), to: (i32, i32)|
             -> Option<petgraph::graph::NodeIndex> {
                if let Some(pipe) = pipes.get(&coords) {
                    let other_pipe = &graph[*pipe];
                    match other_pipe {
                        Pipe::Start => return Some(*pipe),
                        _ => {}
                    }

                    let (other_connection_one, other_connection_two) =
                        other_pipe.connects_relative(coords);

                    if other_connection_one == to || other_connection_two == to {
                        Some(*pipe)
                    } else {
                        None
                    }
                } else {
                    None
                }
            };

            let (one, two) = pipe.connects_relative(coords);
            if let Some(pipe) = get_connection(&graph, one, coords) {
                graph.update_edge(node_index, pipe, 1);
            }
            if let Some(pipe) = get_connection(&graph, two, coords) {
                graph.update_edge(node_index, pipe, 1);
            }
        }

        Ok(Maze {
            pipes: graph,
            start: start.ok_or("No start found")?,
        })
    }
}

fn main() {
    let maze = Maze::try_from(INPUT).expect("invalid input");
    let shortest_path_map =
        petgraph::algo::k_shortest_path(&maze.pipes, maze.start, None, 1, |_| 1);

    let farthest_point = shortest_path_map
        .values()
        .max()
        .expect("path to have at least one step");
    dbg!(farthest_point);
}
