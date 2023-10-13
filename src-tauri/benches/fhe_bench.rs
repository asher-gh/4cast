use std::{fs::File, sync::Arc};

use app::forecast::{enc_sma, f64_div, f64_sum, F64SumSMA, FheProgramState};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use csv::Reader;
use sunscreen::types::bfv::Fractional;
use sunscreen_runtime::Ciphertext;

const DATA: &str = "Date,Beds(England)
1-Aug-20,879
2-Aug-20,847
3-Aug-20,842
4-Aug-20,807
5-Aug-20,805
6-Aug-20,756
7-Aug-20,711
8-Aug-20,650
9-Aug-20,611
10-Aug-20,714
11-Aug-20,672
12-Aug-20,642
13-Aug-20,652
14-Aug-20,642
15-Aug-20,630
16-Aug-20,634
17-Aug-20,626
18-Aug-20,597";

fn fhe_init(c: &mut Criterion) {
    c.bench_function("Fhe Init", |b| {
        // input.rdr = Reader::from_reader(bed_data.as_bytes());
        b.iter(|| {
            let rdr = Reader::from_reader(DATA.as_bytes());
            let _ = FheProgramState::new(black_box(Some(rdr)));
        })
    });
}

// fn bench_enc_sma(c: &mut Criterion) {
//     // FIX: panics on subsequent runs
//     c.bench_function("Encrypted SMA", |b| {
//         let mut fhe_prog = FheProgramState::new(Some(csv::Reader::from_reader(DATA.as_bytes())));
//         b.iter(|| enc_sma(Arc::from(fhe_prog)).unwrap())
//     });
// }
//

fn encryption(c: &mut Criterion) {
    let fhe_prog = FheProgramState::new(Some(Reader::from_reader(DATA.as_bytes())));

    c.bench_function("encryption", |b| {
        b.iter(|| {
            fhe_prog
                .runtime
                .encrypt(
                    black_box(Fractional::<64>::from(10000.0)),
                    &fhe_prog.pub_key,
                )
                .unwrap();
        })
    });
}

fn decryption(c: &mut Criterion) {
    let fhe_prog = FheProgramState::new(Some(Reader::from_reader(DATA.as_bytes())));

    let enc = fhe_prog
        .runtime
        .encrypt(
            black_box(Fractional::<64>::from(10000.0)),
            &fhe_prog.pub_key,
        )
        .unwrap();

    c.bench_function("decryption", |b| {
        b.iter(|| {
            fhe_prog
                .runtime
                .decrypt::<Fractional<64>>(black_box(&enc), &fhe_prog.priv_key)
                .unwrap()
        })
    });
}

fn fhe_mean(c: &mut Criterion) {
    let fhe_prog = FheProgramState::new(Some(Reader::from_reader(DATA.as_bytes())));

    let x = fhe_prog
        .runtime
        .encrypt(
            black_box(Fractional::<64>::from(10000.0)),
            &fhe_prog.pub_key,
        )
        .unwrap();
    let y = fhe_prog
        .runtime
        .encrypt(
            black_box(Fractional::<64>::from(10000.0)),
            &fhe_prog.pub_key,
        )
        .unwrap();

    c.bench_function("fhe mean", |b| {
        b.iter(|| {
            let sum = fhe_prog
                .runtime
                .run(
                    fhe_prog.app.get_fhe_program(f64_sum).unwrap(),
                    F64SumSMA::<File>::gen_args(vec![x.clone(), y.clone()]),
                    &fhe_prog.pub_key,
                )
                .unwrap()[0]
                .clone();

            &fhe_prog
                .runtime
                .run(
                    fhe_prog.app.get_fhe_program(f64_div).unwrap(),
                    F64SumSMA::<File>::gen_args(vec![sum]),
                    &fhe_prog.pub_key,
                )
                .unwrap()[0];
        })
    });
}

criterion_group!(benches, fhe_init, encryption, decryption, fhe_mean);
criterion_main!(benches);
