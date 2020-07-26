use std::collections::HashMap;

struct Cacher<T, U, V>
where
    T: Fn(U) -> V,
{
    calculation: T,
    values: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: std::hash::Hash + Eq + Copy,
    V: Copy,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Self {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        c.value(1);
        let result = c.value(2);
        assert_eq!(result, 2);
    }

    #[test]
    fn call_with_different_types() {
        let mut c = Cacher::new(|a: char| a.is_ascii_alphanumeric());
        let result = c.value('a');
        assert_eq!(result, true);

        let mut c = Cacher::new(|a: &str| a.len());
        let result = c.value("hello");
        assert_eq!(result, 5);
    }
}
