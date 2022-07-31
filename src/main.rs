fn main() {
    

    let mut vec_1:Vec<u32> = vec![5,4,3,6,9];
    let mut vec_2:Vec<u32> = vec![5,8,6,4,10,15,20,21,9];

    println!("The INTERSECTION of the 2 vectors {:?} and {:?} is\n {:?}",vec_1, vec_2, intersect(&vec_1, &vec_2));
    println!("The UNION of the 2 vectors {:?} and {:?} is\n {:?}",vec_1, vec_2, union(&vec_1, &vec_2));

}

fn intersect<'a,'b>(vec1:&'a Vec<u32>, vec2:&'a Vec<u32>) -> Vec<&'a u32> {
    let inter_vec =
    vec1.iter().filter(|&&x| {
        vec2.contains(&x)
    }).collect::<Vec<&u32>>();
    inter_vec
}


fn union<'a,'b>(vec1:&'a Vec<u32>, vec2:&'a Vec<u32>) -> Vec<u32> {
    let mut union_vec: Vec<u32> = vec![];
    for i in vec1  {
        union_vec.push(*i);
    }
    for j in vec2 {
        if(!union_vec.contains(j)){
            union_vec.push(*j);
        }
    }
    union_vec
}



