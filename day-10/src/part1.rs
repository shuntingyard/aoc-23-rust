use std::fmt::Debug;

#[allow(unused_imports)]
use petgraph::dot::{Config, Dot};
use petgraph::{
    graphmap::GraphMap,
    Directed,
    Direction::{Incoming, Outgoing},
};

use crate::custom_error::AocError;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "P({},{})", self.x, self.y)
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // Create a graph, keys must be unique.
    let mut graph = GraphMap::<_, (), Directed>::new();

    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, pipe)| pipe != &'.') // We don't care about Ground.
                .flat_map(move |(col, pipe)| {
                    let x = col as i32;
                    let y = row as i32;
                    let me = Point::new(x, y);

                    // Generate directed edges
                    //
                    match pipe {
                        // For normal Pipes outdegree 2
                        '|' => vec![(me, Point::new(x, y - 1)), (me, Point::new(x, y + 1))],
                        '-' => vec![(me, Point::new(x - 1, y)), (me, Point::new(x + 1, y))],
                        'L' => vec![(me, Point::new(x, y - 1)), (me, Point::new(x + 1, y))],
                        'J' => vec![(me, Point::new(x, y - 1)), (me, Point::new(x - 1, y))],
                        '7' => vec![(me, Point::new(x - 1, y)), (me, Point::new(x, y + 1))],
                        'F' => vec![(me, Point::new(x + 1, y)), (me, Point::new(x, y + 1))],
                        // For Start generate outdegree 8 !
                        _ => vec![
                            (me, Point::new(x, y - 1)),
                            (me, Point::new(x + 1, y - y)),
                            (me, Point::new(x + 1, y)),
                            (me, Point::new(x + 1, y + 1)),
                            (me, Point::new(x, y + 1)),
                            (me, Point::new(x - 1, y + 1)),
                            (me, Point::new(x - 1, y)),
                            (me, Point::new(x - 1, y - 1)),
                        ],
                    }
                })
        })
        /*
           `Graph::from_edges(&[...]);` doesn't work on our custom node type (Point) !
        */
        .for_each(|edge| {
            if !graph.contains_node(edge.0) {
                graph.add_node(edge.0);
            }
            if !graph.contains_node(edge.1) {
                graph.add_node(edge.1);
            }
            graph.add_edge(edge.0, edge.1, ());
        });

    // Debug with grapviz 😍
    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    // Graph iter example
    /*
       graph
           .nodes()
           .into_iter()
           .flat_map(|node| {
               graph.neighbors(dbg!(node)).map(|neigh| {
                   println!("Neigh... {neigh:?}");
               })
           })
           .for_each(drop);
    */

    // Find start
    //
    // There might be 8 incoming for points other than Start:
    //  L|J
    //  - -
    //  F|7
    //
    // But no way there are 8 outgoing except for Start.
    let mut octopi: Vec<Point> = vec![];
    for node in graph.nodes() {
        if graph.edges_directed(node, Outgoing).count() == 8 {
            octopi.push(node);
        }
    }
    assert!(octopi.len() > 0); // Did Advent of Code hide more loops? :)
    let start = octopi[0];
    dbg!(&start);
    drop(octopi);

    // Traversal experiments:
    //
    // Get a starting point in the big pipe.
    /*
       Calls work like:
           for (incoming, _, _) in graph.edges_directed(node, Incoming) {
               do something with incoming ...;
           }
           for (_, outgoing, _) in graph.edges_directed(curr, Outgoing) {
               do something with outgoing ...;
           }
    */
    let mut bipeds: Vec<Point> = vec![];
    for (departure, _, _) in graph.edges_directed(start, Incoming) {
        // Hopefully just 2
        bipeds.push(departure);
    }
    assert!(bipeds.len() == 2);

    // traverse... We need to roll our own traversal,
    // as petgraph requires implementing NodeIndex/IndexType
    // on our Point for this.

    // What we have:
    let dep = bipeds[0]; // Point of departure
    drop(bipeds);
    let goal = start;

    // What we need:
    let predecessor = start; // As all goes both ways, this hints to the
                             // direction *not* to take.

    // Recursion doesn't hold on real graphs; so loop...
    //
    //  (Just checked if this would work with a tail call, it doesn't either!)
    fn traverse_and_count(
        mut dep: Point,
        goal: Point,
        mut predecessor: Point,
        graph: GraphMap<Point, (), Directed>,
        mut count: u16,
    ) -> u16 {
        loop {
            let next = graph
                .edges_directed(dep, Outgoing)
                .into_iter()
                .filter(|(_, my_out, _)| *my_out != predecessor)
                .collect::<Vec<(Point, Point, &())>>();
            assert!(next.len() == 1);
            let (_, next, _) = next[0];

            if goal == next {
                break;
            } else {
                predecessor = dep;
                dep = next;
                count += 1
            }
        }
        count
    }

    let count = traverse_and_count(dep, goal, predecessor, graph, 2);
    Ok((count / 2).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_uncluttered() -> miette::Result<()> {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!("4", process(input)?);
        Ok(())
    }

    #[test]
    fn test_square_cluttered() -> miette::Result<()> {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!("4", process(input)?);
        Ok(())
    }
    #[test]
    fn test_complex_uncluttered() -> miette::Result<()> {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!("8", process(input)?);
        Ok(())
    }

    #[test]
    fn test_complex_cluttered() -> miette::Result<()> {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
