//!There is a parking lot with only one empty spot. Given the initial state
//!of the parking lot and the final state. Each step we are only allowed to
//!move a car
//!out of its place and move it into the empty spot.
//!The goal is to find out the least movement needed to rearrange
//!the parking lot from the initial state to the final state.
//!
//!Say the initial state is an array:
//!
//![1, 2, 3, 0, 4],
//!where 1, 2, 3, 4 are different cars, and 0 is the empty spot.
//!
//!And the final state is
//!
//![0, 3, 2, 1, 4].
//!We can swap 1 with 0 in the initial array to get [0, 2, 3, 1, 4] and so on.
//!Each step swap with 0 only.
//!
//!Edit:
//!Now also prints the sequence of changes in states.
//!Output of this example :-
//!
//!initial: [1, 2, 3, 0, 4]
//!final:   [0, 3, 2, 1, 4]
//!Steps =  4
//!Sequence :
//!0 2 3 1 4
//!2 0 3 1 4
//!2 3 0 1 4
//!0 3 2 1 4

pub fn garage(initial: &[usize], r#final: &[usize]) -> (usize, Vec<Vec<usize>>) {
    let mut initial = initial.to_vec();
    let mut seq = Vec::with_capacity(initial.len()); // list of each step in sequence
    let mut steps = 0;

    while initial != r#final {
        let zero = initial.iter().position(|&n| n == 0)
            .expect("There are no fucking 0 in initial"); // if zero isn't where it should be
        if zero != r#final.iter().position(|&n| n == 0).unwrap() {
            let car_to_move = r#final[zero];
            let pos = initial.iter().position(|n| *n == car_to_move).unwrap();
            initial.swap(zero, pos);
        } else {
            for i in 0..initial.len() {
                if initial[i] != r#final[i] {
                    initial.swap(i, zero);
                    break;
                }
            }
        }
        seq.push(initial.clone());
        steps += 1;
    }
    (steps, seq)
}

//thus:
//1 2 3 0 4 -- zero = 3, true, car_to_move = final[3] = 1,
//             pos = initial.index(1) = 0, switched [0], [3]
//0 2 3 1 4 -- zero = 0, f, initial[1] != final[1], switched 0,1
//2 0 3 1 4 -- zero = 1, t, car_to_move = final[1] = 3,
//             pos = initial.index(3) = 2, switched [1], [2]
//2 3 0 1 4 -- zero = 2, t, car_to_move = final[2] = 2,
//             pos = initial.index(2) = 0, switched [0], [2]
//0 3 2 1 4 -- initial == final

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_garage() {
        let initial = [1, 2, 3, 0, 4];
        let r#final = [0, 3, 2, 1, 4];
        let seq = vec![
            vec![0, 2, 3, 1, 4],
            vec![2, 0, 3, 1, 4],
            vec![2, 3, 0, 1, 4],
            vec![0, 3, 2, 1, 4]
        ];
        assert_eq!((4, seq), garage(&initial, &r#final));
    }
}