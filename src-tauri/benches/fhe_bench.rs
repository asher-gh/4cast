use app::forecast::{enc_sma, EncSMAInput};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use csv::Reader;

fn bench(c: &mut Criterion) {
    let bed_data = String::from(
        "Date,Beds(England)
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
18-Aug-20,597",
    );

    // Initialise FHE
    let rdr = Reader::from_reader(bed_data.as_bytes());
    let mut input = EncSMAInput::new(rdr);

    c.bench_function("Encrypted SMA", |b| {
        input.rdr = Reader::from_reader(bed_data.as_bytes());
        b.iter(|| enc_sma(black_box(&mut input)).unwrap())
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
