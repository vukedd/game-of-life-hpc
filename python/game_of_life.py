import numpy as np
import csv
import os
from datetime import datetime


class GameOfLife:
    def __init__(self, grid):
        self.rows = len(grid)
        self.cols = len(grid[0])

        self.current_grid = np.array(self._validate_grid(grid), dtype=np.int8)
        self.positions = [[1, 0], [0, 1], [-1, 0], [0, -1], [1, 1], [-1, -1], [1, -1], [-1, 1]]

    def _validate_grid(self, grid):
        for i in range(self.rows):
            for j in range(self.cols):
                if grid[i][j] != 0 and grid[i][j] != 1:
                    raise InvalidGridException(i, j)
                
        return grid
    
    # sequential implementation
    def sequential_step(self):
        # instead of doing a deep copy of the current grid, we just need an empty grid of the same size which
        # can be initialized
        new_grid = [[0] * self.cols for _ in range(self.rows)]

        for row in range(0, self.rows):
            for col in range(0, self.cols):
                one_count = 0
                curr_cell = self.current_grid[row][col]
                cell_alive = True if curr_cell == 1 else False

                for px, py in self.positions:
                    updated_row, updated_col = row + px, col + py

                    if (updated_row >= 0 and updated_row < self.rows) and (updated_col >= 0 and updated_col < self.cols):    
                        if self.current_grid[updated_row][updated_col] == 1:
                            one_count += 1
                

                if (cell_alive and one_count == 2) or one_count == 3:
                    new_grid[row][col] = 1
                else:
                    new_grid[row][col] = 0
        
        self.current_grid = new_grid
        return self.current_grid


    def parallel_step(self):
        # TODO
        pass


def main():
    # initialize new result directory
    res_dir_path = create_results_dir()
    if res_dir_path == "":
        raise Exception

    # define the number of steps we want to execute
    STEPS = 10000

    TEST_GRID = [[0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 1, 1, 1, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0]]
    
    g = GameOfLife(TEST_GRID)

    # save the starting grid to keep track of where the game started
    save_grid(TEST_GRID, res_dir_path, 0)

    for i in range(STEPS):
        save_grid(g.sequential_step(), res_dir_path, i + 1)
    


# initializes the result directory where generation ouput files will be stored
def create_results_dir() -> str:
    timestamp_string = datetime.now().strftime("%Y%m%d_%H%M%S")
    
    dir_path = f'python/results/run_{timestamp_string}'
    try:
        # will try to create a directory on the given path, and if it, by any chance,
        # already exist don't throw an exception
        os.makedirs(dir_path, exist_ok=True)
    except OSError as e:
        raise ResultStorageInitializationException(e)

    return dir_path

# saves post-generation matrix state in a new file
def save_grid(grid, result_dir_path, step_no):
    persistence_path = f'{result_dir_path}/{generate_file_name(step_no)}'

    try:
        with open(persistence_path, 'w', newline="") as f:
            writer = csv.writer(f)
            writer.writerows(grid)

    except IOError as e:
        print(f"An error has occurred while saving generation output, err: {e}")

# generates file name
def generate_file_name(step_no) -> str:
    # based on the max steps variable the number of leading 0's will be determined when creating
    # a generation file 
    MAX_STEPS = "10000"
    step_no_str = str(step_no)

    return f'gen_{(len(MAX_STEPS) - len(step_no_str)) * "0"}{step_no_str}.csv'

class InvalidGridException(Exception):
    def __init__(self, x, y):
        self.message = f'The board contains values other than 1 or 0. Cell location row: {x}, column: {y} (0-indexed)'
        super().__init__(self.message)

    def __str__(self):
        return self.message

class ResultStorageInitializationException(Exception):
    def __init__(self, err):
        self.message = f'An error has ocurred while initializing result storage directory, err: {err}'
        super().__init__(self.message)

    def __str__(self):
        return self.message
    
if __name__ == "__main__":
    main()
