# Quad Tree
Rust implementation of an in-memory Quad Tree database. The database accept requests via
a gRPC request.

Quad Trees are tree-like data structures in which each node has exactly four children. Often times they
are used to represent 2-dimensional spacial information by recursively subdividing each node into four
quadrants.

For a more detailed explanation of Quad Trees, see here:
https://en.wikipedia.org/wiki/Quadtree

## Protobuf Definitions
The server and client side Protobuf definition can be found in the `proto` folder.


## Server

The Quad Tree server can be found within the `server` directory. The simplest way of running
this is through
```
cargo run server
```
from within the project root.

Once ran, the server will listen on port 50051 for incoming gRPC connections. 

## Client

The CLI client can be found in the `client` directory. The client exposes the following commands

```
Usage: client <COMMAND>

Commands:
  add-point          Adds a new point to the Quad Tree
  delete-point       Deletes a point from the Quad Tree
  find-within-range  Find all points within the specified circular region
  get-all-quads      Returns all Quad Tree nodes
  help               Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```