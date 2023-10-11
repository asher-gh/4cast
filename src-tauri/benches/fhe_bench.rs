use app::forecast::{enc_sma, FheProgramState};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use csv::Reader;

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

fn bench_init(c: &mut Criterion) {
    c.bench_function("Fhe Init", |b| {
        // input.rdr = Reader::from_reader(bed_data.as_bytes());
        b.iter(|| {
            let rdr = Reader::from_reader(DATA.as_bytes());
            let _ = FheProgramState::new(black_box(Some(rdr)));
        })
    });
}

fn bench_enc_sma(c: &mut Criterion) {
    // Initialise FHE
    let rdr = Reader::from_reader(DATA.as_bytes());
    let mut input = FheProgramState::new(Some(rdr));

    c.bench_function("Encrypted SMA", |b| {
        input.rdr = Some(Reader::from_reader(DATA.as_bytes()));
        b.iter(|| enc_sma(black_box(&mut input)).unwrap())
    });
}

criterion_group!(benches, bench_init, bench_enc_sma);
criterion_main!(benches);
