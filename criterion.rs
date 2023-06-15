fn main() {
    let mut criterion = criterion::Criterion::default().configure_from_args();
    ruint2::bench::group(&mut criterion);
    criterion.final_summary();
}
