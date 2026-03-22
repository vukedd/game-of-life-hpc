# Conway's game of life
- Conway's game of life is a cellular automaton invented by a British mathematician John Horton Conway in 1970. It is a zero player game which means it's evolution is determined by the initial state, requiring no input after the game starts. The only interaction users get is selecting cells before the evolution starts, where the evolution itself is actually an algorithm.


## Rules
- The game of life is happening on a 2D grid where every cell can be in two possible states 0 (dead), 1 (alive). Every cell interacts with its eight neighbours, which are the cells that are horizontally, vertically, diagonally adjacent. For every step the following transition occurs:
    - Every live cell with fewer than two live neighbours dies because of underpopulation
    - Every live cell with two or three live neighbours will live on to the next generation (next step)
    - Every live cell with more than three live neighbours dies because of overpopulation
    - Every dead cell with exactly three live neighbours becomes a live cell, as if by reproduction


## Implementation,
<h3> Python/Rust implementation:</h3>
<ul>
    <li> Sequential version:
        <ul>
            <li>
            Implements a sequential solution in python which will generate at least one file that records the state of the grid for each iteration
            </li>
        </ul>
    </li>
    <li> Parallel version:
        <ul>
            <li>
            Implements a parallel version in Python which will generate at least one file that records the state of the grid for each iteration
            </li>
        </ul>
    </li>
</ul>

<h3>Rust/Python performance experiments:</h3>
<ul>
    <li>
        Performed <strong>strong</strong> (Measures the changes in the execution time when the problem size stays fixed but the number of processors increases) and <strong>weak</strong> scaling (Measures how the execution time changes when the problem size increases proportionally with the number of processors) experiments to measure speedup of the parallel version compared to the sequential version for both rust and python implementations
    </li>
</ul>

<h3>Visualization:</h3>
<ul>
    <li>
        Visualized the evolution by using the output files generated using Rust's <i>Plotter</i> library
    </li>
</ul>
