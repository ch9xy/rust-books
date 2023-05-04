// O(n^2)
pub fn bubble_sort<T: PartialOrd> (v: &mut [T]) {
    for p in 0..v.len() { // v.len() -1 ?
        let mut sorted = true;
        for i in 0..(v.len() - 1) - p {
        if v[i] > v[i + 1] {
            v.swap(i, i+1);
            sorted = false;
        }
    }
    if sorted {
        return;
    }
    }
}

fn main () {
    let mut v = [2, 5, 3, 8 , 4];
    bubble_sort(&mut v);
    dbg!(&v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4,6,1,8,11,13,3];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1,3,4,6,8,11,13]);
    }
}
