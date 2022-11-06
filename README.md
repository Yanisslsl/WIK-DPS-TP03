# Simple Http server

# Getting Started

### Clone the project 

``` 
git clone https://github.com/Yanisslsl/WIK-DPS-TP01.git && cd WIK-DPS-TP01
```

#### - Compile local packages and all of their dependencies.
```
cargo build
```
#### - Set your local variable for server port in .env
```
PORT=3000
```
#### Run the server
``` 
cargo run
```

Hit localhost:[SERVER_PORT]/ping to get your request headers as response.
Any other method or paths will repond a 404 Not Found status.


#### Docker

##### - Build the image
```
docker build -t wik-dps-tp01 .
``` 
##### - Run the container
``` 
docker run -p 3008:3008 wik-dps-tp01
```
#### Build multi-stage docker image
```
docker build -t wik-dps-tp01:multi-stage -f Dockerfile.multi-stage .
```
#### Run multi-stage docker image
```
docker run -p 3008:3008 wik-dps-tp01:multi-stage
```
