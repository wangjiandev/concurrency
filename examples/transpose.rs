fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];
    for (i, row) in matrix.iter().enumerate() {
        for (j, &item) in row.iter().enumerate() {
            transposed[j][i] = item;
        }
    }
    transposed
}

fn main() {
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let transposed = transpose(matrix);
    println!("{:#?}", transposed);
}
