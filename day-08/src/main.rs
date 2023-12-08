use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (directions, map) = INPUT.split_once("\n\n").expect("invalid input");
    let directions = directions
        .chars()
        .map(|c| Direction::try_from(c).expect("invalid input"));

    let directions = std::iter::repeat(directions).flatten();

    let nodes = map
        .lines()
        .map(|line| {
            let node = Node::try_from(line).expect("invalid map input");
            (node.name, node)
        })
        .collect::<HashMap<[char; 3], Node>>();

    let mut current_node = nodes.get(&['A', 'A', 'A']).expect("invalid start node");

    let path = directions
        .map_while(|direction| {
            if current_node.name == ['Z', 'Z', 'Z'] {
                return None;
            }

            let next_node = current_node.get_next_node(direction);
            current_node = nodes.get(&next_node).expect("node not found");

            Some(next_node)
        })
        .count();

    dbg!(path);
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err("invalid direction"),
        }
    }
}

#[derive(Debug)]
struct Node {
    name: [char; 3],
    left: [char; 3],
    right: [char; 3],
}

impl Node {
    fn get_next_node(&self, direction: Direction) -> [char; 3] {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

impl TryFrom<&str> for Node {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (name, next_nodes) = value.split_once(" = ").ok_or("equals format")?;

        fn to_char_slice(s: &str) -> Result<[char; 3], &'static str> {
            s.chars()
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| "node name format invalid")
        }

        let (left, right) = next_nodes
            .strip_prefix('(')
            .ok_or("no opening parentheses")?
            .strip_suffix(')')
            .ok_or("no closing parentheses")?
            .split_once(", ")
            .ok_or("not comma separated")?;

        Ok(Self {
            name: to_char_slice(name)?,
            left: to_char_slice(left)?,
            right: to_char_slice(right)?,
        })
    }
}
