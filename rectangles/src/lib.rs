use core::panic;

pub fn count(lines: &[&str]) -> u32 {
    let mut rectangles: Vec<Rectangle> = Vec::new();

    let symbol_vector = to_symbol_vector(lines);
    for (x, line) in symbol_vector.iter().enumerate() {
        for (y, symbol) in line.iter().enumerate() {
            match symbol {
                Some(Symbol::Vertice) => {
                    let current_vertice = Vertice::new(x, y);
                    for rect in Rectangle::find_all(current_vertice, &symbol_vector) {
                        rectangles.push(rect)
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

    fn all_horizontal_from(start: Vertice, grid: &Vec<Vec<Option<Symbol>>>) -> Vec<Self> {
        let line = &grid[start.x];
        if start.y >= line.len() - 1 {
            return Vec::new();
        }
        let mut edges: Vec<Edge> = Vec::new();
        for index in start.y+1..line.len() {
            let tmp = &line[index]; 
            match tmp {
                Option::None => break,
                Option::Some(Symbol::VerticalEdge) => break,
                Option::Some(Symbol::HorizontalEdge) => continue,
                Option::Some(Symbol::Vertice) => {
                    let edge = Edge::new(start, Vertice { x: start.x, y: index });
                    edges.push(edge);
                }
            }
        }
        edges
    }

    fn all_vertical_from(start: Vertice, grid: &Vec<Vec<Option<Symbol>>>) -> Vec<Edge> {
        if start.x >= grid.len() - 1 {
            return Vec::new();
        }
        let mut edges: Vec<Edge> = Vec::new();
        for index in start.x+1..grid.len() {
            match grid[index][start.y] {
                Option::None => break,
                Option::Some(Symbol::VerticalEdge) => continue,
                Option::Some(Symbol::HorizontalEdge) => break,
                Option::Some(Symbol::Vertice) => {
                    let edge = Edge::new(start, Vertice { x: index, y: start.y });
                    edges.push(edge);
                }
            }
        }
        edges   
    }

    fn validate_horizontal(&self, grid: &Vec<Vec<Option<Symbol>>>) -> bool {
        let line = &grid[self.first.x];
        for index in self.first.y..self.last.y+1 {
            let current = &line[index];
            match current {
                Option::None | Option::Some(Symbol::VerticalEdge) => return false,
                _ => continue,
            }
        }
        true
    }

    fn validate_vertical(&self, grid: &Vec<Vec<Option<Symbol>>>) -> bool {
        for index in self.first.x..self.last.x+1 {
            let current = &grid[index][self.first.y];
            match current {
                Option::None | Option::Some(Symbol::HorizontalEdge) => return false,
                _ => continue,
            }
        }
        true
    }
}

struct Rectangle {
    upper_left: Vertice,
    upper_right: Vertice,
    lower_left: Vertice,
    lower_right: Vertice,
}

impl Rectangle {
    fn find_all(upper_left: Vertice, grid: &Vec<Vec<Option<Symbol>>>) -> Vec<Self> {
        let mut rectangles: Vec<Self> = Vec::new();
        for upper_edge in Edge::all_horizontal_from(upper_left, grid) {
            let found_rectangles = Rectangle::from_upper_edge(upper_edge, grid);
            for rect in found_rectangles {
                rectangles.push(rect);
            }
        }
        rectangles
    }

    fn from_upper_edge(upper_edge: Edge, grid: &Vec<Vec<Option<Symbol>>>) -> Vec<Self> {
        let mut rectangles:Vec<Self> = Vec::new();
        for left_edge in Edge::all_vertical_from(upper_edge.first, grid) {
            if let Some(rect) = Rectangle::from_upper_and_left_edge(upper_edge, left_edge, grid) {
                rectangles.push(rect)
            }
        }
        rectangles
    }

    fn from_upper_and_left_edge(upper_edge: Edge, left_edge: Edge, grid: &Vec<Vec<Option<Symbol>>>) -> Option<Self> {
        let lower_right = Vertice::new(left_edge.last.x, upper_edge.last.y);
        let lower_edge = Edge::new(left_edge.last, lower_right);
        let right_edge = Edge::new(upper_edge.last, lower_right);
        if !lower_edge.validate_horizontal(grid) || !right_edge.validate_vertical(grid) {
            return Option::None;
        }
        return Option::Some(Rectangle{upper_left: upper_edge.first, upper_right: upper_edge.last, lower_left: left_edge.last, lower_right: lower_right});
    }
}