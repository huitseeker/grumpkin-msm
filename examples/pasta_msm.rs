#[cfg(feature = "cuda")]
use grumpkin_msm::cuda_available;

use grumpkin_msm::pasta::utils::{
    gen_points, gen_scalars, naive_multiscalar_mul,
};
use pasta_curves::group::Curve;

fn main() {
    let bench_npow: usize = std::env::var("BENCH_NPOW")
        .unwrap_or("17".to_string())
        .parse()
        .unwrap();
    let npoints: usize = 1 << bench_npow;

    println!("generating {} random points, just hang on...", npoints);
    let points = gen_points(npoints);
    let scalars = gen_scalars(npoints);

    #[cfg(feature = "cuda")]
    if unsafe { cuda_available() } {
        unsafe { grumpkin_msm::CUDA_OFF = false };
    }

    let res = grumpkin_msm::pasta::pallas(&points, &scalars).to_affine();
    let native = naive_multiscalar_mul(&points, &scalars);
    assert_eq!(res, native);
    println!("success!")
}
