# Dockerfile for a Jupyter Notebook environment with Python data science packages and Rust
# run with docker run -p 8888:8888 -v ./:/home/jovyan/work $(CONTAINER_NAME)

# Start with the official Jupyter base image
FROM jupyter/base-notebook:latest

# Switch to root for package installation
USER root

# Update package lists, upgrade system, and install build tools
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y curl build-essential 
#    apt-get clean && \
#    rm -rf /var/lib/apt/lists/*

# install perf
RUN apt-get install -y linux-tools-common linux-tools-generic linux-tools-$(uname -r)

# Install Python data science packages using mamba (faster conda alternative)
RUN pip install pandas numpy matplotlib

# Switch back to jovyan user
USER ${NB_UID}

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/jovyan/.cargo/bin:${PATH}"



# Verify installations
RUN echo "Python packages:" && \
    python -c "import pandas, numpy, matplotlib; print(f'Pandas {pandas.__version__}, Numpy {numpy.__version__}, Matplotlib {matplotlib.__version__}')" && \
    echo "\nRust version:" && \
    rustc --version