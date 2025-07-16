# docker build --platform linux/amd64 -t rust-app-image .

# Step 2 - build step
docker run -it --rm -v .:/app rust-app-image cargo build --release

