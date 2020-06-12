# d-chess backend

Backend implementation for the d-chess holochain app.

## Running two players

To run two players with which to play against each other, run three terminals, all inside the `nix-shell`:

1. Terminal 1:

```
sim2h_server
```

2. Terminal 2:

```
cd agents/alice
holochain -c conductor-config.toml
```

3. Terminal 3:

```
cd agents/bob
holochain -c conductor-config.toml
```

Now, the agents are ready to be connected with the frontend at https://github.com/eyss/d-chess-front.