import numpy as np
import csv
import os
from datetime import datetime
from numba import njit, prange, set_num_threads
import time


'''
    a function annotated with njit will be compiled by the numba JIT and will
    fully avoid the python interpreter
    
    by setting parallel to true we allow numba to use OS level threads to process 
    tasks in parallel with features like prange
'''
@njit(parallel=True)
def update_grid_numba(current_grid, new_grid):
    rows, cols = current_grid.shape
    
    '''
        prange represents a numba feature which provides parallel loop processing

        OS-level threads compute chunks of the grid in parallel
    '''
    for row in prange(rows):
        for col in range(cols):
            one_count = 0
            curr_cell = current_grid[row][col]
            cell_alive = True if curr_cell == 1 else False

            for i in range(-1, 2):
                for j in range(-1, 2):
                    if i == 0 and j == 0:
                        continue
                    
                    r = row + i
                    c = col + j
                    
                    if (r >= 0 and r < rows) and (c >= 0 and c < cols):
                        if current_grid[r, c] == 1:
                            one_count += 1
            
            if (cell_alive and one_count == 2) or one_count == 3:
                new_grid[row, col] = 1
            else:
                new_grid[row, col] = 0

class GameOfLife:
    def __init__(self, grid):
        '''
            numpy arrays are better performance-wise for our particular problem because of benefits such as:
                - they are homogeneous, so we avoid type-checking overhead,
                and can compare data very fast
                - since they are stored in memory as a contiguous sequence
                it is much easier to divide the structure into batches which 
                will later be processed in-parallel, when you combine this
                with the fact that the array elements are being stored as 
                concrete values Cache Locality (Continugous data blocks 
                predicted by the CPU are being saved in cache, this won't work
                when it comes to classic python lists because they are a list 
                of references pointing to random locations on RAM which makes it
                harder for the CPU to predict) becomes enabled which allows 
                much faster reads.
        '''
        self.current_grid = np.array(grid, dtype=np.int8)
    
    '''
        sequential implementation
    '''
    def sequential_step(self):
        new_grid = np.zeros_like(self.current_grid)
        rows, cols = new_grid.shape

        for row in range(0, rows):
            for col in range(0, cols):
                one_count = 0
                cell_alive = self.current_grid[row, col] == 1 

                for px in range(-1, 2):
                    for py in range(-1, 2):
                        if px == py == 0:
                            continue

                        updated_row, updated_col = row + px, col + py

                        if (0 <= updated_row < rows) and (0 <= updated_col < cols):    
                            if self.current_grid[updated_row, updated_col] == 1:
                                one_count += 1
                    
                    if (cell_alive and one_count == 2) or one_count == 3:
                        new_grid[row, col] = 1
                    else:
                        new_grid[row, col] = 0
        
        self.current_grid = new_grid
        return self.current_grid

    '''
        parallel implementation using numba
    '''
    def parallel_step_numba(self):
        new_grid = np.zeros_like(self.current_grid)
        
        update_grid_numba(self.current_grid, new_grid)
        
        self.current_grid = new_grid
        return self.current_grid


def main():
    ''' 
        define the number of steps we want to execute 
    '''
    STEPS = 100
    GRID_SIZE = 100
    TEST_GRID = np.random.randint(0, 2, size=(GRID_SIZE, GRID_SIZE), dtype=np.int8)
    
    ''' 
        initialize new result directory 
    '''
    # res_dir_path = create_results_dir()
    # if res_dir_path == "":
    #     raise Exception
    
    '''
        save the starting grid to keep track of where the game started
    '''
    # save_grid(TEST_GRID, res_dir_path, 0)
    
    '''
        sequential test
    '''
    g1 = GameOfLife(TEST_GRID)

    start_time = time.perf_counter()
    for _ in range(STEPS):
       g1.sequential_step()

    end_time = time.perf_counter()
    seq_time = end_time - start_time


    '''
        parallel test
    '''
    g2 = GameOfLife(TEST_GRID)
    
    set_num_threads(2)
    '''
        numba takes time to start up so we do that beforehand to get maximum performance
    '''
    g2.parallel_step_numba()

    start_time = time.perf_counter()
    for _ in range(STEPS):
       g2.parallel_step_numba()

    end_time = time.perf_counter()
    par_time = end_time - start_time


    '''
        show execution time difference
    '''
    print("-" * 30)
    print(f"Sequential: {seq_time:.4f}s | Parallel: {par_time:.4f}s")
    print("-" * 30)


'''
    initializes the result directory where generation ouput files will be stored
'''
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

'''
    saves post-generation matrix state in a new file
'''
def save_grid(grid, result_dir_path, step_no):
    persistence_path = f'{result_dir_path}/{generate_file_name(step_no)}'

    try:
        with open(persistence_path, 'w', newline="") as f:
            writer = csv.writer(f)
            writer.writerows(grid)

    except IOError as e:
        print(f"An error has occurred while saving generation output, err: {e}")

'''
    generates file name
'''
def generate_file_name(step_no) -> str:
    # based on the max steps variable the number of leading 0's will be determined when creating
    # a generation file 
    MAX_STEPS = "10000"
    step_no_str = str(step_no)

    return f'gen_{(len(MAX_STEPS) - len(step_no_str)) * "0"}{step_no_str}.csv'

class ResultStorageInitializationException(Exception):
    def __init__(self, err):
        self.message = f'An error has ocurred while initializing result storage directory, err: {err}'
        super().__init__(self.message)

    def __str__(self):
        return self.message
    
if __name__ == "__main__":
    main()
