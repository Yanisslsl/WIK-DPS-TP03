# Simple Http server

# Getting Started

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