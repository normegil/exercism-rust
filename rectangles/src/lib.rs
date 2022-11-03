use core::panic;

pub fn count(lines: &[&str]) -> u32 {
    let mut rectangles: Vec<Rectangle> = Vec::new();

    let symbol_vector = to_symbol_vector(lines);
    for (x, line) in symbol_vector.iter().enumerate() {
        for (y, symbol) in line.iter().enumerate() {
            match symbol {
                Some(Symbol::Vertice) => {
                    let current_vertice = Vertice::new(x, y);
                    if rectangles.iter().any(|r| r.is_vertice(current_vertice)) {
                        continue;
                    }
                    match Rectangle::new(current_vertice, &symbol_vector) {
                        Option::None => continue,
                        Option::Some(r) => rectangles.push(r)
                    }
                },
                _ => continue,
            }
        }
    }
    rectangles.len() as u32
}

enum Symbol {
    Vertice,
    HorizontalEdge,
    VerticalEdge,
}

fn to_symbol_vector(lines: &[&str]) -> Vec<Vec<Option<Symbol>>> {
    let mut root_symbol_vector: Vec<Vec<Option<Symbol>>> = Vec::new();
    for line in lines {
        let mut line_symbol_vector: Vec<Option<Symbol>> = Vec::new();
        for c in line.chars() {
            match c {
                '+' => line_symbol_vector.push(Option::Some(Symbol::Vertice)),
                '|' => line_symbol_vector.push(Option::Some(Symbol::VerticalEdge)),
                '-' => line_symbol_vector.push(Option::Some(Symbol::HorizontalEdge)),
                ' ' => line_symbol_vector.push(Option::None),
                _ => panic!("Invalid char detected: {}", c),
            }
        }
        root_symbol_vector.push(line_symbol_vector)
    }
    root_symbol_vector
}

fn find_horizontal_edge(start: Vertice, line: &Vec<Option<Symbol>>) -> Option<Edge> {
    if start.x >= line.len() - 1 {
        return Option::None;
    } else if line[start.x + 1].is_none() {
        return Option::None;
    }
    for index in start.x..line.len()-1 {
        match line[index] {
            Option::None => panic!("Horizontal edge search: space found"),
            Option::Some(Symbol::VerticalEdge) => panic!("Horizontal edge search: vertical edge found"),
            Option::Some(Symbol::HorizontalEdge) => continue,
            Option::Some(Symbol::Vertice) => {
                let edge = Edge::new(start, Vertice { x: index, y: start.y });
                return Option::Some(edge);
            }
        }
    }
    Option::None
}

fn find_vertical_edge(start: Vertice, grid: &Vec<Vec<Option<Symbol>>>) -> Option<Edge> {
    if start.y >= grid.len() - 1 {
        return Option::None;
    } else if grid[start.y + 1][start.x].is_none() {
        return Option::None;
    }
    for index in start.y..grid.len()-1 {
        match grid[index][start.x] {
            Option::None => panic!("Vertical edge search: space found"),
            Option::Some(Symbol::VerticalEdge) => continue,
            Option::Some(Symbol::HorizontalEdge) => panic!("Vertical edge search: horizontal edge found"),
            Option::Some(Symbol::Vertice) => {
                let edge = Edge::new(start, Vertice { x: start.x, y: index });
                return Option::Some(edge);
            }
        }
    }
    Option::None
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Vertice {
    x: usize,
    y: usize,
}

impl Vertice {
    fn new(x: usize, y: usize) -> Self {
        Vertice { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    first: Vertice,
    last: Vertice
}

impl Edge {
    fn new(first: Vertice, last: Vertice) -> Self {
        Edge{first, last}
    }
}

struct Rectangle {
    upper_left: Vertice,
    upper_right: Vertice,
    lower_left: Vertice,
    lower_right: Vertice,
}

impl Rectangle {
    fn new(upper_left: Vertice, grid: &Vec<Vec<Option<Symbol>>>) -> Option<Self> {
        let upper_edge;
        let left_edge;
        let lower_edge;
        let right_edge;
        match find_horizontal_edge(upper_left, &grid[upper_left.y]) {
            None => return Option::None,
            Some(edge) => upper_edge = edge
        }
        match find_vertical_edge(upper_left, grid) {
            None => return Option::None,
            Some(edge) => left_edge = edge,
        }
        match find_horizontal_edge(left_edge.last, &grid[left_edge.last.y]) {
            None => return Option::None,
            Some(edge) => lower_edge = edge
        }
        match find_vertical_edge(upper_edge.last, grid) {
            None => return Option::None,
            Some(edge) => right_edge = edge
        }

        if lower_edge.last != right_edge.last {
            panic!("Malformed rectangle at: {:?}", upper_left)
        }
        Option::Some(Rectangle { upper_left, upper_right: upper_edge.last, lower_left: left_edge.last, lower_right: lower_edge.last })
    }

    fn is_vertice(&self, vertice: Vertice) -> bool {
        self.upper_left == vertice || self.upper_right == vertice || self.lower_left == vertice || self.lower_right == vertice
    }
}