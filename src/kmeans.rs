//! K-Means Clustering.

use rand::{Rng, thread_rng};

use corr;
use csr::{Csr, Row};

pub struct Model {
    row_count: usize,
    column_count: usize,
    cluster_count: usize,
    centroids: Csr,
    row_clusters: Vec<Option<usize>>,
}

impl Model {
    /// Creates a new model.
    pub fn new(row_count: usize, column_count: usize, cluster_count: usize) -> Self {
        let mut centroids = Csr::new();
        let mut rng = thread_rng();

        for _ in 0..cluster_count {
            centroids.start();
            for column_index in 0..column_count {
                centroids.next(column_index, rng.gen_range(0.0, 100.0));
            }
        }
        centroids.start();

        Model {
            row_count: row_count,
            column_count: column_count,
            cluster_count: cluster_count,
            centroids: centroids,
            row_clusters: vec![None; row_count],
        }
    }

    /// Gets cluster count.
    pub fn cluster_count(&self) -> usize {
        self.cluster_count
    }

    /// Gets row cluster index.
    pub fn get_cluster(&self, row_index: usize) -> Option<usize> {
        self.row_clusters[row_index]
    }

    /// Gets cluster centroid.
    pub fn get_centroid(&self, index: usize) -> Row {
        self.centroids.get_row(index)
    }

    /// Makes clustering step.
    pub fn make_step(&mut self, matrix: &Csr) -> usize {
        let mut changed_count = 0;
        // Assign nearest centroids.
        for row_index in 0..self.row_count {
            let row = matrix.get_row(row_index);
            if row.len() < 3 {
                // FIX: this row correlates to any other one so it should be skipped.
                continue;
            }
            let cluster_index = Some(self.get_nearest_centroid(row));
            if cluster_index != self.row_clusters[row_index] {
                changed_count += 1;
            }
            self.row_clusters[row_index] = cluster_index;
        }
        // Reset centroids.
        for cluster_index in 0..self.cluster_count {
            let row = self.centroids.get_mutable_row(cluster_index);
            for value in row.iter_mut() {
                value.value = 0.0;
            }
        }
        // Sum up values.
        let mut value_count = vec![0usize; self.cluster_count * self.column_count];
        for row_index in 0..self.row_count {
            if let Some(cluster_index) = self.row_clusters[row_index] {
                for value in matrix.get_row(row_index) {
                    // Increase column value count.
                    value_count[cluster_index * self.column_count + value.column] += 1;
                    // Increase centroid value.
                    self.centroids.get_mutable_row(cluster_index)[value.column].value += value.value;
                }
            }
        }
        // Divide by value count.
        for cluster_index in 0..self.cluster_count {
            for value in self.centroids.get_mutable_row(cluster_index) {
                value.value /= value_count[cluster_index * self.column_count + value.column] as f64;
            }
        }

        changed_count
    }

    /// Gets the nearest centroid by the given row.
    fn get_nearest_centroid(&self, row: Row) -> usize {
        use std::f64;

        let mut min_distance = f64::INFINITY;
        let mut cluster_index = 0;

        for i in 0..self.cluster_count {
            let distance = 1.0 - corr::pearson(row, self.centroids.get_row(i));
            if distance < min_distance {
                min_distance = distance;
                cluster_index = i;
            }
        }

        cluster_index
    }
}

#[test]
fn test_random() {
    use std::f64;

    let mut rng = thread_rng();
    let mut matrix = Csr::new();
    for _ in 0..10 {
        matrix.start();
        for column_index in 0..10 {
            matrix.next(column_index, if !rng.gen_weighted_bool(5) { rng.gen_range(0.0, 100.0) } else { f64::NAN });
        }
    }
    matrix.start();

    let mut model = Model::new(10, 10, 15);
    for step in 0.. {
        if model.make_step(&matrix) == 0 {
            break;
        }
        assert!(step < 10);
    }
}
