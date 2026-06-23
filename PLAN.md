# Lema OS — Project Plan

> A hobby operating system for x86_64, built in Rust and C.
> **Status:** Phase 0 — Project foundation. Repo, docs, and toolchain only. Not yet booting.

This is the canonical living plan. If something in the README or docs contradicts this file, this file wins.

---

## Vision

Lema is a from-scratch operating system designed to be:

- **Real** — boots on actual hardware (HP G3840 primary target), runs real programs.
- **Open** — MIT licensed, public from day one, contribution-friendly.
- **Educational** — well-documented, small enough to fully understand, written like a learning resource.
- **Useful** — eventually usable for personal projects, not a toy that dies after the screencast.

## Goals

- Boot on x86_64 bare metal (UEFI + legacy BIOS).
- Provide a POSIX-ish syscall surface (~40 syscalls, Linux-shaped).
- Run real userland programs compiled from C.
- Have a working shell, file utilities, and basic networking.
- Be a portfolio piece — documented, tested, well-architected.

## Non-Goals

- Production-grade security (no SELinux, no formally verified code).
- Full Linux binary compatibility.
- Mobile / embedded / ARM (x86_64 desktop only for v1).
- Real-time guarantees.
- SMP (multi-core) in v1 — deferred to Phase 8.

---

## Architectural Decisions

These are the calls we've made up front. Each one has a rationale — don't relitigate without evidence.

| # | Decision | Choice | Why |
|---|---|---|---|
| 1 | Kernel architecture | **Monolithic** | Friendlier first kernel, faster path to userland, more tutorials. Microkernel trades dev time for elegance. |
| 2 | ISA | **x86_64, long mode only** | HP G3840 (Coffee Lake) and QEMU both support it natively. No 32-bit legacy baggage. |
| 3 | Bootloader | **Limine** | Modern, clean, Multiboot2-compatible, plays well with Rust via the `limine` crate. |
| 4 | Kernel core language | **Rust** | Memory safety without GC, in a context where bugs are unrecoverable. Mature `no_std` ecosystem. |
| 5 | Low-level bits | **C + NASM** | Context switch trampolines and GDT stubs are easier here. |
| 6 | Userland | **C** | Cross-compile with musl-gcc, real programs. Lets us reuse existing tooling. |
| 7 | Build orchestration | **Cargo + Make + just** | Each tool does what it's best at. `just` ties them together with sane defaults. |
| 8 | License | **MIT** | Maximum openness — anyone can fork, learn, modify, redistribute. |

---

## Phased Roadmap

See [docs/src/roadmap.md](docs/src/roadmap.md) for live status. High-level horizon:

| Phase | Name | Horizon | Milestone |
|---|---|---|---|
| 0 | Foundation | This weekend | Repo live, toolchain verified, `cargo check` green |
| 1 | Boot to pixels | +2 wk | Lema logo on framebuffer in QEMU |
| 2 | Memory | +5 wk | `Box` / `Vec` / `String` work in kernel |
| 3 | Interrupts & I/O | +7 wk | Type on keyboard, see it on screen |
| 4 | Processes & scheduling | +10 wk | Kernel launches a second program |
| 5 | Filesystem & ELF | +13 wk | `lemash` loads and runs ELF binaries |
| 6 | Userland | +18 wk | Interactive shell with files & utilities |
| 7 | Real hardware | +22 wk | Lema boots on the HP G3840 |
| 8 | Beyond | Post-attachment | Network, GUI, SMP |

**Total core OS: ~6 months of weekends at ~10–15 hrs/wk.** Sized to survive attachment hours.

Each phase ends with something demoable. If you stop at any phase, you still have a thing that works.

---

## Tech Stack

| Layer | Tool |
|---|---|
| Bootloader | Limine (BIOS + UEFI, Multiboot2-compatible) |
| Kernel | Rust (`#![no_std]`, stable initially, nightly as features require) |
| Low-level asm | NASM (a handful of files for context switch, GDT stubs) |
| Userland | C, cross-compiled with musl-gcc |
| Build | Cargo + Make + just |
| Test | QEMU primary, real HP G3840 at milestones |
| CI | GitHub Actions |
| Docs | mdBook |

---

## Repository Layout

```
lema-os/
├── PLAN.md                  # This file — living plan, canonical source of truth
├── README.md                # GitHub landing page
├── LICENSE                  # MIT
├── .gitignore
├── Cargo.toml               # Workspace root
├── justfile                 # Dev commands (build, run, test, debug, clean)
├── .github/
│   └── workflows/
│       └── ci.yml           # GitHub Actions: toolchain check + cargo check
├── docs/                    # mdBook source
│   ├── book.toml
│   └── src/
│       ├── SUMMARY.md
│       ├── introduction.md
│       ├── architecture.md
│       ├── roadmap.md
│       └── contributing.md
├── kernel/                  # Rust kernel crate
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs          # _start (Phase 1: Limine entry)
│       └── lib.rs           # Kernel library root
├── boot/                    # Limine bootloader config + ISO build (Phase 1+)
│   ├── README.md
│   └── limine.conf
├── userland/                # C userland programs (Phase 5+)
│   └── README.md
└── toolchain/               # Cross-compiler scripts (Phase 5+, deferred)
```

---

## Development Workflow

```bash
git clone https://github.com/<your-username>/lema-os
cd lema-os

just toolchain-check    # verify Rust, QEMU, NASM, just installed
just check              # cargo check against x86_64-unknown-none
just build              # (Phase 1+) build the kernel
just run                # (Phase 1+) boot in QEMU
just test               # (Phase 1+) headless boot test
just debug              # (Phase 1+) QEMU + GDB attach
just clean              # clean all artifacts
```

---

## Phase 0 Success Criteria

- [x] Repo created and public on GitHub
- [x] All doc files in place (PLAN, README, ARCHITECTURE, ROADMAP, CONTRIBUTING)
- [x] Cargo workspace + kernel crate skeleton compiles (`just check`)
- [x] `just toolchain-check` passes
- [x] GitHub Actions CI green on first push
- [ ] (Recommended) Clone a known-good Rust-OS hello world (e.g. `blog_os`) and run it in QEMU — validates the full pipeline end-to-end before any of it is Lema's fault.

---

## Risk Register

Things that could derail us, and how we're handling them.

| Risk | Mitigation |
|---|---|
| Toolchain friction (rustup + custom target + Limine all being weird) | Phase 0 explicitly validates the full pipeline with a known-good sample. |
| Real hardware is finicky (UEFI quirks, ACPI nightmares) | 95% dev in QEMU; real HW only at milestones. |
| Attachment eats weekends | Each phase = 1 weekend arc with a clear stop point. |
| Scope creep into networking / GUI / SMP | Deferred to Phase 8, tracked in ROADMAP, don't promise dates. |
| Rust nightly churn | Pin nightly in `rust-toolchain.toml` once we use it (Phase 1+). |

---

## Open Questions

Things we've punted on. Revisit when relevant.

- **Process/thread model.** Leaning: fork+exec + 1:1 threads in v1. POSIX-ish but not strict.
- **C library.** Leaning: write `lemalibc` from scratch — more educational. Re-evaluate in Phase 6.
- **ACPI.** Yes, eventually (power management, shutdown). Deferred to Phase 7.
- **Filesystem for root.** FAT32 in Phase 5 (simple, well-documented). Custom LemaFS someday if motivated.
- **Networking stack.** Phase 8. Will likely be a `smoltcp` port or hand-rolled TCP/IP.

---

## License

MIT. See [LICENSE](LICENSE).
