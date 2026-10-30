use criterion::{Criterion, black_box, criterion_group, criterion_main};
use kayls_core::find_matches;

fn bench_find_matches(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_matches");

    // Uji dengan jumlah baris (bukan karakter)
    for lines in [1_000, 10_000, 100_000].iter() {
        // Setiap baris: "hello world aé\n" (16 karakter)
        let code = "hello world aé\n".repeat(*lines);
        let query = "hello";
        group.bench_with_input(format!("{}_lines", lines), lines, |b, _| {
            b.iter(|| {
                let matches = find_matches(black_box(&code), black_box(query));
                black_box(matches);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_find_matches);
criterion_main!(benches);
