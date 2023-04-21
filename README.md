# Qautomata
Qautomata is a two dimensional quantum [cellular automata](https://en.wikipedia.org/wiki/Cellular_automaton).  
Given a starting [configuration](#configuration) containing some living [cells](#cell) and a set of [rules](#rules), it computes the evolution of the [universe](#universe).

## How does it work

### Automata algorithm

#### Start
We start with a [universe](#universe) containing one [configuration](#configuration) with some living [cells](#cell) and a set of [rules](#rules) represented by an [operator matrix](#operator-matrix).

#### Step computation
During each [step](#step) and for each [configuration](#configuration) in the [global state](#global-state), we apply the [rules](#rules) locally on 2\*2 squares that alternate at each [step](#step):  
- On even [steps](#step), the [rules](#rules) apply on each 2\*2 square outlined in black:

![6-6-square-even](https://user-images.githubusercontent.com/11985913/232360559-3c87237f-855a-4c30-b6da-a0201eb273a2.png)

- On odd [steps](#step), the [rules](#rules) apply on each 2\*2 square outlined in black (some cells are visually missing, but they are here in the computation):

![6-6-square-odd](https://user-images.githubusercontent.com/11985913/232360353-cb827f1c-4d44-4276-81c5-b4e3baee3549.png)

This alternation allows the propagation of living [cells](#cell) in the entire [universe](#universe).

We compute the next state of each local 2\*2 square with at least one living [cell](#cell) using the [operator matrix](#operator-matrix).  
We obtain a list (with at least one element) of 2\*2 square state with each element associated with a complex number.  
This list is obtained by computing the product of the vector that represents the state of a local 2\*2 square and the [operator matrix](#operator-matrix).  
- If there is only one element in the list, we update the [configuration](#configuration) with the new 2\*2 square state and multiply its [amplitude](#amplitude) by the complex number associated with the new 2\*2 square state. (see [visual example](#visual-example) step 5-8)
- If there are several elements in the list, we split the [configuration](#configuration) into [superposed](https://en.wikipedia.org/wiki/Quantum_superposition) [configurations](#configuration), one for each element in the list. We then update each new [configuration](#configuration) with with one element of the list: we update the [configuration](#configuration) with the new 2\*2 square state and multiply its [amplitude](#amplitude) by the complex number associated with the new 2\*2 square state. (see [visual example](#visual-example) step 0-1)

We then check for [interference](#interference) and the [step](#step) is over. (see [visual example](#visual-example) step 4-5)

During the [step](#step) we also compute the [combined state](#combined-state) of the [universe](#universe) that we will be used to visualize the [universe](#universe) in the UI.

#### Visual example
https://user-images.githubusercontent.com/11985913/232820455-15974aac-31ce-46e3-9fa2-cfe942b8c3cb.mp4

### UI

#### Pause button
Button that pauses the qautomata and give access to all other buttons.

#### Reset button
Button that resets the qautomata to the starting state file.

#### Run button
Button that runs the qautomata (unpausing), steps will then be computed automatically.

#### Step button
Button that computes the next [step](#step) of the qautomata.

#### Measure button
Button that applies a [measure](#measure) to the qautomata.  
An automatic [measure](#measure) is applied if there are more than 128 [configurations](#configuration) in the [global state](#global-state) after a [step](#step), we do this to limit the time complexity of the algorithm.

#### Show numbers button
Button to enable/disable the display of the probabilities on the [combined state](#combined-state).

#### Combined state button
Button to display the [combined state](#combined-state).

#### Configurations buttons
Button to display a given [configuration](#configuration).

#### Demo
https://user-images.githubusercontent.com/11985913/232825192-bae9645e-2e0c-4054-8783-51ea2fd232ce.mp4

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
- A [global state](#global-state)
- A set of [rules](#rules)
- The parity of the current [step](#step)
- The number of [step](#step) elapsed since the beginning
- The [combined state](#combined-state)

### Global state
A list of [superposed](https://en.wikipedia.org/wiki/Quantum_superposition) [configurations](#configuration) each associated with an [amplitude](#amplitude).

### Configuration
A grid of [cells](#cell).

### Cell
An element of the grid of the [configuration](#configuration) that can either be dead or alive.

### Amplitude
A complex number associated with a [configuration](#configuration), it can be used to compute the [probability](#configuration-probability) associated with the [configuration](#configuration).

### Configuration probability
Probability of a [configuration](#configuration) to be selected in case of a [measure](#measure). It's the [squared norm](https://en.wikipedia.org/wiki/Norm_(mathematics)) of the [amplitude](#amplitude).

### Measure
Randomly select a [configuration](#configuration) from the [global state](#global-state), set its amplitude to 1 and remove all other [configurations](#configuration). The random selection is made with a [density probability](#configuration-probability) computed with the [amplitudes](#amplitude) of the [configurations](#configuration).

### Rules
A set of rule for the universe, see [Operator matrix](#operator-matrix).

### Operator matrix
A 16\*16 [unitary matrix](https://en.wikipedia.org/wiki/Unitary_matrix) used to compute the [steps](#step) of the [universe](#universe).   

### Step
An instant of the [universe](#universe).

### Interference
When several [configurations](#configuration) have exactly the same alive [cells](#cell), they interfer and merge into one [configuration](#configuration) with their [amplitudes](#amplitude) added.

### Combined state
It containes all [cells](#cell) that are alive in at least one [configuration](#configuration) of the [global state](#global-state), each [cell](#cell) associated with a probability that is eaqual to the sum of the [configuration probabilitiy](#configuration-probability) of all the [configurations](#configuration) in witch the [cell](#cell) is alive.
