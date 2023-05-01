use criterion::{black_box, criterion_group, criterion_main, Criterion};
use onefetch::{cli::CliOptions, info::Info};
use onefetch::utils::repo;

fn bench_repo_info(c: &mut Criterion) {
    let repo = repo("repo.sh").unwrap();
    let config: CliOptions = CliOptions {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };

    c.bench_function("get repo information", |b| {
        b.iter(|| {
            let result = black_box(Info::new(&config));
            assert!(result.is_ok());
        })
    });
}

criterion_group!(benches, bench_repo_info);
criterion_main!(benches);
