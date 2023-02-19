use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gix::{open, ThreadSafeRepository};
use onefetch::{cli::Config, info::Info};

fn bench_repo_info(c: &mut Criterion) {
    let name = "repo.sh".to_string();
    let repo_path = git_testtools::scripted_fixture_read_only(name).unwrap();
    let repo = ThreadSafeRepository::open_opts(repo_path, open::Options::isolated()).unwrap();
    let config: Config = Config {
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
