---
globs: ["src/evolution/**/*.rs"]
---

# Evolution Module Rules

## Generation lifecycle (8 steps)
1. Reset innovation counter for this generation
2. Evaluate fitness for all prey (survival time + food + kin bonuses + communication metrics)
3. Assign prey to species (compatibility distance threshold)
4. Adjust fitness by species size (explicit fitness sharing)
5. Remove stagnant species (no fitness improvement for N generations) - EXCEPTION: never remove the species with the overall best genome
6. Select parents via tournament selection within each species
7. Produce offspring: crossover from two parents, then mutate
8. Replace population, carry over elites unchanged

## Speciation requires sorted connections
Compatibility distance uses innovation-number alignment to compare genomes. If connections are not sorted by innovation number, the alignment is wrong and speciation breaks silently. Always trust that `sort_connections()` has been called - do not re-sort inside speciation.

## Crossover from fitter parent
When crossing two genomes, the fitter parent's topology dominates: excess and disjoint genes come from the fitter parent only. Matching genes (same innovation number) randomly pick from either parent. If fitness is equal, pick the shorter genome as primary.

## Mutation rates
All mutation probabilities come from `SimConfig`. Never hardcode mutation rates. The config file is the single source of truth for evolutionary parameters.

## Stagnation exception
When removing stagnant species, ALWAYS keep the species containing the single best genome across the entire population, even if that species is stagnant. Losing the best genome is catastrophic for evolutionary progress.
