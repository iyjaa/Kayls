use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kayls_core::Document;
use std::io::Write;
use tempfile::NamedTempFile;

fn make_temp_file(size: usize) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    for i in 0..size {
        writeln!(file, "line {}: some content here for benchmarking", i).unwrap();
    }
    file
}

fn bench_document_open(c: &mut Criterion) {
    let mut group = c.benchmark_group("document_open");
    for size in [100, 1_000, 10_000].iter() {
        let file = make_temp_file(*size);
        group.bench_with_input(format!("{}_lines", size), size, |b, _| {
            b.iter(|| {
                let doc = Document::open(file.path()).unwrap();
                black_box(doc);
            });
        });
    }
    group.finish();
}

fn bench_document_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("document_insert");
    for size in [100, 1_000, 10_000].iter() {
        let file = make_temp_file(*size);
        // Buka file DI LUAR loop agar tidak mengukur I/O
        let mut doc = Document::open(file.path()).unwrap();
        doc.cursor.line = *size / 2;
        doc.cursor.col = 0;
        group.bench_with_input(format!("{}_lines", size), size, |b, _| {
            b.iter(|| {
                // Clone ringan? Tidak, kita langsung insert ke doc yang sama
                // (ini mengukur insert, bukan open)
                for _ in 0..100 {
                    doc.insert_char('x');
                }
                black_box(&doc);
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_document_open, bench_document_insert);
criterion_main!(benches);
