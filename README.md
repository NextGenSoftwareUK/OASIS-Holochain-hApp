# OASIS Holochain hApp

This is the rust based hApp (Holochain App) that forms the DAL (Data Access Layer) to the HoloOASIS OASIS Provider.

## Build Instructions

The toolchain is pinned by `flake.lock`. From the repository root run:

```bash
nix develop . --command npm run build:happ
```

The Git-backed `.` flake is intentional: do not use `nix develop path:.` in a
working tree containing `target` or `node_modules`, because Nix will copy those
directories into its store before evaluating the flake.

## How To Run

To run simply call the RUN command from the root of this repo/dir.

You may alternatively manually run it following these instructions: \
https://github.com/holochain-open-dev/wiki/wiki/Installing-Holochain--&-Building-hApps-Natively-On-Windows

## Running In Nix Shell

Alternatively if you want to run in a nix shell, follow the instructions below:

## Environment Setup

> PREREQUISITE: set up the [holochain development environment](https://developer.holochain.org/docs/install/).

Enter the pinned development shell from the repository root:

```bash
nix develop .
```

Run the remaining commands in that shell.

## Running 2 agents
 
```bash
npm start
```

This will create a network of 2 nodes connected to each other and their respective UIs.
It will also bring up the Holochain Playground for advanced introspection of the conductors.

## Running the backend tests

```bash
npm test
```

This runs integrity validation unit tests, builds and packs the Holochain 0.7
DNA/hApp, and then runs the real-conductor Sweettest suite. The older JavaScript
Tryorama harness is retained only as migration reference: Tryorama 0.19 targets
Holochain 0.6 and is not the release gate for this Holochain 0.7 hApp.

## Bootstrapping a network

Create a custom network of nodes connected to each other and their respective UIs with:

```bash
AGENTS=3 npm run network
```

Substitute the "3" for the number of nodes that you want to bootstrap in your network.
This will also bring up the Holochain Playground for advanced introspection of the conductors.

## Packaging

To package the web happ:
``` bash
npm run package
```

You'll have the `oasis.webhapp` in `workdir`. This is what you should distribute so that the Holochain Launcher can install it.
You will also have its subcomponent `oasis.happ` in the same folder`.

## Documentation

This repository is using these tools:
- [NPM Workspaces](https://docs.npmjs.com/cli/v7/using-npm/workspaces/): npm v7's built-in monorepo capabilities.
- [hc](https://github.com/holochain/holochain/tree/develop/crates/hc): Holochain CLI to easily manage Holochain development instances.
- [Holochain Sweettest](https://docs.rs/holochain/latest/holochain/sweettest/index.html): real-conductor test framework and release gate.
- [@holochain/client](https://www.npmjs.com/package/@holochain/client): client library to connect to Holochain from the UI.
- [@holochain-playground/cli](https://www.npmjs.com/package/@holochain-playground/cli): introspection tooling to understand what's going on in the Holochain nodes.
