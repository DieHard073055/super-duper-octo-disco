use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut output_image = image.clone();
        let mut explore = vec![(sr, sc)];
        let mut visited_map = HashMap::new();

        let image_y_len = image.len() - 1;
        let image_x_len = image[0].len() - 1;

        if sr > image_y_len as i32 || sc > image_x_len as i32 {
            return image;
        }
        let source_color = image[sr as usize][sc as usize];

        loop {
            if explore.is_empty() {
                break;
            }
            // println!("ex {:?}", explore);
            // println!("vi {:?}", visited);
            if let Some(current_node) = explore.pop() {
                // println!("current node {:?}", current_node);
                visited_map.insert(current_node.clone(), 0);
                output_image[current_node.0 as usize][current_node.1 as usize] = color;
                if current_node.0 > 0 {
                    // check above
                    if image[(current_node.0 - 1) as usize][current_node.1 as usize] == source_color
                        && !visited_map.contains_key(&(current_node.0 - 1, current_node.1))
                    {
                        explore.push((current_node.0 - 1, current_node.1));
                    }
                }
                if current_node.0 < (image_y_len) as i32 {
                    // check below
                    if image[(current_node.0 + 1) as usize][current_node.1 as usize] == source_color
                        && !visited_map.contains_key(&(current_node.0 + 1, current_node.1))
                    {
                        explore.push((current_node.0 + 1, current_node.1));
                    }
                }
                if current_node.1 > 0 {
                    // check left
                    if image[current_node.0 as usize][(current_node.1 - 1) as usize] == source_color
                        && !visited_map.contains_key(&(current_node.0, current_node.1 - 1))
                    {
                        explore.push((current_node.0, current_node.1 - 1));
                    }
                }
                if current_node.1 < (image_x_len) as i32 {
                    // check right
                    if image[current_node.0 as usize][(current_node.1 + 1) as usize] == source_color
                        && !visited_map.contains_key(&(current_node.0, current_node.1 + 1))
                    {
                        explore.push((current_node.0, current_node.1 + 1));
                    }
                }
            }
        }
        output_image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn test_flood_fill() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let sr = 1;
        let sc = 1;
        let color = 2;
        let expected_output = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        assert_eq!(
            Solution::flood_fill(image.clone(), sr, sc, color),
            expected_output
        );
    }

    #[test]
    // #[ignore]
    fn test_flood_fill_already_flooded() {
        let image = vec![vec![1, 1, 1], vec![1, 2, 2], vec![1, 2, 2]];
        let sr = 1;
        let sc = 1;
        let color = 2;
        let expected_output = vec![vec![1, 1, 1], vec![1, 2, 2], vec![1, 2, 2]];
        assert_eq!(Solution::flood_fill(image, sr, sc, color), expected_output);
    }

    #[test]
    // #[ignore]
    fn test_flood_fill_single_pixel() {
        let image = vec![vec![1]];
        let sr = 0;
        let sc = 0;
        let color = 2;
        let expected_output = vec![vec![2]];
        assert_eq!(Solution::flood_fill(image, sr, sc, color), expected_output);
    }

    #[test]
    // #[ignore]
    fn test_flood_fill_large_image() {
        let image = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];
        let sr = 0;
        let sc = 0;
        let color = 2;
        let expected_output = vec![
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        ];
        assert_eq!(Solution::flood_fill(image, sr, sc, color), expected_output);
    }

    #[test]
    fn test_flood_fill_invalid_sr_sc() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let sr = 3;
        let sc = 1;
        let color = 2;
        let expected_output = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        assert_eq!(Solution::flood_fill(image, sr, sc, color), expected_output);
    }
}
