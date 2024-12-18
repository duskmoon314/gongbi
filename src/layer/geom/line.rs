#[macro_export]
macro_rules! geom_line {
    ($aes:expr $(, $($param: tt)*)?) => {
        $crate::geom!(Line, mapping = $aes $(, $($param)*)?)
    };

    ($($param: tt)*) => {
        $crate::geom!(Line, $($param)*)
    };
}
