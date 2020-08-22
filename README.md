
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

## How to run the server locally

The local server uses Warp, which can be started like so:

    cargo run warp_server

The server will start at port 7878.

    curl http://localhost:7878/node/a

    {"children":["e","c"],"reward":100}

## Deploying to AWS Lambda

The API is currently deployed at `graph.modulitos.com`

For example:

    curl https://graph.modulitos.com/node/a

    {"children":["e","c"],"reward":100}

### To create a package for deployment to AWS Lambda

    ./deploy.sh

Then upload the resulting `rust.zip` file to the Lambda function

### some notes while setting this up on AWS Lambda:

Followed steps here for basic Lambda setup: https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/

And here for setting up REST API Gateway:
https://kennbrodhagen.net/2015/12/06/how-to-create-a-request-object-for-your-lambda-event-from-api-gateway/

And here for setting up API Gateway with path params:
https://docs.aws.amazon.com/apigateway/latest/developerguide/integrating-api-with-aws-services-lambda.html#api-as-lambda-proxy-expose-get-method-with-path-parameters-to-call-lambda-function
