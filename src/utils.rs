pub mod functions {

    use std::any::type_name;
    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }
}
