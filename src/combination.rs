#[derive(Debug)]
pub struct Entry<T: Copy> {
    pub value: T,
    pub value_idx: usize,
}

pub fn memoized_combination_tree<T: Copy>(
    vals: &Vec<T>,
    num_options: usize,
    option_closures: Vec<Box<dyn Fn(T, T) -> T>>,
) -> Vec<Entry<T>> {
    assert_eq!(num_options, option_closures.len());

    let mut len: usize = 0;
    for i in 0..vals.len() {
        len += num_options.pow(i as u32);
    }

    let mut tree = Vec::<Entry<T>>::with_capacity(len);

    // Initialize tree with root value
    tree.push(Entry {
        value: vals[0],
        value_idx: 0,
    });

    for i in 1..len {
        let parent = (i - 1) / num_options;
        let value_idx = tree[parent].value_idx + 1;
        let value: T = option_closures[i % num_options](tree[parent].value, vals[value_idx]);

        tree.push(Entry { value, value_idx });
    }

    tree
}
