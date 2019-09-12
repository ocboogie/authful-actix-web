# authful-actix-web
This is a simple example of how to use [Actix Web](https://github.com/actix/actix-web) with decoupled business logic and user authentication. This project is very similar to the one described in this [blog post](https://gill.net.in/posts/auth-microservice-rust-actix-web-diesel-complete-tutorial-part-1/), but with some key differences.

This project is split into two crates, `core`, and `http_transport`.
 * `core` contains all the business logic, and is what interacts with the database. This is done to be protocol agnostic.
 * `http_transport` is a thin layer between http requests and the core.
## Routes
 * `/auth`
   - `POST` - Attempts to login using the credentials provided. 
   Returns 200 with a session cookie if logging in was successful, will otherwise return 401.
   - `GET` - Returns 200 if the user is logged in, will otherwise return 401.
   - `DELETE`- Logs the user out. Returns 200 if the user is logged in, will otherwise return 401.
   - `/signup/`
     - `POST` - Creates an account with the provided credentials.
   
## Security Info
### Passwords
Passwords are slated and hashed by [argonautica](https://github.com/bcmyers/argonautica). Can be seen [here](https://github.com/ocboogie/authful-actix-web/blob/65dc764da1d966a6bf21f5ca9daab71558187441/core/src/utils/auth.rs#L16).
### Sessions
Sessions are managed by the server, and are identified by a 32 character string containing a-z, A-Z and 0-9,
generated [here](https://github.com/ocboogie/authful-actix-web/blob/65dc764da1d966a6bf21f5ca9daab71558187441/core/src/services/auth.rs#L74). 
Session ids are hashed, using Sha256, to be saved in the database.
