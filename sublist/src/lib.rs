#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    } else if is_sublist_of(_first_list, _second_list) {
        return Comparison::Sublist;
    } else if is_sublist_of(_second_list, _first_list) {
        return Comparison::Superlist;
    }
    return Comparison::Unequal;
}

pub fn is_sublist_of<T: PartialEq>(_searched_list: &[T], _super_list: &[T]) -> bool {
    if _super_list.len() == 0 {
        return _searched_list.len() == 0
    } else if _searched_list.len() == 0 {
        return true;
    } else if _searched_list.len() > _super_list.len() {
        return false;
    } else if _searched_list.len() == _super_list.len() {
        return _searched_list == _super_list
    } else {
        let found_indexes = _super_list
            .iter()
            .enumerate()
            .filter(|(_,s)| **s == _searched_list[0])
            .map(|(index, _)| index)
            .collect::<Vec<_>>();

        for index in found_indexes.iter() {
            let searched_list_final_index = _searched_list.len() - 1; 
            let end_index = index + searched_list_final_index;
            if end_index > _super_list.len() {
                continue;
            }
            let s = &_super_list[*index..end_index];
            if _searched_list == s {
                return true
            }
        }
        return false;
    }
}
