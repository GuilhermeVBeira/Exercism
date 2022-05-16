#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let nome = "guilherme";
    if _first_list == _second_list {
        return Comparison::Equal;
    }
    if _first_list.len() == 0 && _second_list.len() > 0 {
        return Comparison::Sublist;
    }
    if _second_list.len() == 0 && _first_list.len() > 0 {
        return Comparison::Superlist;
    }

    let is_a_in_b = _first_list
        .windows(_second_list.len())
        .any(|x| x == _second_list);

    let is_b_in_a = _second_list
        .windows(_first_list.len())
        .any(|x| x == _first_list);

    if is_a_in_b {
        return Comparison::Superlist;
    }
    if is_b_in_a {
        return Comparison::Sublist;
    }
    Comparison::Unequal
}
