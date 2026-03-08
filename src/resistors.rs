// これは汎用モジュールなので、専用のクレートを生成した方が良さそう
pub mod resistor_function 
{
    // この系列の値は固定のため、コンパイル時定数constで記述し、後から値を変更させない。
    // float32の配列で、E24系列とE96系列の抵抗値を定義する。
    // constで定義する値は大文字にするルールらしい
    // 未使用の定数は警告が出るため、名前の前にアンダースコアをつけて、未使用があり得ると明示する。
    const _E24_RESISTORS: [f32; 24] = [1.0,1.1,1.2,1.3,1.5,1.6,1.8,2.0,2.2,2.4,2.7,3.0,3.3,3.6,
        3.9,4.3,4.7,5.1,5.6,6.2,6.8,7.5,8.2,9.1];

    const _E96_RESISTORS: [f32; 96] = [1.0,1.02,1.05,1.07,1.1,1.13,1.15,1.18,1.21,1.24,1.27,
        1.3,1.33,1.37,1.4,1.43,1.47,1.5,1.54,1.58,1.62,1.65,1.69,1.74,1.78,
        1.82,1.87,1.91,1.96,2.0,2.05,2.1,2.15,2.21,2.26,2.32,2.37,2.43,2.49,
        2.55,2.61,2.67,2.74,2.8,2.87,2.94,3.01,3.09,3.16,3.24,3.32,3.4,3.48,3.57,
        3.65,3.74,3.83,3.92,4.02,4.12,4.22,4.32,4.42,4.53,4.64,4.75,4.87,4.99,5.11,
        5.23,5.36,5.49,5.62,5.76,5.9,6.04,6.19,6.34,6.49,6.65,6.81,6.98,7.15,7.32,7.5,
        7.68,7.87,8.06,8.25,8.45,8.66,8.87,9.09,9.31,9.53,9.76];
        
    pub fn _e24(coefficient: i32) -> [f32; 24] {//関数名は小文字で始めるルールらしい
        let coefficient_f = coefficient as f32; 
        // 各要素をcoefficient_f倍する
        let e24_valurs: [f32; 24] = _E24_RESISTORS.iter().map(|&x| x * coefficient_f).collect::<Vec<f32>>().try_into().unwrap();
        return e24_valurs
    }
    pub fn _e96(coefficient: i32) -> [f32; 96] {//関数名は小文字で始めるルールらしい
        let coefficient_f = coefficient as f32; 
        // 各要素をcoefficient_f倍する
        let e96_valurs: [f32; 96] = _E96_RESISTORS.iter().map(|&x| x * coefficient_f).collect::<Vec<f32>>().try_into().unwrap();
        return e96_valurs
    }
    pub fn _test(){
        println!("Connected resistor_function module!");
        let e24_list: [f32; 24] = _e24(1);
        //iter - この関数は、各周回においてコレクションの要素を借用。
        // // よってコレクションには手を加えないので、ループの実行後もコレクションを再利用できる。
        for &value in e24_list.iter() {
            println!("E24 resistor value: {}", value);
        }
    }
}//end mod resistor_function
