#[macro_export]
macro_rules! bit_vec_builder {
    (
        #[$len:expr]
        $( [$range:expr] : $value:expr; )*
    ) => {
        {
            let mut bv = $crate::bit_vec::BitVec::new($len);
            $( bv.sets($range, &$value); )*
            bv
        }
    };
}
