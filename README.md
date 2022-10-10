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