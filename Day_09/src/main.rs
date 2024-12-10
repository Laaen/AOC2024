use std::fs;
use std::time;

fn defragmentate(data : &mut Vec<u32>){
    let mut rev_idx = data.len() - 1;

    for i in 0..data.len(){
        if data[i] == u32::MAX{
            for j in (0..rev_idx + 1).rev(){
                if data[j] != u32::MAX{
                    if i > j{
                        return;
                    }
                    data[i] = data[j];
                    data[j] = u32::MAX;
                    rev_idx = j;
                    break;
                }
            }
        }
    }
}

fn part_2(data : &mut Vec<u32>){
    // Search group
    let mut i = data.len() - 1;
    while i > 0{
        // Get a file "group"
        if data[i] != u32::MAX{
            let start_idx = i;
            let value= data[i];
            let mut size = 1;
            // Add file chunks
            while i > 0 && data[i] != u32::MAX && data[i] == data[i - 1]{
                size += 1;
                i -= 1
            }
            // The "fun" part
            // Get the "free" blocks
            let mut j = 0;
            while j < i{
                if data[j] == u32::MAX{
                    let start_idx_free = j;
                    let mut size_free = 1;
                    // Add free chunkx
                    while j < i && data[j] == u32::MAX && data[j] == data[j + 1]{
                        size_free += 1;
                        j += 1
                    }
                    // If enough free space, we replace
                    if size_free >= size{
                        let data_to_replace = &data[(start_idx-(size - 1))..start_idx+1].to_vec().clone();
                        data[start_idx_free..(start_idx_free + data_to_replace.len())].copy_from_slice(&data_to_replace);

                        data[(start_idx-(size - 1))..start_idx+1].copy_from_slice(&[u32::MAX].repeat(size));

                        break;
                    }
                }
                j += 1;
            }

        }
        if i > 0{
            i -= 1;
        }
    }
}

fn main() {

    let start = time::Instant::now();

    let data: Vec<char> = fs::read_to_string("input")
    .unwrap()
    .chars()
    .collect();
    
    let mut data_repr: Vec<u32> = vec![];

    for (i, a) in data.chunks(2).enumerate(){
        if a.len() > 1{
            data_repr.append(&mut [i as u32].repeat(a[0].to_digit(10).unwrap() as usize));
            data_repr.append(&mut [u32::MAX].repeat(a[1].to_digit(10).unwrap() as usize));
        } else {
            data_repr.append(&mut [i as u32].repeat(a[0].to_digit(10).unwrap() as usize));
        }
    }

    let mut data_part_1 = data_repr.clone();
    defragmentate(&mut data_part_1);

    let result_1 : u64 = data_part_1.iter()
    .filter(|x| *x != &u32::MAX)
    .enumerate()
    .map(|(idx, val)| idx as u64 * *val as u64)
    .sum();

    println!("{:?}", result_1);

    let mut data_part_2 = data_repr.clone();
    part_2(&mut data_part_2);

    let result_2 : u64 = data_part_2.iter()
    .enumerate()
    .filter(|x| *x.1 != u32::MAX)
    .map(|(idx, val)| idx as u64 * *val as u64)
    .sum();

    println!("{:?}", result_2);

    println!("{:?}", time::Instant::now() - start);

}
