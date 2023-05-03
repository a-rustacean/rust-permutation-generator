#[allow(dead_code)]
fn generate_all_permutations<I, T>(arr: &I) -> Vec<Vec<T>>
where
    I: IntoIterator<Item = T> + Clone,
    T: Clone + Eq
{
    let arr = arr.clone();
    let vec: Vec<T> = arr.clone().into_iter().collect();
    if vec.is_empty() {
        return vec![];
    }
    if vec.len() == 1 {
        return vec![vec![vec[0].clone()]];
    }
    if vec.len() == 2 {
        return vec![
            vec![vec[0].clone(), vec[1].clone()],
            vec![vec[1].clone(), vec[0].clone()]
        ]
    }
    let mut permutations = vec![];
    for el in vec.into_iter() {
        let slice: Vec<T> = arr
            .clone()
            .into_iter()
            .filter(|slice_el| slice_el.clone() != el)
            .collect();
        let slice_permutations = generate_all_permutations(&slice);
        for mut slice_permutation in slice_permutations.into_iter() {
            let mut permutation = vec![el.clone()];
            permutation.append(&mut slice_permutation);
            permutations.push(permutation);
        }
    };
    permutations
}
