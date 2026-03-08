// unit.rs
//宣言マクロの実装
// c++のようなユーザ定義リテラルはなさそうだった
pub mod electric_units {
    pub const PICO: f32 = 1e-12;
    pub const NANO: f32 = 1e-9;
    pub const MICRO: f32 = 1e-6;
    pub const MILLI: f32 = 1e-3;
    pub const KILO: f32 = 1e3;
    pub const MEGA: f32 = 1e6;
    pub const GIGA: f32 = 1e9;

    //-------------------------------///
    // 電圧の単位変換マクロ
    //-------------------------------///
    #[macro_export]//クレートルートレベルで公開
    macro_rules! nV {
        ($val:expr) => {
            ($val as f32)* 1e-9 // nVに変換
        };
    }
    #[macro_export]//クレートルートレベルで公開
    macro_rules! uV {
        ($val:expr) => {
            ($val as f32)* 1e-6 // uVに変換
        };
    }
    #[macro_export]//クレートルートレベルで公開
    macro_rules! mV {
        ($val:expr) => {
            ($val as f32)* 1e-3 // mVに変換
        };
    }
    #[macro_export]//クレートルートレベルで公開
    macro_rules! V {
        ($val:expr) => {
            ($val as f32)* 1.0 // Vに変換
        };
    }

    //-------------------------------///
    // 電流の単位変換マクロ
    //-------------------------------///
    #[macro_export]//クレートルートレベルで公開
    macro_rules! pA {
        ($val:expr) => {
            ($val as f32)* 1e-12 // pAに変換
        };
    }
    #[macro_export]//クレートルートレベルで公開
    macro_rules! nA {
        ($val:expr) => {
            ($val as f32)* 1e-9 // nAに変換
        };
    }
    #[macro_export]//クレートルートレベルで公開
    macro_rules! uA {
        ($val:expr) => {
            ($val as f32)* 1e-6 // uAに変換
        };
    }
    #[macro_export]//クレートルートレベルで公開
    macro_rules! mA  {
        ($val:expr) => {
            ($val as f32)* 1e-3 // mAに変換
        };
    }
    #[macro_export]//クレートルートレベルで公開
    macro_rules! A {
        ($val:expr) => {
            ($val as f32)* 1.0 // Aに変換
        };
    }
    //-------------------------------///
    // 抵抗値の単位変換マクロ
    //-------------------------------///
    #[macro_export]//クレートルートレベルで公開
    macro_rules! nOhm {
        ($val:expr) => {
            ($val as f32)* 1e-9 // nOhmに変換
        };
    }
    #[macro_export]//クレートルートレベルで公開
    macro_rules! uOhm {
        ($val:expr) => {
            ($val as f32)* 1e-6 // uOhmに変換
        };
    }
    #[macro_export]//クレートルートレベルで公開
    macro_rules! mOhm {
        ($val:expr) => {
            ($val as f32)* 1e-3 // mOhmに変換
        };
    }
    #[macro_export]//クレートルートレベルで公開
    macro_rules! Ohm {
        ($val:expr) => {
            ($val as f32)* 1.0 // Ohmに変換
        };
    }
    #[macro_export]//クレートルートレベルで公開
    macro_rules! kOhm {
        ($val:expr) => {
            (($val as f32)* 1e3) // kOhmに変換
        };
    }
    //-------------------------------///
    // 温度の単位変換マクロ
    //-------------------------------///
    #[macro_export]//クレートルートレベルで公開
    macro_rules! degree {
        ($val:expr) => {
            ($val as f32) // 度に変換
        };
    }

}

