pub mod vector;
pub mod color;
pub mod ray;
#[cfg(test)]
mod tests {
    use vector::Vector3;

    use crate::vector;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn scalar_product(){
        let vector=vector::Vector3::new(3.0,8.0,4.0);
        let scaler=2.0;
        let new_vector=vector*scaler;
        assert_eq!(new_vector,Vector3::new(6.0,16.0,8.0))
    }

    #[test]
    fn dot_product_eq_length_squared(){
        let vector=vector::Vector3::new(12.0,32.0,4.333);
        assert_eq!(vector.dot(vector),vector.length_squared());
    }
}
