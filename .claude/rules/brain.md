---
globs: ["src/brain/**/*.rs"]
---

# Brain Module Rules

## Dependency boundary
brain/ must NEVER import from world/, agent/, or signal/. It is a pure neural network library. If you need world state, pass it as function parameters from the caller.

## Connection sort invariant
`NeatGenome.connections` must be sorted by `InnovationNumber` at all times. After ANY mutation that adds or reorders connections, call `genome.sort_connections()`. Crossover alignment depends on this. `validate()` (debug builds) will panic if violated.

## Innovation numbers
Same structural mutation (same from-node, same to-node) in the same generation gets the same innovation number from `InnovationCounter`. This is what makes NEAT crossover alignment work. Never assign innovation numbers manually - always go through `InnovationCounter::get_connection_innovation()`.

## Feedforward enforcement
`NeatNetwork::from_genome()` does a topological sort. Any connections forming cycles are silently dropped (nodes unreachable in topo order are excluded). Do not add recurrent connection support - the architecture is feedforward-only.

## Node ordering in genome
Input nodes come first (indices 0..input_count), then hidden, then output nodes last. `NeatNetwork::activate()` relies on this: it reads inputs from the first N nodes and outputs from the last M nodes in topological order.

## Activation functions
`ActivationFn` enum in activation.rs. Hidden nodes can use any variant. Output nodes use `Sigmoid` (action probabilities) or `Tanh` (signal values). Do not add activation functions without updating `NeatNetwork::activate()`.
