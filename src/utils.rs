use rand::distributions::Standard;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    // RNGを初期化する。再現性を持たせるため毎回同じシード値を使う
    let mut rng = Pcg64Mcg::from_seed([0; 16]);

    // n個の要素が格納できるようベクタを初期化する
    let mut v = Vec::with_capacity(n);

    // 0からn-1までの合計n回、繰り返し乱数を生成し、ベクタに追加する
    // (0からn-1の数列は使わないので、_で受けることですぐに破棄している)
    for _ in 0..n {
        // RNGのsampleメソッドは引数として与えられた分布に従う乱数を1つ生成する

        // Standard分布は生成する値が数値型(ここではu32型)のときは一様分布になる
        // つまり、その型が取りうる全ての値が同じ確率で出現する
        v.push(rng.sample(&Standard));
    }

    // ベクタを返す
    v
}
