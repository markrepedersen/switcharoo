- [Clients](#clients)

# Description
A multi-tenant feature flag service.

Currently, only the Redis backend is supported.
User information, including tenant ID, is stored in a Postgres database. Each tenant is mapped to a Redis node.

# Docker
Everything is Dockerized, so you can run the following command to spin the application up quickly:

#+begin_src bash
docker-compose up
#+end_src

# Clients
Javascript/Node.js: https://github.com/markrepedersen/switcharoo-client-nodejs

# TODO
- Support different backends other than Redis
