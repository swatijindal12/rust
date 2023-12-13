use std::cmp::Ordering;

fn main() {
    // let us consider A, K, Q, J, as 14, 13, 12, 11 respectively.
    let mut poker_hand1 = [10, 11, 6, 2, 3];
    let mut poker_hand2 = [14, 4, 5, 10, 2];

    poker_hand1.sort_by(|a, b| b.partial_cmp(a).unwrap());
    poker_hand2.sort_by(|a, b| b.partial_cmp(a).unwrap());

    println!("Sorted value of poker_hand1: {:?}", poker_hand1);
    println!("Sorted value of poker_hand2: {:?}", poker_hand2);
    
    let mut result = Ordering::Equal;

    for (val1, val2) in poker_hand1.iter().zip(poker_hand2.iter()) {
        let cmp_result = val1.partial_cmp(val2).unwrap_or(Ordering::Equal);

        if cmp_result != Ordering::Equal {
            result = cmp_result;
            break;
        }
    }

    if result == Ordering::Greater {
        println!("poker_hand1 is the winner!");
    } else if result == Ordering::Less {
        println!("poker_hand2 is the winner!");
    } else {
        println!("It's a tie!");
    }

}
