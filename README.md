
This is the api to support the concurrency exercise here:
https://github.com/modulitos/async-graph-exercise

# The API

This API is an HTTP server that returns information about the nodes in a graph, with various delays in the response times for each node. It only serves GET requests.

Each node has a `reward` and `children`.

For example:

    GET http://localhost:7878/node/a

returns JSON of the form:

    {
       "children": [ "c", "e" ],
       "reward": 100
    }

Where the children in this example can be accessed via:
    GET http://localhost:7878/node/c
    GET http://localhost:7878/node/e

# How to run the server

    cargo run

The server will start at port 7878.

    curl http://localhost:7878/node/a

    {"children":["e","c"],"reward":100}

