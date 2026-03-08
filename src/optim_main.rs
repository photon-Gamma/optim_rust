

//兄弟のモジュールはcrate::のパスなどでアクセスできる
// モジュールの定義
//use ndarray::Array1;
//C_SI/VOL_GAIN
pub mod optim_function {
    const VREF_IC : f32 = crate::V!(3.3);//VrefICの電圧
    const C_SI : f32=crate::mV!(600)/crate::degree!(1);//Siの温度依存の物性値
    const VOL_GAIN : f32=45.0;//電源のゲイン
    const BASE_TEMPERATURE : f32 = crate::degree!(30);//基準温度30度にする
    const R_PT_30DO : f32 = crate::kOhm!(1)*(1.0+3850e-6*BASE_TEMPERATURE);//基準温度の白金抵抗値
    fn max_f32(data: &Vec<f32>) -> f32 {
    let max = data
        .iter()
        .copied()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    return max;
    }
    fn min_f32(data: &Vec<f32>) -> f32 {
    let min = data
        .iter()
        .copied()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    return min;
    }
    

    ///--------------------------------------///
    ///信号基板から印加される温度電圧
    ///--------------------------------------///
    fn vtemp_in(r_pt: f32 ) -> f32 {
        let r2:f32 = crate::kOhm!(20.0);
        let r1:f32 = crate::kOhm!(1.1);
        return VREF_IC*(r_pt/(r_pt+r2))*(1.0+r2/r1)
    }
    fn vtemp_in_degree() -> f32 {
        let r2:f32 = crate::kOhm!(20.0);
        let r1:f32 = crate::kOhm!(1.1);
        let r_pt:f32 = crate::kOhm!(1)*(1.0+3850e-6*BASE_TEMPERATURE);
        return VREF_IC*((crate::kOhm!(1)*3850e-6)/(r_pt+r2))*(1.0+r2/r1)
    }
    ///--------------------------------------///
    ///オフセット補正用の回路
    ///--------------------------------------///
    fn offset_func(v_offset_out: f32) -> (f32, f32, f32) {
        //r_offset1
        let mut vout_list = Vec::new();
        let mut r_offset1_list = Vec::new();
        let mut r_offset2_list = Vec::new();
        let mut r_offset2m_list = Vec::new();
        //iter()でイテレータを生成し、forループで各要素を借用して処理する
        for r_offset1_i in (crate::resistors::resistor_function::_e24(crate::kOhm!(10) as i32)).iter(){
            for r_offset2m_i in (crate::resistors::resistor_function::_e24(crate::Ohm!(100) as i32)).iter(){
                for r_offset2_i in (crate::resistors::resistor_function::_e24(crate::kOhm!(10) as i32)).iter(){
                    let vout_abs = (v_offset_out-(VREF_IC*((r_offset2_i+r_offset2m_i)/(r_offset1_i))) ).abs();
                    //println!("vout_abs: {}", vout_abs);
                    if vout_abs < crate::mV!(100.0) {
                        //println!("条件を満たした。");
                        vout_list.push(vout_abs);
                        r_offset1_list.push(*r_offset1_i);
                        r_offset2_list.push(*r_offset2_i);
                        r_offset2m_list.push(*r_offset2m_i);
                    }
                }

            }
        }
        let min_index = vout_list
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(i, _)| i);
        match min_index{
            Some(i) => println!("最小のvout_abs: {:?}mV, r_offset1: {:?}kOhm, r_offset2: {:?}kOhm, r_offset2m: {:?}Ohm", vout_list[i]*1e-3, r_offset1_list[i]*1e-3, r_offset2_list[i]*1e-3, r_offset2m_list[i]),
            None => println!("条件を満たす組み合わせが見つかりませんでした。"),
        }
        (r_offset1_list[min_index.unwrap_or(0)], r_offset2_list[min_index.unwrap_or(0)], r_offset2m_list[min_index.unwrap_or(0)])
    }
    ///--------------------------------------///
    ///温度補正用の回路
    ///--------------------------------------///
    fn vtemp_func(v_temp_in: Vec<f32>, r_offset1: f32, r_offset2: f32, r_offset2m: f32) -> (f32, f32, f32, f32) {
        
        let mut vout_list = Vec::new();
        let mut r_temp1m_list = Vec::new();
        let mut r_temp1_list = Vec::new();
        let mut r_temp2_list = Vec::new();
        let mut r_temp2m_list = Vec::new();
        let V_ofsset_gain =1.0+(r_offset2+r_offset2m)/r_offset1; //抵抗分圧
        //iter()でイテレータを生成し、forループで各要素を借用して処理する
        for r_temp1_i in (crate::resistors::resistor_function::_e24(crate::kOhm!(1) as i32)).iter(){
            for r_temp1m_i in (crate::resistors::resistor_function::_e24(crate::Ohm!(100) as i32)).iter(){
                for r_temp2_i in (crate::resistors::resistor_function::_e24(crate::kOhm!(10) as i32)).iter(){
                    for r_temp2m_i in (crate::resistors::resistor_function::_e24(crate::Ohm!(100) as i32)).iter(){
                        let V_temp_partial = (r_temp1_i+r_temp1m_i)/(r_temp1_i+r_temp1m_i+r_temp2_i+r_temp2m_i+300.0); //温度電圧側の抵抗分圧
                        let v_temp_o_arr: Vec<f32> = v_temp_in.iter().map(|x| x * V_temp_partial*V_ofsset_gain).collect();
                        
                        let vout: Vec<f32> = v_temp_o_arr.iter().map(|x| C_SI/VOL_GAIN - *x).collect();
                        let vout_abs = (max_f32(&vout) - min_f32(&vout)).abs();

                        //println!("vout_abs: {}", vout_abs);
                        if vout_abs < crate::mV!(100.0) {
                            //println!("条件を満たした。");
                            vout_list.push(vout_abs);
                            r_temp1_list.push(*r_temp1_i);
                            r_temp1m_list.push(*r_temp1m_i);
                            r_temp2_list.push(*r_temp2_i);
                            r_temp2m_list.push(*r_temp2m_i);
                        }
                    }
                }

            }
            
        }
        let min_index = vout_list
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(i, _)| i);
        match min_index{
            Some(i) => println!("最小のvout_abs: {:?}mV, r_temp1: {:?}kOhm, r_temp1m: {:?}kOhm, r_temp2: {:?}kOhm, r_temp2m: {:?}Ohm", vout_list[i]*1e3, r_temp1_list[i], r_temp1m_list[i]*1e-3, r_temp2_list[i], r_temp2m_list[i]*1e-3),
            None => println!("条件を満たす組み合わせが見つかりませんでした。"),
        }
        //.unwrap_or(0)で空だった時に備えたデフォルト値を設定
        (r_temp1_list[min_index.unwrap_or(0)], r_temp1m_list[min_index.unwrap_or(0)], r_temp2_list[min_index.unwrap_or(0)], r_temp2m_list[min_index.unwrap_or(0)])
    }

    pub fn run() {//pubは外部からアクセスできるようにするためのキーワード
        //---------------------------------------///
        //テスト用の範囲
        //---------------------------------------///
        {
            println!("Connected optim_function module!");
            let e24_list: [f32; 24] = crate::resistors::resistor_function::_e24(crate::kOhm!(1) as i32);
            //iter - この関数は、各周回においてコレクションの要素を借用。
            // よってコレクションには手を加えないので、ループの実行後もコレクションを再利用できる。
            println!("{:?}", e24_list);
            for &value in e24_list.iter() {
                println!("E24 resistor value: {}", value);
            }
            let voltage = crate::nV!(5); // 5_nV の代わりに nV!(5) を使用
            println!("module {}V", voltage);

            let vecs = vec![3.2, 1.5, 4.8, 0.9];
            let max_val = max_f32(&vecs);
            println!("max = {}", max_val);

        }

        //---------------------------------------///
        // 温度をコンパイル時に固定で設定する場合
        // 決定的な値なのでコンパイル時にデータメモリに配置させる
        //---------------------------------------///
        //const TEMP_START : f32 = 10.0;//開始温度
        //const N_TEMP : usize = 10;//生成する温度の数
        //const TEMP_END : f32 = 45.0;//終了温度
        //const TEMP: [f32; N_TEMP] = arith_seq!(TEMP_START, (TEMP_END - TEMP_START) / (N_TEMP as f32- 1.0); N_TEMP);
        

        //---------------------------------------///
        //標準値の生成
        //---------------------------------------///
        //const R_PT : [f32; N_TEMP] = r_pt_array!(TEMP, crate::kOhm!(1));//白金抵抗値
        //println!("{:?}", TEMP);
        //println!("{:?}", R_PT);

        //---------------------------------------///
        // 外部入力で実行時に温度範囲も決まることを想定(外部入力機能はまだない)
        //---------------------------------------///
        // ヒープ領域に確保
        // これがC++で解放忘れでメモリリークに繋がる領域
        // このスコープ内で有効
        let temp: Vec<f32> = {
            let temp_start : f32 = 10.0;//開始温度, スタックメモリ, imutable
            let n_temp : usize = 10;//生成する温度の数, スタックメモリ
            let temp_end : f32 = 45.0;//終了温度, スタックメモリ
            let temp0: Vec<f32> = (0..n_temp)
                .map(|i| temp_start + i as f32 * (temp_end-temp_start)/(n_temp as f32 -1.0))
                .collect();//ヒープ領域, 計算する温度範囲のデータ, temp0が領域所有者
            temp0
        };
        //println!("start {} do", temp_start);//error スコープ{}を抜けたので解放されている
        //println!("{:?}", temp0);//error スコープを抜けたのでtemp0も解放される
        println!("温度範囲: {:?}", temp);//tempはrun{}のスコープ内で有効
        let r_pt: Vec<f32> = temp.iter().map(|t| crate::kOhm!(1)*(1.0+3850e-6*t) ).collect();
        //tempはiter()により借用しているだけで元のデータも消えない
        println!("白金抵抗値: {:?}", r_pt);
        //println!("温度範囲: {:?}", temp); //上と同様にデータが残っている
        let mut vtemp_in_dc30 = vtemp_in(R_PT_30DO);
        let mut vtemp_gain = (C_SI/VOL_GAIN)/ vtemp_in_degree();//初期値は温度電圧側のゲインに合わせる
        let mut v_offset_out = vtemp_gain*vtemp_in_dc30;//温度電圧により出力に出るオフセット電圧
        println!("温度電圧係数: {:?}mV/do", vtemp_in_degree());
        println!("出力に出るオフセット: {:?}V", v_offset_out);
        //let mut r_temp1m:f32=;
        //-------------------------------///
        // 温度電圧によるオフセットの補正
        //-------------------------------///
        println!("\n-------------------------------");
        println!("出力に出るオフセットの補正");
        println!("-------------------------------");
        let (r_offset1, r_offset2, r_offset2m) =offset_func(v_offset_out);
        let v_temp_in: Vec<f32> = r_pt.iter().map(|r| vtemp_in(*r)).collect();
        println!("温度電圧入力: {:?}", v_temp_in);
        println!("r_offset1: {:?}kOhm, r_offset2: {:?}kOhm, r_offset2m: {:?}Ohm", r_offset1*1e-3, r_offset2*1e-3, r_offset2m);
        println!("\n-------------------------------");
        println!("出力温度電圧の補正");
        println!("-------------------------------");
        println!("入力の温度電圧{}, 温度電圧の目標値: {} mV/do", vtemp_in_degree()*1e3, C_SI/VOL_GAIN*1e3);
        vtemp_func(v_temp_in, r_offset1, r_offset2, r_offset2m);
        
    

    }
    
}//end mod optim_function