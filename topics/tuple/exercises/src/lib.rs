pub fn first(t: (bool, u32, char)) -> bool {
    return t.0;
}

pub fn last(t: (bool, u32, char)) -> char {
    return t.2;
}

pub fn swap(t: (u32, u32)) -> (u32, u32) {
    return (t.1, t.0);
}
