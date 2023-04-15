# Qautomata
Qautomata is a two dimensional quantum [cellular automata](https://en.wikipedia.org/wiki/Cellular_automaton).
Given a starting [universe](#universe) with a single [configuration](#configuration) containing some living [cells](#cell) and a set of [rules](#rules), it computes the evolution of the [universe](#universe).

## Screenshot

## How does it work

## Developpement

### Dependencies
- [Rust](https://www.rust-lang.org/tools/install)

### Architecture
The project is divided into 3 parts:
- core: contains the datas and computing part of the qautomata
- ui: desktop ui implemented with [nannou](https://nannou.cc/), used to visualize and interact with the qautomata
- web: launch the ui into the web (not fully functional yet)

### How to run

#### UI
From the root of the repository run `cargo run -p ui [state file]`

#### Web
To test the web ui:
- install `npm` and `node` > 18
- install `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
- move to the web folder `cd ./web`
- install deps `npm install`
- run the web app `npm run start`

To test the web application as a binary:
- from the root of the repository run `cargo run -p web`

## Glossary

### Universe
An instance of the qautomata, it contains:
- A [state](#state)
- A set of [rules](#rules)
- The parity of the current [step](#step)
- The [generation](#generation) count (number of generation elapsed since the beginning).

### State
A list of [superposed](https://en.wikipedia.org/wiki/Quantum_superposition) [configurations](#configuration).

### Configuration
A grid of [cells](#cell) that can either be dead or alive, asssociated with an [amplitude](#amplitude).

### Cell
An element of the grid of the [configuration](#configuration) that can either be dead or alive.

### Amplitude
A complex number asociated with a [configuration](#configuration), it can be used to compute the [probability](#probability) associated with it.

### Probability
[Probability](#probability) of a configuration to be selected in case of a [measure](#measure). It's the [squared norm](https://en.wikipedia.org/wiki/Norm_(mathematics)) of the [amplitude](#amplitude).

### Measure
Randomly select a [configuration](#configuration) from the [state](#state), set its amplitude to 1 and remove all other [configurations](#configuration). The random selection is made with a [density probability](#probability) computed with the [amplitudes](#amplitude) of the [configurations](#configuration).

### Rules
A set of rule for the universe, see [Operator matrix](#operator-matrix).

### Operator matrix
A 16*16 matrix used to compute the [steps](#steps) of the universe. TODO improve

### Step
Computation of a new [Generation](#generation).

### Generation
An instant of the [universe](#universe).

### Interference
When several [configurations](#configuration) have exactly the same alive [cells](#cell), they interfer and merge into one [configuration](#configuration) with their [amplitudes](#amplitude) added.
