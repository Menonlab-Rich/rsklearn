<div id="top">

<!-- HEADER STYLE: CLASSIC -->
<div align="center">

<img src="readmeai/assets/logos/purple.svg" width="30%" style="position: relative; top: 0; right: 0;" alt="Project Logo"/>

# RSKLEARN

<em>Supercharge Machine Learning: Rust-Powered Precision at Scale</em>

<!-- BADGES -->
<img src="https://img.shields.io/github/license/Menonlab-Rich/rsklearn?style=default&logo=opensourceinitiative&logoColor=white&color=0080ff" alt="license">
<img src="https://img.shields.io/github/last-commit/Menonlab-Rich/rsklearn?style=default&logo=git&logoColor=white&color=0080ff" alt="last-commit">
<img src="https://img.shields.io/github/languages/top/Menonlab-Rich/rsklearn?style=default&color=0080ff" alt="repo-top-language">
<img src="https://img.shields.io/github/languages/count/Menonlab-Rich/rsklearn?style=default&color=0080ff" alt="repo-language-count">

<!-- default option, no dependency badges. -->


<!-- default option, no dependency badges. -->

</div>
<br>

---

## Table of Contents

- [Table of Contents](#table-of-contents)
- [Overview](#overview)
- [Features](#features)
- [Project Structure](#project-structure)
    - [Project Index](#project-index)
- [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
    - [Usage](#usage)
    - [Testing](#testing)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

---

## Overview

rsklearn is a high-performance Rust implementation of select scikit-learn modules, seamlessly integrated with Python. It combines the speed of Rust with the familiarity of scikit-learn's API.

**Why rsklearn?**

This project accelerates machine learning workflows by leveraging Rust's performance while maintaining Python's ease of use. The core features include:

- **🚀 Rust-powered performance:** Benefit from significantly faster execution times compared to pure Python implementations.
- **🐍 Python integration:** Seamlessly use rsklearn in your existing Python ML pipelines with a familiar sklearn-like API.
- **🧠 DBSCAN clustering:** Efficiently perform density-based clustering on large datasets using the Rust implementation.
- **🧩 Extensible architecture:** Easily add new ML algorithms to the library, expanding its capabilities over time.
- **🔗 Ecosystem compatibility:** Integrate smoothly with popular Python data science libraries like NumPy and Matplotlib.

---

## Features

|      | Component       | Details                              |
| :--- | :-------------- | :----------------------------------- |
| ⚙️  | **Architecture**  | <ul><li>Rust-based machine learning library</li><li>Python bindings using PyO3</li><li>Hybrid Rust/Python structure</li></ul> |
| 🔩 | **Code Quality**  | <ul><li>Uses `cargo` for Rust package management</li><li>Employs `uv` for Python dependency management</li></ul> |
| 📄 | **Documentation** | <ul><li>Limited documentation observed</li><li>Relies on code comments for explanation</li></ul> |
| 🔌 | **Integrations**  | <ul><li>Integration with `numpy` for numerical operations</li><li>Uses `matplotlib` for data visualization</li><li>Leverages `petal-clustering` and `petal-neighbors` libraries</li></ul> |
| 🧩 | **Modularity**    | <ul><li>Separate Rust library (`lib/`) and Python package structure</li><li>Modular design with distinct clustering and neighbor components</li></ul> |
| 🧪 | **Testing**       | <ul><li>No explicit testing framework observed</li><li>Potential for Rust tests using `cargo test`</li></ul> |
| ⚡️  | **Performance**   | <ul><li>Utilizes Rust for performance-critical operations</li><li>Leverages `ndarray` for efficient numerical computations</li></ul> |
| 🛡️ | **Security**      | <ul><li>No explicit security measures observed</li><li>Relies on Rust's memory safety guarantees</li></ul> |
| 📦 | **Dependencies**  | <ul><li>Rust: `ndarray`, `pyo3`, `petal-clustering`, `petal-neighbors`</li><li>Python: `numpy`, `matplotlib`, `toml`</li></ul> |

---

## Project Structure

```sh
└── rsklearn/
    ├── README.md
    ├── dbscan_result.png
    ├── lib
    │   ├── .gitignore
    │   ├── Cargo.lock
    │   ├── Cargo.toml
    │   └── src
    ├── pyproject.toml
    ├── tests
    │   └── simple.py
    └── uv.lock
```

### Project Index

<details open>
	<summary><b><code>RSKLEARN/</code></b></summary>
	<!-- __root__ Submodule -->
	<details>
		<summary><b>__root__</b></summary>
		<blockquote>
			<div class='directory-path' style='padding: 8px 0; color: #666;'>
				<code><b>⦿ __root__</b></code>
			<table style='width: 100%; border-collapse: collapse;'>
			<thead>
				<tr style='background-color: #f8f9fa;'>
					<th style='width: 30%; text-align: left; padding: 8px;'>File Name</th>
					<th style='text-align: left; padding: 8px;'>Summary</th>
				</tr>
			</thead>
				<tr style='border-bottom: 1px solid #eee;'>
					<td style='padding: 8px;'><b><a href='https://github.com/Menonlab-Rich/rsklearn/blob/master/pyproject.toml'>pyproject.toml</a></b></td>
					<td style='padding: 8px;'>- Configures the build system and project metadata for rsklearn, a Rust implementation of select sklearn modules<br>- Specifies dependencies, including maturin for building, matplotlib and numpy for Python integration<br>- Defines project details like name, version, and author information<br>- Sets up development dependencies and Maturin-specific configurations for compiling Rust code into a Python extension module.</td>
				</tr>
			</table>
		</blockquote>
	</details>
	<!-- lib Submodule -->
	<details>
		<summary><b>lib</b></summary>
		<blockquote>
			<div class='directory-path' style='padding: 8px 0; color: #666;'>
				<code><b>⦿ lib</b></code>
			<table style='width: 100%; border-collapse: collapse;'>
			<thead>
				<tr style='background-color: #f8f9fa;'>
					<th style='width: 30%; text-align: left; padding: 8px;'>File Name</th>
					<th style='text-align: left; padding: 8px;'>Summary</th>
				</tr>
			</thead>
				<tr style='border-bottom: 1px solid #eee;'>
					<td style='padding: 8px;'><b><a href='https://github.com/Menonlab-Rich/rsklearn/blob/master/lib/Cargo.toml'>Cargo.toml</a></b></td>
					<td style='padding: 8px;'>- Defines the Rust package configuration for rsklearn, a machine learning library<br>- Specifies the project name, version, and edition, along with the crate type as a dynamic library<br>- Declares dependencies on essential numerical computing and machine learning crates, as well as PyO3 for Python integration<br>- Sets up the foundation for building a Rust-based scikit-learn alternative with Python bindings.</td>
				</tr>
			</table>
			<!-- src Submodule -->
			<details>
				<summary><b>src</b></summary>
				<blockquote>
					<div class='directory-path' style='padding: 8px 0; color: #666;'>
						<code><b>⦿ lib.src</b></code>
					<table style='width: 100%; border-collapse: collapse;'>
					<thead>
						<tr style='background-color: #f8f9fa;'>
							<th style='width: 30%; text-align: left; padding: 8px;'>File Name</th>
							<th style='text-align: left; padding: 8px;'>Summary</th>
						</tr>
					</thead>
						<tr style='border-bottom: 1px solid #eee;'>
							<td style='padding: 8px;'><b><a href='https://github.com/Menonlab-Rich/rsklearn/blob/master/lib/src/lib.rs'>lib.rs</a></b></td>
							<td style='padding: 8px;'>- Serves as the entry point for the rsklearn Rust library, defining the Python module structure<br>- Initializes and exposes the clustering submodule, enabling seamless integration of Rust-implemented clustering algorithms into Python environments<br>- Facilitates the extension of scikit-learns functionality with high-performance Rust implementations, enhancing the librarys capabilities for machine learning tasks, particularly in clustering operations.</td>
						</tr>
					</table>
					<!-- clustering Submodule -->
					<details>
						<summary><b>clustering</b></summary>
						<blockquote>
							<div class='directory-path' style='padding: 8px 0; color: #666;'>
								<code><b>⦿ lib.src.clustering</b></code>
							<table style='width: 100%; border-collapse: collapse;'>
							<thead>
								<tr style='background-color: #f8f9fa;'>
									<th style='width: 30%; text-align: left; padding: 8px;'>File Name</th>
									<th style='text-align: left; padding: 8px;'>Summary</th>
								</tr>
							</thead>
								<tr style='border-bottom: 1px solid #eee;'>
									<td style='padding: 8px;'><b><a href='https://github.com/Menonlab-Rich/rsklearn/blob/master/lib/src/clustering/mod.rs'>mod.rs</a></b></td>
									<td style='padding: 8px;'>- Initializes the clustering module within the projects Rust implementation<br>- Exposes the DBSCAN (Density-Based Spatial Clustering of Applications with Noise) algorithm to Python through PyO3 bindings<br>- Acts as a bridge between Rust and Python, allowing the DBSCAN functionality to be accessed and utilized in Python code<br>- Facilitates integration of efficient Rust-based clustering capabilities into the broader Python-centric project structure.</td>
								</tr>
								<tr style='border-bottom: 1px solid #eee;'>
									<td style='padding: 8px;'><b><a href='https://github.com/Menonlab-Rich/rsklearn/blob/master/lib/src/clustering/dbscan.rs'>dbscan.rs</a></b></td>
									<td style='padding: 8px;'>- Implements the DBSCAN clustering algorithm in Rust for Python integration<br>- Defines a DBScan struct with configurable parameters and a fit method to perform clustering on input data<br>- Utilizes the petal-clustering and petal-neighbors crates for core functionality<br>- Enables users to create DBSCAN instances, set parameters, and apply the algorithm to datasets, returning cluster labels as a NumPy array.</td>
								</tr>
							</table>
						</blockquote>
					</details>
				</blockquote>
			</details>
		</blockquote>
	</details>
</details>

---

## Getting Started

### Prerequisites

This project requires the following dependencies:

- **Programming Language:** Rust
- **Package Manager:** Uv, Cargo

### Installation

Build rsklearn from the source and intsall dependencies:

1. **Clone the repository:**

    ```sh
    ❯ git clone https://github.com/Menonlab-Rich/rsklearn
    ```

2. **Navigate to the project directory:**

    ```sh
    ❯ cd rsklearn
    ```

3. **Install the dependencies:**

<!-- SHIELDS BADGE CURRENTLY DISABLED -->
	<!-- [![uv][uv-shield]][uv-link] -->
	<!-- REFERENCE LINKS -->
	<!-- [uv-shield]: None -->
	<!-- [uv-link]: None -->

	**Using [uv](None):**

	```sh
	❯ echo 'INSERT-INSTALL-COMMAND-HERE'
	```
<!-- SHIELDS BADGE CURRENTLY DISABLED -->
	<!-- [![cargo][cargo-shield]][cargo-link] -->
	<!-- REFERENCE LINKS -->
	<!-- [cargo-shield]: https://img.shields.io/badge/Rust-000000.svg?style={badge_style}&logo=rust&logoColor=white -->
	<!-- [cargo-link]: https://www.rust-lang.org/ -->

	**Using [cargo](https://www.rust-lang.org/):**

	```sh
	❯ cargo build
	```

### Usage

Run the project with:

**Using [uv](None):**
```sh
echo 'INSERT-RUN-COMMAND-HERE'
```
**Using [cargo](https://www.rust-lang.org/):**
```sh
cargo run
```

### Testing

Rsklearn uses the {__test_framework__} test framework. Run the test suite with:

**Using [uv](None):**
```sh
echo 'INSERT-TEST-COMMAND-HERE'
```
**Using [cargo](https://www.rust-lang.org/):**
```sh
cargo test
```

---

## Roadmap

- [X] **`Task 1`**: <strike>Implement feature one.</strike>
- [ ] **`Task 2`**: Implement feature two.
- [ ] **`Task 3`**: Implement feature three.

---

## Contributing

- **💬 [Join the Discussions](https://github.com/Menonlab-Rich/rsklearn/discussions)**: Share your insights, provide feedback, or ask questions.
- **🐛 [Report Issues](https://github.com/Menonlab-Rich/rsklearn/issues)**: Submit bugs found or log feature requests for the `rsklearn` project.
- **💡 [Submit Pull Requests](https://github.com/Menonlab-Rich/rsklearn/blob/main/CONTRIBUTING.md)**: Review open PRs, and submit your own PRs.

<details closed>
<summary>Contributing Guidelines</summary>

1. **Fork the Repository**: Start by forking the project repository to your github account.
2. **Clone Locally**: Clone the forked repository to your local machine using a git client.
   ```sh
   git clone https://github.com/Menonlab-Rich/rsklearn
   ```
3. **Create a New Branch**: Always work on a new branch, giving it a descriptive name.
   ```sh
   git checkout -b new-feature-x
   ```
4. **Make Your Changes**: Develop and test your changes locally.
5. **Commit Your Changes**: Commit with a clear message describing your updates.
   ```sh
   git commit -m 'Implemented new feature x.'
   ```
6. **Push to github**: Push the changes to your forked repository.
   ```sh
   git push origin new-feature-x
   ```
7. **Submit a Pull Request**: Create a PR against the original project repository. Clearly describe the changes and their motivations.
8. **Review**: Once your PR is reviewed and approved, it will be merged into the main branch. Congratulations on your contribution!
</details>

<details closed>
<summary>Contributor Graph</summary>
<br>
<p align="left">
   <a href="https://github.com{/Menonlab-Rich/rsklearn/}graphs/contributors">
      <img src="https://contrib.rocks/image?repo=Menonlab-Rich/rsklearn">
   </a>
</p>
</details>

---

## License

Rsklearn is protected under the [LICENSE](https://choosealicense.com/licenses) License. For more details, refer to the [LICENSE](https://choosealicense.com/licenses/) file.

---

## Acknowledgments

- Credit `contributors`, `inspiration`, `references`, etc.

<div align="right">

[![][back-to-top]](#top)

</div>


[back-to-top]: https://img.shields.io/badge/-BACK_TO_TOP-151515?style=flat-square


---
