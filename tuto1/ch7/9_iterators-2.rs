fn main () {
    let a = vec![0,1,2, 3,4,5,6,7];
    let filtered_values = a.iter().filter(|&x| *x > 5).collect::<Vec<&u32>>();
    println!("{:?}", filtered_values);


    let b = a.clone();

    let filtered_values = a.into_iter().filter(|&x| x > 5).collect::<Vec<u32>>();
    println!("{:?}", filtered_values);

    let mut mapped_values = b.iter().map(|&x| 2 * x).collect::<Vec<u32>>(); // SAME OUTPUT
    //let mut mapped_values = b.iter().map(|x| 2 * x).collect::<Vec<u32>>(); // SAME OUTPUT
    //let mut mapped_values = b.iter().map(|x| 2 * *x).collect::<Vec<u32>>(); // SAME OUTPUT

    println!("{:?}", mapped_values);

    let mut mapped_filtered_values = b.iter().map(|&x| 2 * x).filter(|x| *x > 10).collect::<Vec<u32>>();
    println!("{:?}", mapped_filtered_values);
}