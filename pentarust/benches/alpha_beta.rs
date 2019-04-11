use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BatchSize;
use criterion::Benchmark;
use criterion::Criterion;
use pentarust::alpha_beta::negamax;
use pentarust::alpha_beta::TranspositionTable;
use pentarust::game::Board;

fn alpha_beta_benchmark(c: &mut Criterion) {
    c.bench(
        "alpha-beta",
        Benchmark::new("alpha-beta from root, depth 3+4", |b| {
            b.iter_batched_ref(
                || TranspositionTable::new(1_000_000),
                |transpo_table| {
                    let board = Board::default();
                    black_box(negamax(
                        board,
                        3,
                        -1_000_000,
                        1_000_000,
                        transpo_table,
                        &|| false,
                    ));
                    black_box(negamax(
                        board,
                        4,
                        -1_000_000,
                        1_000_000,
                        transpo_table,
                        &|| false,
                    ));
                },
                BatchSize::PerIteration,
            )
        })
        .sample_size(10),
    );
}

criterion_group!(benches, alpha_beta_benchmark);
criterion_main!(benches);
