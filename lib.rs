#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]

//you can change the type of vec[T] that this works on by finding and replacing every 'usize' with the type you want to use. it should work

fn make_subsets (v:Vec<usize>) -> Vec<Vec<usize>> { // works in the same time complexity as the rest of the sort.
    let mut subsets:Vec<(Vec<usize>, Vec<usize>)> = Vec::new(); //alllowing for negative indexes
    let mut op = v[0];
    let mut lastItem:Vec<(usize, usize)> = vec![(op, op)];
    let mut end = 0;
    let mut start = 0;
    let mut pivot = 0;
    let mut right = false;
    let mut left = false;
    
    subsets.push((vec![op], vec![op]));
    lastItem = vec![(op, op)];
    
    for i in 1..v.len() {
        op = v[i];
        start = 0;
        end = subsets.len() - 1;
        
        //println!("{} {:?} {:?}", pivot, lastItem, subsets);
        
        if op > lastItem[lastItem.len() - 1].0 {
            loop { // performs a binary search on subsets to find a place where the number being operated on is between the last item of the list before the pivot and the last item of the pivot.
                pivot = (start + end) / 2;
                //println!("{} {} {} {}", op, start, end, pivot);
                
                left = pivot > 0 && op >= lastItem[pivot - 1].0;
                right = op < lastItem[pivot].0;
                
                //println!("{} {}", left, right);
                
                if !(left || right) { //if neither left or right
                    lastItem[pivot].0 = op;
                    subsets[pivot].0.push(op);
                    break;
                } else if left {
                    end = pivot - 1;
                } else if right {
                    start = pivot + 1;
                }
            }
        } else {
            loop {
                pivot = (start + end) / 2;
                //println!("{} {} {} {}", op, start, end, pivot);
                
                left = pivot > 0 && op <= lastItem[pivot - 1].1;
                right = op > lastItem[pivot].1;
                
                //println!("{} {}", left, right);
                
                if start == end && right {
                    subsets.push((vec![op], vec![op]));
                    lastItem.push((op, op));
                    break;
                }
                
                if !(left || right) { //if neither left or right
                    lastItem[pivot].1 = op;
                    subsets[pivot].1.push(op);
                    break;
                } else if left {
                    end = pivot - 1;
                } else if right {
                    start = pivot + 1;
                }
            }
        }
    }
    let mut out:Vec<Vec<usize>> = Vec::new();
    
    for i in 0..subsets.len() {
        out.push(Vec::new());
        
        for o in (1..subsets[i].1.len()).rev() {
            out.last_mut().unwrap().push(subsets[i].1[o]);
        }
        for o in 0..subsets[i].0.len() {
            out.last_mut().unwrap().push(subsets[i].0[o]);
        }
    }
    println!("{}, {}", v.len(), subsets.len());
    //println!("{:?}", out);
    
    out
}

pub fn sort(v:Vec<usize>) -> Vec<usize> {
    let mut subsets = make_subsets(v);
    
    while subsets.len() > 1 {
        let mut subsets2:Vec<Vec<usize>> = Vec::new();
        
        for i in 0..subsets.len() / 2 {
            subsets2.push(merge(&subsets[i * 2], &subsets[i * 2 + 1]))
        }
        
        if subsets.len() % 2 == 1 {
            subsets2.push(subsets.pop().unwrap());
        }
        subsets = subsets2;
    }
    
    //println!("Done!");
    
    //println!("{:?}", subsets);
    let out = subsets[0].clone();
    out
}

fn merge(v1:&Vec<usize>, v2:&Vec<usize>) -> Vec<usize> {
    let mut poi1 = 0;
    let mut poi2 = 0;
    let mut out:Vec<usize> = Vec::new();
    
    while poi1 < v1.len() && poi2 < v2.len() {
        if v1[poi1] < v2[poi2] {
            out.push(v1[poi1]);
            poi1 += 1;
        } else {
            out.push(v2[poi2]);
            poi2 += 1;
        }
    }
    if poi1 == v1.len() {
        for i in poi2..v2.len() {
            out.push(v2[i]);
        }
    } else {
        for i in poi1..v1.len() {
            out.push(v1[i]);
        }
    }
    out
}

fn rand(x:u32) -> u32 {
	let y = x as u64 * x as u64 + x as u64;
	
	((y >> 16) ^ (y << 16)) as u32
}
