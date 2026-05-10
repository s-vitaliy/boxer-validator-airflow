# agents.md

## 🗂️ Code Organization for Rust components (Expected)

The agent should respect and work within this structure:

```
model/
config.rs // hyperparameters
weights.rs // strongly typed weights
model.rs // forward_step()

runtime/
kv_cache.rs
state.rs
session.rs // autoregressive loop

math/
matmul.rs
softmax.rs
rmsnorm.rs
rope.rs

tokenizer/
simple.rs // minimal or stub tokenizer

io/
npy_loader.rs // load real weights from PyTorch exports
```

### Module Organization Principles

- **No `mod.rs` files**: Use `module_name.rs` instead of `module_name/mod.rs`
- **Tests in submodules**: Tests are in `module_name/tests.rs` (e.g., `math/matrix/tests.rs`)
- **Flat structure**: Top-level modules are single `.rs` files that declare submodules

---

## 🧪 Testing Expectations

The agent should prefer:

- unit tests on math primitives (matmul, softmax, RMSNorm)
- shape and invariant checks (especially KV-cache)
- golden tests comparing logits to PyTorch
- epsilon-based floating point comparisons

The agent MUST NOT rely on:
- “the model output looks reasonable”
- long prompt-based testing
- qualitative evaluation of generated text

---

## 🧨 Unsafe Code Policy

- `unsafe` is allowed **only** in clearly isolated numerical kernels
- each `unsafe` block must have:
  - a comment explaining why it is safe
  - a comment explaining what invariant it relies on
- the agent MUST NOT spread `unsafe` across high-level logic

---

## 🧭 Agent Behavior Guidelines

When proposing code or changes, the agent should:

- explain *why* the change exists
- explicitly state which part of the inference pipeline it affects
- prefer minimal diffs over refactors
- ask before expanding scope

If uncertain, the agent should:
- ask a clarifying question
- or propose the simplest possible version first
