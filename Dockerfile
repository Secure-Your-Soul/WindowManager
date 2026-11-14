FROM rust:latest

RUN apt-get update && apt-get install -y \
    pkg-config \
    libudev-dev \
    libinput-dev \
    libxkbcommon-dev \
    libwayland-dev \
    libgl1-mesa-dev \
    libegl1-mesa-dev \
    libgbm-dev \
    libseat-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
CMD ["cargo", "run"]