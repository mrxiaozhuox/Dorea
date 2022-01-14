//! 相关宏程序等待更新中 ...

/// use database_type to create a "DataValue"
///
/// - @Number -> 1
/// - @String -> "hello world".to_string()
/// - @Dict -> HashMap::new()
/// - @Boolean -> false
#[macro_export]
macro_rules! database_type {
    (
        @$key:ident : $value:expr
    ) => {
        DataValue::$key($value)
    };
}
