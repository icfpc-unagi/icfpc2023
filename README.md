# Team Unagi

Team Unagi's repository for ICFPC 2023

## Members

- Takuya Akiba
- Kentaro Imajo
- Hiroaki Iwami
- Yoichi Iwata
- Toshiki Kataoka
- Naohiro Takahashi


## Programming Language

We thoroughly used **Rust** for writing solvers, visualizers, a web server, and tools.


## Approach

### Step 1: Initial Solutions
In this problem, it was necessary to determine the placement of each musician. However, determining this directly can be challenging, so we divided it into two phases:

First, we decide only the location for placement, without specifying which musician.
Then, we assign each musician to the candidate locations.
The second phase is simple. Once the placement of the musicians is decided, the problem can be reduced to an assignment problem. We solved this using minimum-cost flow. The added feature that the effect is amplified when the same type of musicians are nearby could not be solved as an assignment problem, but we resolved this by determining a provisional placement, solving the assignment problem, and repeating this process.

The first phase is not easy to solve. However, considering the significant influence of the audience near the stage, we tried multiple patterns of arrangements to perform as close as possible to the audience near the stage, and expanded the rest accordingly. The simplest arrangement is to place one musician directly in a nearby area, and we also tried patterns to place musicians in configurations where the sound reaches from 3, 5, or 7 people. We constructed the overall solution by combining these patterns.

With this solution, we were able to further improve the solution by performing hill-climbing, destroying the arrangement of some musicians, and reconstructing it with a different pattern.

See `src/bin/chokudai.rs` for details.


### Step 2: Simulated Annealing
We enumerated certain candidate positions for the musicians and solved it as a discrete optimization problem instead of a continuous one. By pre-enumerating the candidates, it is possible to pre-calculate which pairs are obstructed by each position. This allows for fast calculation of score differences when changing a musician's playing position. Optimization was performed using a simulated annealing approach, considering changes and swaps in positions as the neighborhood.

The process involved finding candidate positions, applying simulated annealing, finding promising positions around the current playing positions and adding them to the candidates, and then applying simulated annealing again. This cycle was repeated.

Moreover, we added the best solution we have found so far to the initial candidates, enabling optimization from that point onward.

See `src/bin/wata2.rs` for details.


### Step 3: Refinement
We have developed several algorithms to improve the solution further and to escape from the local optimum using transitions that are not utilized in simulated annealing. One is to find the gradient of the musicians' positions with respect to the score and move the musicians in that direction. The second is to move in a random direction. The third is a greedy hillclimb that broadens the candidate positions of simulated annealing. We alternately applied these to try to improve the solution. Then, we applied the improved solution back to the simulated annealing in step 2.

See `src/bin/hillclimb_mix.rs` for details.




## Infrastructure

### Tools
We managed the import and submission of problem and submission data through the official API, and created a visualizer that produces SVG images to visually confirm the status and statistical information of the problem and solution. These were implemented as shared libraries, which can be used with command-line tools and private web servers, and can be compiled into WASM for interactive manipulation on a browser-based visualizer.


### Scoring
Regarding the determination of "blocked", we also performed precise calculations with BigRational, as there was a possibility of different errors depending on the choice of geometric algorithms with float64 calculations. As a result, we found that if it is parallel to the x-axis or y-axis, specializing it this way does not cause problems with errors even in float64, so we implemented it in this manner.

See `src/scoring.rs` for details.




### Private Servers
We built a database and web service on the GCP (Google Cloud Platform) environment to internally calculate and save the scores of solutions created within the team, without relying on the official API. This was done to review them.

We regularly synchronized with the submission status, easily checked the overall situation from the web, obtained the current best solution, and managed metadata and statistical information. We utilized this setup for various developments and debugging, as well as for tooling and automation.



### Computational infrastructure

Our solution for this year does not require a large number of runs.
We did not proceed with automation that required hundreds of instances.
Thanks to Rust, we safely parallelized our solution.
Our program runs faster with more CPUs, so we worked on machines with 224 vCPUs.


### Services

We used Discord, Github Copilot, ChatGPT, Google Workspace, GitHub, and Pipedream.
