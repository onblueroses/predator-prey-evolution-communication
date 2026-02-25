---
globs: ["src/stats/**/*.rs"]
---

# Stats Module Rules

## Key metrics definitions
- **TopSim** (Topographic Similarity): correlation between signal distance and meaning distance. High TopSim (~0.5+) indicates compositional communication. Measure: Mantel test between signal-pair edit distances and referent-pair Euclidean distances.
- **Mutual Information (MI)**: I(Signal; Context) where Context = predator type/distance bin. High MI means signals carry information about the environment. Low MI = random signaling.
- **Deception rate**: fraction of signals emitted when no predator is within detection range. Some baseline is expected (exploratory signaling); sustained high rates indicate adversarial or noisy strategies.

## SignalEvent collection
Every signal emission must be recorded as a `SignalEvent` containing: emitter ID, symbol, emitter position, nearest predator type + distance (or None), tick, and a list of receivers (prey IDs within range). Missing any of these fields makes MI and TopSim calculations impossible.

## Export formats
- **CSV**: one row per generation, columns for all aggregate metrics. Header row required. Use consistent column ordering across runs.
- **JSON**: per-generation snapshots with nested signal events for detailed analysis. Use serde derive, not manual serialization.

## Collection frequency
Per-tick: signal events, prey deaths, food consumption. Per-generation: aggregate fitness stats, species count, TopSim, MI, deception rate. Do not compute expensive metrics (TopSim, MI) every tick - only at generation boundaries.

## No side effects
Stats collection is read-only. The collector observes world state but never modifies it. Stats types must not hold mutable references to World.
