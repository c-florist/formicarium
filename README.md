# formicarium


> noun: formicarium
>
> an ant's nest, especially one in an artificial container for purposes of study.

## TODO

### Currently working on
1. Tiled map: Create a seamless background for the world view that's bigger than the world itself. 256x256 (16x16).
1. Ant population dynamics: Instead of spawning new ants randomly, nest should keep track of its current population and food and spawn new ants based on a predefined growth rate and how much food is available. When ants return to nest they should deposit their food in the nest's food storage. Ants should only despawn when they've returned to the nest.
1. Add ant health to stats bubbles.
1. World creation with parameters: Allow user to specify parameters (starting number of ants, starting number of food sources, number of obstacles, ant spawn rate, max food sources).
1. Add load progress tracker when assets are loading:
```javascript
// Load with progress tracking
const textures = await Assets.load(['sprite1.png', 'sprite2.png'],
    (progress) => console.log(`Loading: ${Math.round(progress * 100)}%`)
);
```

### Improvements
1. Add a collision system with obstacles, i.e. boulders.
1. Improve pheromone following system so it influences direction to specific food source instead of nearest one.
1. Add `to_nest` pheromones.
1. Visualise pheromone trails.
1. Make the procedural world generation more interesting: More obstacles, more interesting background textures.

### Proposed features
1. Introduce different ant types: `Solder`, `Queen`, `Worker`.
1. Add user interaction with world: Allow for creating new food sources and obstacles with limits (e.g. only 14 obstacles allowed at a time).
1. Distinguish between `outside` and `inside` worlds and allow ants to go inside a nest and dig tunnels, lay eggs, feed the queen.
1. Introduce predators and design a system of interaction with them.
1. Support multiple ant colonies per world: Allow for up to X number of colonies per world, design interactions between them (often ants kill other ant colonies when discovered).

## Attributions to include

* App icon by [Freepik - Flaticon](https://www.flaticon.com/authors/freepik)
