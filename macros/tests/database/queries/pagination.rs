pub trait Paginator {
    type E: std::fmt::Debug + Clone + PartialEq + Eq + Copy;
}
