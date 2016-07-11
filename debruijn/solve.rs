// Generating de Bruijn sequences
// https://en.wikipedia.org/wiki/De_Bruijn_sequence
// Peixian Wang - July 7, 2016



//recursively extends the de bruijn sequence
fn debrujin_extension(mut init: &mut Vec<i32>, mut seq: &mut Vec<i32>, t: i32, p: i32, k: i32, n:i32) {
    if t > n {
        if n % p == 0 {
            let next_p: usize = (p+1) as usize;
            seq.extend(&init[1 as usize..next_p]);
        }
    }
    else {
        println!("REACHED 16");
        let diff: usize = (t-p) as usize;
        init[t as usize] = init[diff];
        debrujin_extension(&mut init, &mut seq, t + 1, p, k, n);
        for i in (init[diff] + 1)..k {
            init[t as usize] = i;
            debrujin_extension(&mut init, &mut seq, t+1, t, k, n);
            println!("REACHED 23");
        }
    }
}

//integer version of the debruijn sequence, assuming the alphabet is [0, k)
fn debruijn_int(k: i32, n:i32) {
    let mut init = vec![0, k*n];
    let mut seq: Vec<i32> = Vec::new();
    debrujin_extension(&mut init, &mut seq, 1, 1, k, n);
    println!("{:?}", seq)
}

fn main() {
    debruijn_int(2,3);
}
