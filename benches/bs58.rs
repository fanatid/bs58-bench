use {
    bs58_bench::generate,
    criterion::{criterion_group, criterion_main, Bencher, BenchmarkId, Criterion},
    solana_sdk::pubkey::Pubkey,
};

fn criterion_benchmark(c: &mut Criterion) {
    let input_pubkeys = generate(42, 100_000);
    let input_strings = input_pubkeys
        .iter()
        .map(|pk| pk.to_string())
        .collect::<Vec<_>>();

    fn encode(b: &mut Bencher<'_>, pubkeys: &Vec<Pubkey>) {
        b.iter(|| {
            for pubkey in pubkeys {
                let _pubkey: String = pubkey.to_string();
            }
        })
    }

    fn decode(b: &mut Bencher<'_>, pubkeys: &Vec<String>) {
        b.iter(|| {
            for pubkey in pubkeys {
                let _pubkey: Result<Pubkey, _> = pubkey.parse();
            }
        })
    }

    c.bench_with_input(
        BenchmarkId::new("bs58 encode", "10k"),
        &input_pubkeys.as_slice()[0..10_000].to_vec(),
        encode,
    );

    c.bench_with_input(
        BenchmarkId::new("bs58 encode", "100k"),
        &input_pubkeys.as_slice()[0..100_000].to_vec(),
        encode,
    );

    c.bench_with_input(
        BenchmarkId::new("bs58 decode", "10k"),
        &input_strings.as_slice()[0..10_000].to_vec(),
        decode,
    );

    c.bench_with_input(
        BenchmarkId::new("bs58 decode", "100k"),
        &input_strings.as_slice()[0..100_000].to_vec(),
        decode,
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
