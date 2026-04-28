// This benchmark file was created with the help of an AI assistant.

use crazy_linked_list::{safe_rc, safe_vec, unsafe_box};
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rand::{prelude::*, seq::SliceRandom};
use std::collections::LinkedList;

const SIZES: &[usize] = &[100, 1000, 10_000, 25_000, 50_000, 75_000, 100_000];

fn benchmark_push_front(c: &mut Criterion) {
    let mut group = c.benchmark_group("write::push_front");

    for size in SIZES.iter() {
        group.bench_with_input(
            BenchmarkId::new("safe_vec::LinkedList", size),
            size,
            |b, &s| {
                b.iter(|| {
                    let mut dll = safe_vec::LinkedList::new();
                    for i in 0..s {
                        dll.push_front(black_box(i));
                    }
                    black_box(dll)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("safe_rc::LinkedList", size),
            size,
            |b, &s| {
                b.iter(|| {
                    let mut dll = safe_rc::LinkedList::new();
                    for i in 0..s {
                        dll.push_front(black_box(i));
                    }
                    black_box(dll)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("unsafe_box::LinkedList", size),
            size,
            |b, &s| {
                b.iter(|| {
                    let mut dll = unsafe_box::LinkedList::new();
                    for i in 0..s {
                        dll.push_front(black_box(i));
                    }
                    black_box(dll)
                })
            },
        );

        group.bench_with_input(BenchmarkId::new("std::LinkedList", size), size, |b, &s| {
            b.iter(|| {
                let mut list = LinkedList::new();
                for i in 0..s {
                    list.push_front(black_box(i));
                }
                black_box(list)
            })
        });
    }

    group.finish();
}

fn benchmark_push_back(c: &mut Criterion) {
    let mut group = c.benchmark_group("write::push_back");

    for size in SIZES.iter() {
        group.bench_with_input(
            BenchmarkId::new("safe_vec::LinkedList", size),
            size,
            |b, &s| {
                b.iter(|| {
                    let mut dll = safe_vec::LinkedList::new();
                    for i in 0..s {
                        dll.push_back(black_box(i));
                    }
                    black_box(dll)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("safe_rc::LinkedList", size),
            size,
            |b, &s| {
                b.iter(|| {
                    let mut dll = safe_rc::LinkedList::new();
                    for i in 0..s {
                        dll.push_back(black_box(i));
                    }
                    black_box(dll)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("unsafe_box::LinkedList", size),
            size,
            |b, &s| {
                b.iter(|| {
                    let mut dll = unsafe_box::LinkedList::new();
                    for i in 0..s {
                        dll.push_back(black_box(i));
                    }
                    black_box(dll)
                })
            },
        );

        group.bench_with_input(BenchmarkId::new("std::LinkedList", size), size, |b, &s| {
            b.iter(|| {
                let mut list = LinkedList::new();
                for i in 0..s {
                    list.push_back(black_box(i));
                }
                black_box(list)
            })
        });
    }

    group.finish();
}

fn benchmark_pop_front(c: &mut Criterion) {
    let mut group = c.benchmark_group("read::pop_front");

    for size in SIZES.iter() {
        group.bench_with_input(
            BenchmarkId::new("safe_vec::LinkedList", size),
            size,
            |b, &s| {
                b.iter_batched(
                    || safe_vec::LinkedList::from_iter(0..*size),
                    |mut dll| {
                        for _ in 0..s {
                            black_box(dll.pop_front().unwrap());
                        }
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("safe_rc::LinkedList", size),
            size,
            |b, &s| {
                b.iter_batched(
                    || safe_rc::LinkedList::from_iter(0..*size),
                    |mut dll| {
                        for _ in 0..s {
                            black_box(dll.pop_front().unwrap());
                        }
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("unsafe_box::LinkedList", size),
            size,
            |b, &s| {
                b.iter_batched(
                    || unsafe_box::LinkedList::from_iter(0..*size),
                    |mut dll| {
                        for _ in 0..s {
                            black_box(dll.pop_front().unwrap());
                        }
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(BenchmarkId::new("std::LinkedList", size), size, |b, &s| {
            b.iter_batched(
                || LinkedList::from_iter(0..*size),
                |mut list| {
                    for _ in 0..s {
                        black_box(list.pop_front().unwrap());
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

fn benchmark_pop_back(c: &mut Criterion) {
    let mut group = c.benchmark_group("read::pop_back");

    for size in SIZES.iter() {
        group.bench_with_input(
            BenchmarkId::new("safe_vec::LinkedList", size),
            size,
            |b, &s| {
                b.iter_batched(
                    || safe_vec::LinkedList::from_iter(0..*size),
                    |mut dll| {
                        for _ in 0..s {
                            black_box(dll.pop_back().unwrap());
                        }
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("safe_rc::LinkedList", size),
            size,
            |b, &s| {
                b.iter_batched(
                    || safe_rc::LinkedList::from_iter(0..*size),
                    |mut dll| {
                        for _ in 0..s {
                            black_box(dll.pop_back().unwrap());
                        }
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("unsafe_box::LinkedList", size),
            size,
            |b, &s| {
                b.iter_batched(
                    || unsafe_box::LinkedList::from_iter(0..*size),
                    |mut dll| {
                        for _ in 0..s {
                            black_box(dll.pop_back().unwrap());
                        }
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(BenchmarkId::new("std::LinkedList", size), size, |b, &s| {
            b.iter_batched(
                || LinkedList::from_iter(0..*size),
                |mut list| {
                    for _ in 0..s {
                        black_box(list.pop_back().unwrap());
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

fn benchmark_iter_forward(c: &mut Criterion) {
    let mut group = c.benchmark_group("read::iter_forward");

    for size in SIZES.iter() {
        group.bench_with_input(
            BenchmarkId::new("safe_vec::LinkedList", size),
            size,
            |b, &_s| {
                let vec_ll = safe_vec::LinkedList::from_vec((0..*size).collect());
                b.iter(|| {
                    let mut sum = 0usize;
                    for v in vec_ll.iter() {
                        sum = sum.wrapping_add(*v);
                    }
                    black_box(sum)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("safe_rc::LinkedList", size),
            size,
            |b, &_s| {
                let rc_ll = safe_rc::LinkedList::from_iter(0..*size);
                b.iter(|| {
                    let mut sum = 0usize;
                    for v in rc_ll.iter() {
                        sum = sum.wrapping_add(*v);
                    }
                    black_box(sum)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("unsafe_box::LinkedList", size),
            size,
            |b, &_s| {
                let unsafe_ll = unsafe_box::LinkedList::from_iter(0..*size);
                b.iter(|| {
                    let mut sum = 0usize;
                    for v in unsafe_ll.iter() {
                        sum = sum.wrapping_add(*v);
                    }
                    black_box(sum)
                })
            },
        );

        group.bench_with_input(BenchmarkId::new("std::LinkedList", size), size, |b, &_s| {
            let std_ll = LinkedList::from_iter(0..*size);
            b.iter(|| {
                let mut sum = 0usize;
                for v in std_ll.iter() {
                    sum = sum.wrapping_add(*v);
                }
                black_box(sum);
            })
        });
    }

    group.finish();
}

fn benchmark_iter_backward(c: &mut Criterion) {
    let mut group = c.benchmark_group("read::iter_backward");

    for size in SIZES.iter() {
        group.bench_with_input(
            BenchmarkId::new("safe_vec::LinkedList", size),
            size,
            |b, &_s| {
                let vec_ll = safe_vec::LinkedList::from_vec((0..*size).collect());
                b.iter(|| {
                    let mut sum = 0usize;
                    for v in vec_ll.iter().rev() {
                        sum = sum.wrapping_add(*v);
                    }
                    black_box(sum)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("safe_rc::LinkedList", size),
            size,
            |b, &_s| {
                b.iter(|| {
                    let rc_ll = safe_rc::LinkedList::from_iter(0..*size);
                    let mut sum = 0usize;
                    for v in rc_ll.iter().rev() {
                        sum = sum.wrapping_add(*v);
                    }
                    black_box(sum)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("unsafe_box::LinkedList", size),
            size,
            |b, &_s| {
                let unsafe_ll = unsafe_box::LinkedList::from_iter(0..*size);
                b.iter(|| {
                    let mut sum = 0usize;
                    for v in unsafe_ll.iter().rev() {
                        sum = sum.wrapping_add(*v);
                    }
                    black_box(sum)
                })
            },
        );

        group.bench_with_input(BenchmarkId::new("std::LinkedList", size), size, |b, &_s| {
            let list = LinkedList::from_iter(0..*size);
            b.iter(|| {
                let mut sum = 0usize;
                for v in list.iter().rev() {
                    sum = sum.wrapping_add(*v);
                }
                black_box(sum);
            })
        });
    }

    group.finish();
}

fn benchmark_get_by_cursor(c: &mut Criterion) {
    let mut group = c.benchmark_group("read::get_by_cursor");

    for size in SIZES.iter() {
        // safe_vec::LinkedList with cursor-based access
        group.bench_with_input(
            BenchmarkId::new("safe_vec::LinkedList", size),
            size,
            |b, &s| {
                let mut dll = safe_vec::LinkedList::new();
                let mut cursors = Vec::new();
                for i in 0..s {
                    cursors.push(dll.push_back(i));
                }
                b.iter(|| {
                    let mut sum = 0usize;
                    for &cursor in &cursors {
                        sum = sum.wrapping_add(*dll.get(black_box(cursor)).unwrap());
                    }
                    black_box(sum)
                })
            },
        );

        // safe_rc::LinkedList - using iter().nth() as equivalent
        group.bench_with_input(
            BenchmarkId::new("safe_rc::LinkedList", size),
            size,
            |b, &s| {
                let mut dll = safe_rc::LinkedList::new();
                let mut cursors = Vec::new();
                for i in 0..s {
                    cursors.push(dll.push_back(i));
                }
                b.iter(|| {
                    let mut sum = 0usize;
                    for cursor in cursors.iter() {
                        sum = sum.wrapping_add(*dll.get(black_box(cursor.clone())));
                    }
                    black_box(sum)
                })
            },
        );

        // unsafe_box::LinkedList - using iter().nth() as equivalent
        group.bench_with_input(
            BenchmarkId::new("unsafe_box::LinkedList", size),
            size,
            |b, &s| {
                let mut dll = unsafe_box::LinkedList::new();
                let mut cursors = Vec::new();
                for i in 0..s {
                    cursors.push(dll.push_back(i));
                }

                b.iter(|| {
                    let mut sum = 0usize;
                    for cursor in cursors.iter() {
                        sum = sum.wrapping_add(*dll.get(black_box(cursor)).unwrap());
                    }
                    black_box(sum)
                })
            },
        );

        // std::LinkedList - has no cursor based access
    }

    group.finish();
}

fn benchmark_move_front(c: &mut Criterion) {
    let mut group = c.benchmark_group("move::move_front");

    for size in SIZES.iter() {
        // safe_vec::LinkedList with cursor-based move_front
        group.bench_with_input(
            BenchmarkId::new("safe_vec::LinkedList", size),
            size,
            |b, &s| {
                let mut dll = safe_vec::LinkedList::new();
                let mut cursors = Vec::new();
                for i in 0..s {
                    cursors.push(dll.push_back(i));
                }

                b.iter(|| {
                    for &cursor in &cursors {
                        let _ = dll.move_front(black_box(cursor));
                    }
                })
            },
        );

        // safe_rc::LinkedList - using iter and reconstruct as equivalent (no cursor move)
        group.bench_with_input(
            BenchmarkId::new("safe_rc::LinkedList", size),
            size,
            |b, &s| {
                let mut dll = safe_rc::LinkedList::new();
                let mut cursors = Vec::new();
                for i in 0..s {
                    cursors.push(dll.push_back(i));
                }
                b.iter(|| {
                    for cursor in cursors.iter() {
                        dll.move_front(black_box(cursor.clone()));
                    }
                })
            },
        );

        // unsafe_box::LinkedList - using iter and reconstruct as equivalent (no cursor move)
        group.bench_with_input(
            BenchmarkId::new("unsafe_box::LinkedList", size),
            size,
            |b, &s| {
                let mut dll = unsafe_box::LinkedList::new();
                let mut cursors = Vec::new();
                for i in 0..s {
                    cursors.push(dll.push_back(i));
                }

                b.iter(|| {
                    for cursor in &cursors {
                        let _ = dll.move_front(black_box(cursor));
                    }
                })
            },
        );

        // std::LinkedList has no move_front
    }

    group.finish();
}

fn benchmark_random_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("read::random_get_by_cursor");

    for size in SIZES.iter() {
        // safe_vec::LinkedList with cursor-based access
        group.bench_with_input(
            BenchmarkId::new("safe_vec::LinkedList", size),
            size,
            |b, &s| {
                let mut dll = safe_vec::LinkedList::new();
                let mut cursors = Vec::new();
                for i in 0..s {
                    cursors.push(dll.push_back(i));
                }
                cursors.shuffle(&mut thread_rng());

                b.iter(|| {
                    let mut sum = 0usize;
                    for &cursor in &cursors {
                        sum = sum.wrapping_add(*dll.get(black_box(cursor)).unwrap());
                    }
                    black_box(sum)
                })
            },
        );

        // safe_rc::LinkedList - using iter().nth() as equivalent
        group.bench_with_input(
            BenchmarkId::new("safe_rc::LinkedList", size),
            size,
            |b, &s| {
                let mut dll = safe_rc::LinkedList::new();
                let mut cursors = Vec::new();
                for i in 0..s {
                    cursors.push(dll.push_back(i));
                }
                cursors.shuffle(&mut thread_rng());

                b.iter(|| {
                    let mut sum = 0usize;
                    for cursor in cursors.iter() {
                        sum = sum.wrapping_add(*dll.get(black_box(cursor.clone())));
                    }
                    black_box(sum)
                })
            },
        );

        // unsafe_box::LinkedList - using iter().nth() as equivalent
        group.bench_with_input(
            BenchmarkId::new("unsafe_box::LinkedList", size),
            size,
            |b, &s| {
                let mut dll = unsafe_box::LinkedList::new();
                let mut cursors = Vec::new();
                for i in 0..s {
                    cursors.push(dll.push_back(i));
                }
                cursors.shuffle(&mut thread_rng());

                b.iter(|| {
                    let mut sum = 0usize;
                    for cursor in cursors.iter() {
                        sum = sum.wrapping_add(*dll.get(black_box(cursor)).unwrap());
                    }
                    black_box(sum)
                })
            },
        );

        // std::LinkedList - has no cursor based access
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_push_front,
    benchmark_push_back,
    benchmark_pop_front,
    benchmark_pop_back,
    benchmark_iter_forward,
    benchmark_iter_backward,
    benchmark_get_by_cursor,
    benchmark_move_front,
    benchmark_random_access,
);

criterion_main!(benches);
