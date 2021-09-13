# rustpowfun
rust pow fun 


## stage 1: 

client send InitRequest (just {}) to server

## stage 2:

server send to client base and desired compexity

## stage 3: 

client try to generate nonce, for which   hash((base << 64 ) | nonce)  & ((1 << treshold) - 1) == 0

## stage 4:

server drops connection, if there is no answer for TIMEOUT_MS millis, or check response from client and send quotes (payload)

## stage 5:

client recv payload and print it

## profit
