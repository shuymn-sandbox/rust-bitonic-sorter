pub mod first;
pub mod second;
pub mod third;

// SortOrderを列挙型として定義する
pub enum SortOrder {
    // SortOrderには2つのバリアントがある
    Ascending,  // 昇順
    Descending, // 降順
}
