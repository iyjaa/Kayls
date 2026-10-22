use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ropey::Rope;

fn bench_rope_from_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("rope");
    for size in [100, 1_000, 10_000].iter() {
        let text = (0..*size).map(|i| format!("line {}: some content here\n", i)).collect::<String>();
        group.bench_with_input(format!("from_str_{}_lines", size), &text, |b, text| {
            b.iter(|| {
                let rope = Rope::from_str(black_box(text));
                black_box(rope);
            });
        });
    }
    group.finish();
}

fn bench_rope_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("rope_insert");
    for size in [100, 1_000, 10_000].iter() {
        let text = (0..*size).map(|i| format!("line {}: some content here\n", i)).collect::<String>();
        group.bench_with_input(format!("insert_{}_lines", size), size, |b, _| {
            b.iter(|| {
                let mut rope = Rope::from_str(&text);
                let mid = rope.len_chars() / 2;
                for _ in 0..100 {
                    rope.insert_char(mid, 'x');
                }
                black_box(rope);
            });
        });
    }
    group.finish();
}


fn bench_fs_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("fs");
    
    // Buat file sementara 100 baris
    use std::io::Write;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    for i in 0..100 {
        writeln!(file, "line {}: some content here", i).unwrap();
    }
    let path = file.path().to_path_buf();

    group.bench_function("canonicalize_100_lines", |b| {
        b.iter(|| {
            let p = std::fs::canonicalize(black_box(&path)).unwrap();
            black_box(p);
        });
    });

    group.bench_function("read_to_string_100_lines", |b| {
        b.iter(|| {
            let s = std::fs::read_to_string(black_box(&path)).unwrap();
            black_box(s);
        });
    });

    group.finish();
}

criterion_group!(benches, bench_rope_from_str, bench_rope_insert, bench_fs_operations);

criterion_main!(benches);
