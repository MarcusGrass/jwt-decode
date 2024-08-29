# Jwt decode
A tiny cli for decoding `JWTs` (specifically the [JWS kind](https://datatracker.ietf.org/doc/html/rfc7515)).  

## Why?

I have found myself having to look to see what's inside a `JWT` many times, 
I do think [jwt.io](https://jwt.io/) is trustworthy, but I'm too paranoid to use it.

I just want a CLI where I can examine my `JWTs` locally, that's why I made this.

You could also check [stackoverflow](https://stackoverflow.com/questions/75776014/cant-correctly-decode-jwt-payload-using-base64-d), 
and really, I could have just aliased that monster of an expression, but I thought this 
was a bit more fun.  

## Features

Decode a jwt as an arg showing relative time
```bash
jwt-decode eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNzI0OTUyOTQwLCJuYmYiOjE3MjQ5NTI5NTAsImV4cCI6MTcyNDk5Mjk0MH0.ZrQSkD_tzm7yyTC0Cw3_qmBRZhi0QCirL4hfICAWNr0 -v -r
# Outputs:
# Header {
#   "alg": "HS256",
#   "typ": "JWT"
# }
# Payload {
#   "exp": "1724992940 [2024-08-30 4:42:20.0 +00:00:00 (In 39843 seconds)]",
#   "iat": "1724952940 [2024-08-29 17:35:40.0 +00:00:00 (157 seconds ago)]",
#   "name": "John Doe",
#   "nbf": "1724952950 [2024-08-29 17:35:50.0 +00:00:00 (147 seconds ago)]",
#   "sub": "1234567890"
# }

```

Or just as an arg without time

```bash
jwt-decode -v eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI3Nzc3NyIsIm5hbWUiOiJNeSB0ZXN0IG5hbWUiLCJpYXQiOjE1MTYyMzkwMzl9.r0YSbk-Gjr4gWATqbDnirs102IUBQRru-_TNu5AtE18
# Outputs:
# Header {
#   "alg": "HS256",
#   "typ": "JWT"
# }
# Payload {
#   "iat": 1516239039,
#   "name": "My test name",
#   "sub": "77777"
# }
```

Or piped
```bash
echo eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI3Nzc3NyIsIm5hbWUiOiJNeSB0ZXN0IG5hbWUiLCJpYXQiOjE1MTYyMzkwMzl9.r0YSbk-Gjr4gWATqbDnirs102IUBQRru-_TNu5AtE18 | jwt-decode -v
# Same output as above
```

Or from a file

```bash
jwt-decode -v -p ./.local/data/test.jwt
# Same output as above
```

Output only the header

```bash
jwt-decode -p ./.local/data/test.jwt -o header
# Output: 
# {
#   "alg": "HS256",
#   "typ": "JWT"
# }
```
Or the payload 

```bash
jwt-decode -p ./.local/data/test.jwt -o header
# Output: 
# {
#   "iat": 1516239039,
#   "name": "My test name",
#   "sub": "77777"
# }
```

## License
Licensed under [GPL-3.0](./LICENSE)