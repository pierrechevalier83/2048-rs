extern crate num;

#[cfg(test)]
mod slide_test {
    use super::slide_right;
    use super::slide_left;
    #[test]
    fn test_slide_right_with_one_element() {
        assert_eq!(vec![0, 0, 0, 1], slide_right(&[0, 1, 0, 0]));
    }
    #[test]
    fn test_slide_left_with_one_element() {
        assert_eq!(vec![1, 0, 0, 0], slide_left(&[0, 1, 0, 0]));
    }
    #[test]
    fn test_slide_right_with_two_different_elements() {
        assert_eq!(vec![0, 0, 1, 2], slide_right(&[1, 0, 2, 0]));
    }
    #[test]
    fn test_slide_left_with_two_different_elements() {
        assert_eq!(vec![1, 2, 0, 0], slide_left(&[1, 0, 2, 0]));
    }
    #[test]
    fn test_slide_right_with_two_same_elements() {
        assert_eq!(vec![0, 0, 0, 2], slide_right(&[1, 0, 1, 0]));
    }
    #[test]
    fn test_slide_left_with_two_same_elements() {
        assert_eq!(vec![2, 0, 0, 0], slide_left(&[1, 0, 1, 0]));
    }
    #[test]
    fn test_slide_right_with_three_same_elements() {
        assert_eq!(vec![0, 0, 1, 2], slide_right(&[1, 0, 1, 1]));
    }
    #[test]
    fn test_slide_left_with_three_same_elements() {
        assert_eq!(vec![2, 1, 0, 0], slide_left(&[1, 0, 1, 1]));
    }
    #[test]
    fn test_slide_right_with_three_different_elements() {
        assert_eq!(vec![0, 0, 2, 2], slide_right(&[1, 0, 1, 2]));
        assert_eq!(vec![0, 2, 1, 2], slide_right(&[2, 0, 1, 2]));
        assert_eq!(vec![0, 0, 2, 2], slide_right(&[0, 1, 1, 2]));
    }
    #[test]
    fn test_slide_left_with_three_different_elements() {
        assert_eq!(vec![2, 2, 0, 0], slide_left(&[1, 0, 1, 2]));
        assert_eq!(vec![2, 1, 2, 0], slide_left(&[2, 0, 1, 2]));
        assert_eq!(vec![2, 2, 0, 0], slide_left(&[0, 1, 1, 2]));
    }
    #[test]
    fn test_slide_right_with_four_same_elements() {
        assert_eq!(vec![0, 0, 2, 2], slide_right(&[1, 1, 1, 1]));
    }
    #[test]
    fn test_slide_left_with_four_same_elements() {
        assert_eq!(vec![2, 2, 0, 0], slide_left(&[1, 1, 1, 1]));
    }
    #[test]
    fn test_slide_right_with_four_different_elements() {
        assert_eq!(vec![1, 2, 1, 2], slide_right(&[1, 2, 1, 2]));
    }
    #[test]
    fn test_slide_left_with_four_different_elements() {
        assert_eq!(vec![1, 2, 1, 2], slide_left(&[1, 2, 1, 2]));
    }
}

fn merge_backward(slice: &mut [i32]) -> i32 {
    if slice[0] == slice[1] && slice[1] != 0 {
        slice[0] = 0;
        slice[1] += 1;
        num::pow::pow(2, slice[1] as usize)
    } else {
        0
    }
}

fn stable_partition<T, I, F>(slice: I, pred: F) -> Vec<T>
    where T: Copy,
          I: IntoIterator<Item = T>,
          for<'r> F: Fn(&'r T) -> bool
{
    let (mut left, right): (Vec<T>, Vec<T>) = slice.into_iter().partition(pred);
    left.extend(right.iter());
    left
}

pub fn slide_right(data: &[i32]) -> (Vec<i32>, i32) {
    let mut ret = stable_partition(data.iter().cloned(), |x| *x == 0);
    let mut index = data.len();
    let mut score = 0;
    while index > 1 {
        score += merge_backward(&mut ret[index - 2..index]);
        index -= 1;
    }
    (stable_partition(ret.iter().cloned(), |x| *x == 0), score)
}

pub fn slide_left(data: &[i32]) -> (Vec<i32>, i32) {
    let ret = data.clone()
        .iter()
        .rev()
        .cloned()
        .collect::<Vec<_>>();
    let (data, score) = slide_right(&ret);
    (data.iter()
         .cloned()
         .rev()
         .collect::<Vec<_>>(),
     score)
}

pub fn transpose(data: &mut [i32; 16]) {
    for i in 0..4 {
        for j in i..4 {
            data.swap(i + 4 * j, j + 4 * i);
        }
    }
}
