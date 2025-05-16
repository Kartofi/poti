## Installation

```bash
# Clone the git repo
git clone https://github.com/Kartofi/poti
cd ./poti/poti

## Build the binary
cargo build --release

# Build the docker image
docker build -t poti-server .

# Run the image
docker run -v /path/to/config:/config -v /path/to/backup:/backup -p 3000:3000 poti-server

```

Then in the docker console you will see: "Your secret is...".<br>
If you dont see it open your config folder adn then settings.poti and copy the secret from there.
