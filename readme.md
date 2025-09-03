# parking-stress

Reproducing a deadlock with upgradable readers.

It seems the `deadlock_detection` feature creates deadlock.

Simply `cargo run` and get after a few minutes get:

```
[2025-09-03T13:48:04.890Z INFO  parking_stress] 718 requesting upgradable write access
[2025-09-03T13:48:04.904Z INFO  parking_stress] 766 requesting read access
[2025-09-03T13:48:04.965Z INFO  parking_stress] 767 requesting read access
[2025-09-03T13:48:04.977Z INFO  parking_stress] 768 requesting read access
[2025-09-03T13:48:05.019Z INFO  parking_stress] 769 requesting read access
[2025-09-03T13:48:05.030Z INFO  parking_stress] 770 requesting read access
[2025-09-03T13:48:05.343Z ERROR parking_stress] 1 deadlocks detected
    Deadlock #0
    Thread Id 136378943760064
       0:     0x57db1ca81143 - parking_lot_core::parking_lot::deadlock_impl::on_unpark::he273ac1dd412a8c2
                                   at /home/agourlay/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.11/src/parking_lot.rs:1211:32
       1:     0x57db1ca45cdb - parking_lot_core::parking_lot::deadlock::on_unpark::h2a1b2c41e9bbdca2
                                   at /home/agourlay/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.11/src/parking_lot.rs:1144:9
       2:     0x57db1ca4482f - parking_lot_core::parking_lot::park::{{closure}}::hed3d5d78129fc2c3
                                   at /home/agourlay/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.11/src/parking_lot.rs:637:17
       3:     0x57db1ca42c97 - parking_lot_core::parking_lot::with_thread_data::h441ed05b07c4f512
                                   at /home/agourlay/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.11/src/parking_lot.rs:207:5
                               parking_lot_core::parking_lot::park::h0bbc060b0481a374
                                   at /home/agourlay/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.11/src/parking_lot.rs:600:5
       4:     0x57db1ca4b04c - parking_lot::raw_rwlock::RawRwLock::wait_for_readers::h2c6fb81f052a6f7e
                                   at /home/agourlay/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.4/src/raw_rwlock.rs:1022:17
       5:     0x57db1ca4a7ab - parking_lot::raw_rwlock::RawRwLock::upgrade_slow::h6c49c43caf111146
                                   at /home/agourlay/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.4/src/raw_rwlock.rs:875:14
       6:     0x57db1c9fb8f0 - <parking_lot::raw_rwlock::RawRwLock as lock_api::rwlock::RawRwLockUpgrade>::upgrade::h1436c0376a214b1b
                                   at /home/agourlay/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.4/src/raw_rwlock.rs:374:31
       7:     0x57db1c9f5a9d - lock_api::rwlock::RwLockUpgradableReadGuard<R,T>::upgrade::hd8e23c93d41104ea
                                   at /home/agourlay/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.13/src/rwlock.rs:1946:26
       8:     0x57db1c9f7df6 - parking_stress::Shard::upgrade::h9188d2f3290b43fe
                                   at src/main.rs:37:34
       9:     0x57db1c9fe80f - parking_stress::main::{{closure}}::h2c853e8382fbcb98
                                   at src/main.rs:158:27
      10:     0x57db1c9fa416 - std::sys::backtrace::__rust_begin_short_backtrace::hc30acf4b6cf7a57a
                                   at /home/agourlay/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs:152:18
      11:     0x57db1c9f4983 - std::thread::Builder::spawn_unchecked_::{{closure}}::{{closure}}::h28cf380275620c04
                                   at /home/agourlay/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/mod.rs:559:17
      12:     0x57db1c9ffc20 - <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h57193f375e101aa8
                                   at /home/agourlay/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panic/unwind_safe.rs:272:9
      13:     0x57db1c9f705b - std::panicking::catch_unwind::do_call::h12e95d0f488a34a2
                                   at /home/agourlay/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:589:40
      14:     0x57db1c9f54db - __rust_try
      15:     0x57db1c9f2ee5 - std::panicking::catch_unwind::ha8447ca25f4ed665
                                   at /home/agourlay/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:552:19
                               std::panic::catch_unwind::h308e3a112e5ef14e
                                   at /home/agourlay/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:359:14
                               std::thread::Builder::spawn_unchecked_::{{closure}}::h6c3756c1c40300c1
                                   at /home/agourlay/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/mod.rs:557:30
      16:     0x57db1c9ffdfe - core::ops::function::FnOnce::call_once{{vtable.shim}}::h6f9c5dfc9a3714be
                                   at /home/agourlay/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
      17:     0x57db1cbd48ef - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h8703e59bc8145d18
                                   at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/alloc/src/boxed.rs:1966:9
                               std::sys::pal::unix::thread::Thread::new::thread_start::h1ff51d6e85162efd
                                   at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/sys/pal/unix/thread.rs:107:17
      18:     0x7c0933aa27f1 - start_thread
                                   at ./nptl/pthread_create.c:448:8
      19:     0x7c0933b33c9c - __GI___clone3
                                   at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:78:0
      20:                0x0 - <unknown>
```
