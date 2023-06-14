struct Array<T, const N: usize> {
    data : [T; N]
}

pub fn main() {
    let arrays = [
        Array{
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3]
        }
    ];

    println!("Success!");
}
