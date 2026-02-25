---
globs: ["src/signal/**/*.rs"]
---

# Signal Module Rules

## One-tick delay
Signals emitted on tick T are added to `World.signals` but NOT receivable until tick T+1. The propagation step runs at the START of each tick, making previous-tick signals available to sensors. This prevents instantaneous information transfer.

## Signal cost economics
Emitting a signal costs energy. The cost must be high enough that dishonest signaling (crying wolf) is selected against, following Hamilton's rule: cost < relatedness * benefit_to_kin. The fitness function in evolution/ calibrates this.

## Strongest-per-symbol reception
When multiple signals of the same symbol reach a prey, only the strongest (highest remaining strength after distance decay) is used for that symbol's sensor input. Do not average or sum - take the max.

## No truth verification
The signal system has NO built-in mechanism to verify signal truthfulness. A prey can emit any symbol regardless of actual danger. Honest signaling must emerge from evolutionary pressure alone. Never add code that checks if a signal "matches" reality.

## Signal propagation and decay
Signals have a broadcast range and decay with distance. `ActiveSignal` stores the origin position, symbol, initial strength, and tick emitted. The propagation system computes received strength as `initial_strength - (distance / range)` or similar decay. Signals with zero or negative strength are removed.

## Symbol vocabulary
`Symbol` is a u8 index into a fixed-size vocabulary (vocab_size in config). All symbols are semantically identical at initialization - meaning emerges only through evolution.
