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

# running


### server
args: addr:port
''' 
$cargo run --bin helloserver 0.0.0.0:4322
'''

### client

args: addr:port retries 
''' 
$ cargo run --bin helloclient 127.0.0.1:4322 20
   Compiling powddos v0.1.0 (/home/igor/devel/job/ddospow/powddos)
    Finished dev [unoptimized + debuginfo] target(s) in 0.80s
     Running `target/debug/helloclient '127.0.0.1:4322' 20`
found good nonce for 517 millis
payload:
Fighting for peace is like screwing for virginity
    badfish

found good nonce for 1395 millis
err with request too long :(
found good nonce for 217 millis
payload:
Many have marked the speed with which Muad'Dib learned the necessities of
Arrakis. The Bene Gesserit, of course, know the basis of this speed. For the
others, we can say that Muad'Dib learned rapidly because his first training was
in how to learn. And the first lesson of all was the basic trust that he could
learn. It's shocking to find how many people do not believe they can learn, and
how many more believe learning to be difficult. Muad'Dib knew that every
experience carries its lesson.
    from "The Humanity of Muad'Dib" by the Princess Irulan

found good nonce for 129 millis
payload:
If it ain't broke, don't fix it.
    Ronald Reagan, Santa Barbara, California

found good nonce for 597 millis
payload:
Doubt is not a pleasant condition, but certainty is absurd
    Voltaire

found good nonce for 354 millis
payload:
Age is only a number, a cipher for the records. A man can't retire his
experience. He must use it. Experience achieves more with less energy and time.
    Bernard Baruch


found good nonce for 1448 millis
err with request too long :(
found good nonce for 1570 millis
err with request too long :(
found good nonce for 632 millis
payload:
Arrakis teaches the attitude of the knife - chopping off what's incomplete and
saying: "Now it's complete because it's ended here."
    Muad'dib, "Dune"

found good nonce for 411 millis
payload:
Sinclair : Ready?
Delenn : Why is it that your people always ask someone if they are ready right
before you are about to do something massively unwise?
Sinclair : Tradition.
    Babylon 5

found good nonce for 524 millis
payload:
ATTORNEY: This myasthenia gravis, does it affect your memory at all?
WITNESS: Yes.
ATTORNEY: And in what ways does it affect your memory?
WITNESS: I forget.
ATTORNEY: You forget? Can you give us an example of something you forgot?
    Seen at, http://www.moronland.com/moronia/moron/919/

found good nonce for 77 millis
payload:
They (the British) are like their own beer; froth on top, dregs at bottom, the
middle excellent
    Voltaire

found good nonce for 104 millis
payload:
If the grass is greener on the other side, you can bet the water bill is higher

found good nonce for 85 millis
payload:
Build your own dreams, or someone else will hire you to build theirs.
    Farrah Gray

found good nonce for 476 millis
payload:
The disadvantage of working over networks is that you can't so easily go into
someone else's office and rip their bloody heart out
    Jim McDonald

found good nonce for 1254 millis
err with request too long :(
found good nonce for 834 millis
payload:
As men are not able to fight against death, misery, ignorance, they have taken
it into their heads, in order to be happy, not to think of them at all.
    Blaise Pascal

found good nonce for 1371 millis
err with request too long :(
found good nonce for 401 millis
payload:
"If I painted my turtle black, would it be spooky?"
    Jhonen Vasquez, Johnny the Homicidal Maniac


'''
