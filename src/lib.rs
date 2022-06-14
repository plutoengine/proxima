pub enum Constness {
    None,
    Single,
    Double,
    Triple
}

pub enum Keyword {
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
