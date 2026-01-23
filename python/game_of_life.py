import copy
import csv
import os
from datetime import datetime

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
    

class GameOfLife:
    def __init__(self, grid):
        self.rows = len(grid)
        self.cols = len(grid[0])

        self._validate_grid(grid)

        self.current_grid = grid
        self.positions = [[1, 0], [0, 1], [-1, 0], [0, -1], [1, 1], [-1, -1], [1, -1], [-1, 1]]

    def _validate_grid(self, grid):
        for i in range(self.rows):
            for j in range(self.cols):
                if grid[i][j] != 0 and grid[i][j] != 1:
                    raise InvalidGridException(i, j)
                
        return
    
    
    def sequential_step(self):
        new_grid = copy.deepcopy(self.current_grid)

        for row in range(0, self.rows):
            for col in range(0, self.cols):
                one_count = 0
                curr_cell = self.current_grid[row][col]
                cell_alive = True if curr_cell == 1 else False

                for pos in self.positions:
                    updated_row = row + pos[0]
                    updated_col = col + pos[1]

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
    
    # save the starting grid to keep track of where the game started
    save_grid(TEST_GRID, res_dir_path, 0)
    
    g = GameOfLife(TEST_GRID)

    for i in range(STEPS):
        save_grid(g.sequential_step(), res_dir_path, i + 1)
    


# initializes the result directory where generation ouput files will be stored
def create_results_dir() -> str:
    timestamp_string = datetime.now().strftime("%Y%m%d_%H%M%S")
    
    dir_path = f'python/results/run_{timestamp_string}'
    try:
        os.makedirs(dir_path, exist_ok=True)
    except OSError as e:
        raise ResultStorageInitializationException(e)

    return dir_path

# saves post-generation matrix state in a new file
def save_grid(grid, result_dir_path, step_no):
    persistence_path = f'{result_dir_path}/{generate_generation_persistance_path(step_no)}'

    try:
        with open(persistence_path, 'w', newline="") as f:
            writer = csv.writer(f)
            writer.writerows(grid)

    except IOError as e:
        print(f"An error has occurred while saving generation output, err: {e}")

# generates file name
def generate_generation_persistance_path(step_no) -> str:
    MAX_STEPS = "10000"
    step_no_str = str(step_no)

    return f'gen_{(len(MAX_STEPS) - len(step_no_str)) * "0"}{step_no_str}.csv'

if __name__ == "__main__":
    main()
