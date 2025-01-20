import numpy as np
from numba import jit

@jit(nopython=True)
def optimize_point_cloud(points):
    # Remove duplicate points
    unique_points = np.unique(points, axis=0)
    
    # Remove outliers (simple distance-based filtering)
    mean = np.mean(unique_points, axis=0)
    distances = np.sqrt(np.sum((unique_points - mean) ** 2, axis=1))
    threshold = np.percentile(distances, 95)
    filtered_points = unique_points[distances < threshold]
    
    return filtered_points
