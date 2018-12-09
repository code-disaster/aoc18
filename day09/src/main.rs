#[macro_use]
extern crate text_io;

pub fn main() {
    let input = include_str!("input.txt");

    let players: usize;
    let last_marble: usize;
    scan!(input.bytes() => "{} players; last marble is worth {} points", players, last_marble);

    let max_score = play(players, last_marble);
    println!("winning score: {}", max_score);

    let max_score_100 = play(players, last_marble * 100);
    println!("winning score x 100: {}", max_score_100);
}

struct Node {
    value: usize,
    prev: usize,
    next: usize,
}

fn play(players: usize, last_marble: usize) -> usize {
    let marbles = last_marble + 1;
    let mut scores = vec![0; players];
    let mut circle = Vec::with_capacity(marbles);

    for pos in 0..marbles {
        circle.push(Node {
            value: pos,
            prev: pos,
            next: pos
        });
    }

    let mut current_pos = 0;
    let mut current_player = 1;

    for marble in 1..marbles {
        if marble % 23 == 0 {
            scores[current_player] += marble;
            current_pos = reverse(&circle, current_pos, 7);
            scores[current_player] += current_pos;
            current_pos = remove(&mut circle, current_pos);
        } else {
            current_pos = forward(&circle, current_pos, 1);
            current_pos = insert(&mut circle, marble, current_pos);
        }
        current_player = (current_player + 1) % players;
    }

    *scores.iter().max().unwrap()
}

fn forward(circle: &Vec<Node>, pos: usize, steps: usize) -> usize {
    let mut node: Option<&Node> = circle.get(pos);
    for _ in 0..steps {
        let next = node.unwrap().next;
        node = circle.get(next);
    }
    node.unwrap().value
}

fn reverse(circle: &Vec<Node>, pos: usize, steps: usize) -> usize {
    let mut node: Option<&Node> = circle.get(pos);
    for _ in 0..steps {
        let prev = node.unwrap().prev;
        node = circle.get(prev);
    }
    node.unwrap().value
}

fn insert(circle: &mut Vec<Node>, pos: usize, at: usize) -> usize {
    let next = {
        circle.get_mut(at).unwrap().next
    };

    {
        let node: &mut Node = circle.get_mut(pos).unwrap();
        node.prev = at;
        node.next = next;
    }

    {
        let prev_node: &mut Node = circle.get_mut(at).unwrap();
        prev_node.next = pos;
    }

    {
        let next_node: &mut Node = circle.get_mut(next).unwrap();
        next_node.prev = pos;
    }

    pos
}

fn remove(circle: &mut Vec<Node>, at: usize) -> usize {
    let prev = {
        circle.get_mut(at).unwrap().prev
    };
    let next = {
        circle.get_mut(at).unwrap().next
    };

    {
        let node: &mut Node = circle.get_mut(at).unwrap();
        node.prev = at;
        node.next = at;
    }

    {
        let prev_node: &mut Node = circle.get_mut(prev).unwrap();
        prev_node.next = next;
    }

    {
        let next_node: &mut Node = circle.get_mut(next).unwrap();
        next_node.prev = prev;
    }

    next
}

/* solution using linear vector - way too slow for part 2 */
fn _linear_play_xtra_slow(players: usize, last_marble: usize) -> usize {
    let marbles = last_marble + 1;
    let mut scores = vec![0; players];
    let mut circle = Vec::with_capacity(marbles);

    circle.push(0);
    let mut current_pos= 0;
    let mut current_player = 1;

    for marble in 1..marbles {
        if marble % 23 == 0 {
            scores[current_player] += marble;
            current_pos = (current_pos + circle.len() - 7) % circle.len();
            let removed_marble = circle.remove(current_pos);
            scores[current_player] += removed_marble;
        } else {
            current_pos = (current_pos + 2) % circle.len();
            circle.insert(current_pos, marble);
        }
        current_player = (current_player + 1) % players;
    }

    *scores.iter().max().unwrap()
}
