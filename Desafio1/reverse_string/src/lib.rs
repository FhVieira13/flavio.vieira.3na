reverse_string/
├── Cargo.toml
└── src/
    └── lib.rs

pub unsafe fn multiply_array(ptr: *const i32, len: usize) -> i32 {
    let mut product = 1;
    for i in 0..len {
        product *= *ptr.offset(i as isize);
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_array() {
        let arr = [2, 3, 4];
        let product = unsafe { multiply_array(arr.as_ptr(), arr.len()) };
        assert_eq!(product, 24);
    }
} 
//O loop for i in 1..len começa em i = 1. 
// Isso significa que o primeiro elemento do array (índice 0) é ignorado no cálculo do produto.
//  Este é um erro lógico que fará com que a função retorne um resultado incorreto. 