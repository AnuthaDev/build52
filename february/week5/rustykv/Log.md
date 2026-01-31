# In-memory cache implementation

RustyKV is an in-memory cache, that supports `GET` and `SET` commands

## Implementation Plan
- Create a hashmap in memory
    - This hashmap should be accessible to multiple threads and needs to be mutable.
    This is a classic problem that can lead to race consitions. How to make sure
    that this _global_ hashmap stays consistent? The first option I can think of is
    to use a mutex. I will implement that and then look for better ways

    I first tried something like this:
    ```rust
    static mut GLOBAL_MAP: LazyLock<HashMap<String, String>> = LazyLock::new(|| HashMap::new());
    ```

    But Rust really doesn't like mutable statics
    Will need to study how to do this

    So it turns out I don't exactly need a static mut. I can create the HashMap inside the main
    function and then pass it to spawned threads.
- Create a TCP server
    - So, first I need to implement the single threaded version.
    This turned out to be simple enough. The Rust documentation for `TcpListener` was
    enough
- Add support for `GET`, `SET`, `DEL` and `PING` command
    - Here I learned about netcat (`nc`) which can be used to communicate with TCP servers.
    A command via `nc` to the server looks like this:

    ```sh
    echo -n SET hello world | nc localhost 9736
    ```

    Earlier I used direct if-else statements for selecting commands, later moved to a proper command
    parsing framework.
    At this point I had a serial TCP server. I tested by adding a `sleep` call, incoming requests would block
    while the current one is being processed. We need to allow for concurrent processing. My first approach will
    use threads
- Allow concurrent request processing using threads
    - This involved using `thread::spawn` to spawn new threads to handle the request.
    To share the `KVStore` between threads I had to wrap it inside an `Arc` and a `Mutex`. I have some idea about
    a mutex, but `Arc` is totally alien to me. Will read about it.
    So `Arc` allows multiple ownership, while `mutex` ensures single access
- Migrate the server to async IO instead of threads.
    - For this I had to add the `tokio` dependency in `Cargo.toml`, add a `#[tokio::main]` declaration over the main
    function. Use a `RwLock` instead of a `Mutex`, use `tokio::spawn` instead of `thread::spawn` and make some functions
    async while calling `await` inside them.
- Create a CLI that can interact with the server
    - Finally made a simple cli to interact with the server instead of using `nc`