// staticな配列をコンパイル時に演算させるためのマクロ
// 等差数列のマクロ
macro_rules! arith_seq {
    ($start:expr, $step:expr; $count:expr) => {{
        const ARR: [f32; $count] = {
            let mut arr = [0.0; $count];
            let mut i = 0;
            while i < $count {
                arr[i] = $start + i as f32 * $step;
                i += 1;
            }
            arr
        };
        ARR
    }};
}
// A*(1+3850e-6*TEMP)の形の配列を生成するためのマクロ
macro_rules! r_pt_array {
    ($temp:expr, $A:expr) => {{
        const LEN: usize = $temp.len();
        const OUT: [f32; LEN] = {
            let mut r_pt_arr = [0.0; LEN];
            let mut i = 0;
            while i < LEN {
                r_pt_arr[i] = $A * (1.0 + 3850e-6 * $temp[i]);
                i += 1;
            }
            r_pt_arr
        };
        OUT
    }};
}