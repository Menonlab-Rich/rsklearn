import numpy as np
import matplotlib.pyplot as plt
from rsklearn import clustering

# Create some synthetic data for testing
np.random.seed(42)
# Generate two clusters
cluster1 = np.random.randn(100, 2) * 0.4 + np.array([-2, -2])
cluster2 = np.random.randn(100, 2) * 0.5 + np.array([2, 2])
# Add some noise
noise = np.random.uniform(-4, 4, (30, 2))
# Combine all data
X = np.vstack([cluster1, cluster2, noise]).astype(np.float32)

# Create and fit the DBSCAN model
dbscan = clustering.DBScan(eps=0.5, min_samples=5, metric="")
labels = dbscan.fit(X)

print(f"Labels shape: {labels.shape}")
print(f"Unique labels: {np.unique(labels)}")
print(f"Number of clusters found: {len(np.unique(labels[labels >= 0]))}")
print(f"Number of noise points: {np.sum(labels == -1)}")

# Visualize the results
plt.figure(figsize=(10, 8))
# Plot noise points in black
plt.scatter(
    X[labels == -1, 0], X[labels == -1, 1], c="black", marker="x", s=40, label="Noise"
)
# Plot clusters with different colors
unique_labels = np.unique(labels[labels >= 0])
colors = plt.cm.get_cmap("tab20")(np.linspace(0, 1, len(unique_labels)))
for label, color in zip(unique_labels, colors):
    cluster_points = X[labels == label]
    plt.scatter(
        cluster_points[:, 0],
        cluster_points[:, 1],
        c=[color],
        s=30,
        label=f"Cluster {label}",
    )
plt.title("DBSCAN Clustering Results")
plt.legend()
plt.grid(True)
plt.savefig("dbscan_result.png")
plt.show()
