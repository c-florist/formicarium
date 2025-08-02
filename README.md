# Formicarium

![Formicarium App Icon](packages/domain/static/icons/64x64.png)

> noun: formicarium
>
> An ant's nest, especially one in an artificial container for purposes of study.

A real-time simulation of an ant colony, built to visualise the emergent behavior of ants (for fun).

## Getting started

TODO

## Roadmap

### Core simulation
- [ ] **Diverse ant roles:** Introduce different ant types such as `Queen`, `Worker`, and `Soldier` with unique behaviors.
- [ ] **Predators & obstacles:** Add predators and environmental obstacles (e.g., boulders) to create a more dynamic simulation.
- [ ] **Multi-colony support:** Allow multiple ant colonies to coexist and interact within the same world.
- [ ] **Nest interior vs exterior:** Distinguish between the outside world and the nest interior, allowing for complex behaviors like tunneling and brood care.

### User interface & experience
- [ ] **Simulation controls:** Implement UI controls to pause, reset, and adjust the simulation speed.
- [ ] **Pheromone visualization:** Create a visual representation of pheromone trails.

## Project history

What started as a way to test out writing desktop apps using Tauri, turned into a bit of an obsession in creating a realistic and immersive simulation. Whilst Tauri makes things easy to package for desktop, I wanted a way to allow anyone to run the simulation quickly, and so migrated to a WASM-based app.

Below you'll find the old demo for the first iteration of the simulation on desktop.

![Formicarium Application Demo](docs/v1.0_demo.gif)

## Attributions

App icon by [Freepik - Flaticon](https://www.flaticon.com/authors/freepik)
