//! Typed schemas for primitives, the JSON-over-stdio protocol, and
//! initial-release LLM extension points.
//!
//! Each submodule mirrors one type group from
//! [`specs/022-deterministic-runtime/data-model.md`]:
//!
//! - [`procedure`] — the AST emitted by the procedure parser.
//! - [`protocol`] — the JSON-over-stdio envelope and message types.
//! - [`primitives`] — per-primitive args/result shapes.
//! - [`extensions`] — the three initial-release extension-point payloads.
//! - [`services`] — the `[services]` registry shape from the project config
//!   (spec 030 cross-service references).
//! - [`constitutions`] — the `[constitutions]` registry shape from the
//!   project config (spec 055 shared constitutions).
//! - [`paths`] — the `[paths]` block shape from the project config, resolving
//!   the configurable spec-root directory name (spec 040), and the
//!   three-tier `CONFIG_CHAIN` / `SESSION_CHAIN` resolution ladders themselves.
//!
//! Two constant registries also live here as the single source of truth for
//! their respective closed sets: [`registry`] (primitive names) and `status`
//! (spec lifecycle statuses). `status` is crate-internal; `registry` is public
//! because `main.rs` is a separate crate and its CLI-parity test has to name
//! the canonical set — see that module's docs.

pub mod constitutions;
pub mod extensions;
pub mod paths;
pub mod primitives;
pub mod procedure;
pub mod protocol;
pub mod registry;
pub mod services;
pub(crate) mod status;
