#[macro_export]
macro_rules! geom_point {
    ($aes:expr $(, $($param: tt)*)?) => {
        $crate::geom!(Point, mapping = $aes $(, $($param)*)?)
    };

    ($($param: tt)*) => {
        $crate::geom!(Point, $($param)*)
    };
}
