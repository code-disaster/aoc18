use std::str::SplitWhitespace;

pub fn main() {
    let input = include_str!("input.txt").split_whitespace();
    let license = License::new(input);

    let (_, meta_sum) = license.collect_meta_sum(0);
    println!("sum of meta data: {}", meta_sum);

    let (_, root_value) = license.collect_value(0);
    println!("value of root node: {}", root_value);
}

struct License {
    data: Vec<usize>,
}

impl License {
    fn new(input: SplitWhitespace) -> Self {
        let mut data = Vec::new();

        for (_, number) in input.enumerate() {
            data.push(number.parse::<usize>().unwrap());
        }

        License {
            data,
        }
    }

    fn collect_meta_sum(&self, node: usize) -> (usize, usize) {
        let num_children = self.data[node];
        let num_meta = self.data[node + 1];

        let mut node_tail = node + 2;
        let mut node_sum = 0;

        for _ in 0..num_children {
            let (t, s) = self.collect_meta_sum(node_tail);
            node_tail = t;
            node_sum += s;
        }

        node_sum += self.get_meta_sum(node_tail, num_meta);
        node_tail += num_meta;

        (node_tail, node_sum)
    }

    fn collect_value(&self, node: usize) -> (usize, usize) {
        let num_children = self.data[node];
        let num_meta = self.data[node + 1];

        let mut node_tail = node + 2;
        let node_value;

        if num_children == 0 {
            node_value = self.get_meta_sum(node_tail, num_meta);
        } else {
            let mut child_values = Vec::new();
            for _ in 0..num_children {
                let (t, v) = self.collect_value(node_tail);
                node_tail = t;
                child_values.push(v);
            }
            node_value = self.get_child_value_sum(node_tail, num_meta, &child_values);
        }

        node_tail += num_meta;

        (node_tail, node_value)
    }

    fn get_meta_sum(&self, pos: usize, len: usize) -> usize {
        let mut sum = 0;
        for p in pos..pos + len {
            sum += self.data[p];
        }
        sum
    }

    fn get_child_value_sum(&self, pos: usize, len: usize, values: &Vec<usize>) -> usize {
        let mut sum = 0;
        for p in pos..pos + len {
            let meta = self.data[p];
            if meta > 0 && meta <= values.len() {
                sum += values[meta - 1];
            }
        }
        sum
    }
}
