#!/bin/sh -l

# Setup workspace path from arguments
WORKSPACE_PATH=$1

# Install SQLx CLI
cargo install sqlx-cli

# Set DATABASE_URL
export DATABASE_URL=sqlite://$WORKSPACE_PATH/newsletter.sqlite

# Create Database and Run Migrations
cargo sqlx database create
cargo sqlx migrate run

# Build Release
cargo build --release