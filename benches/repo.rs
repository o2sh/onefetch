use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gix::{open, ThreadSafeRepository};
use onefetch::{cli::CliOptions, info::build_info};

fn bench_repo_info(c: &mut Criterion) {
    let name = "repo.sh".to_string();
    let repo_path = gix_testtools::scripted_fixture_read_only(name).unwrap();
    let repo = ThreadSafeRepository::open_opts(repo_path, open::Options::isolated()).unwrap();
    let config: CliOptions = CliOptions {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };

    c.bench_function("get repo information", |b| {
        b.iter(|| {
            let result = black_box(build_info(&config));
            assert!(result.is_ok());
        });
    });
}

criterion_group!(benches, bench_repo_info);
criterion_main!(benches);
