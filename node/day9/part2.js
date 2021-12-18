const input = require('fs').readFileSync('input.txt', 'utf-8').split('\n').map(x => x.split(''));
const w = input[0].length;
const h = input.length;

const getAdjacents = (x, y) => {
    const vecs = [
        { x: -1, y: 0 },
        { x: 1, y: 0 },
        { x: 0, y: -1 },
        { x: 0, y: 1 },
    ];
    let adjacents = [];

    vecs.forEach(vec => {
        if (input[vec.y + y] !== undefined && input[vec.y + y][vec.x + x] !== undefined)
            adjacents = [...adjacents, { x: vec.x + x, y: vec.y + y }];
    });
    return adjacents;
}

let visited = [];

const lowPointToBasin = (lowPoint) => {
    const vecs = [
        { x: -1, y: 0 },
        { x: 1, y: 0 },
        { x: 0, y: -1 },
        { x: 0, y: 1 },
    ];
    let adjacents = [];

    vecs.forEach(vec => {
        if (input[vec.y + lowPoint.y] !== undefined && input[vec.y + lowPoint.y][vec.x + lowPoint.x] !== undefined &&
            input[vec.y + lowPoint.y][vec.x + lowPoint.x] < 9)
            adjacents = [...adjacents, { x: vec.x + lowPoint.x, y: vec.y + lowPoint.y }];
    });
    adjacents = adjacents.filter(x => {
        for (let i = 0; i < visited.length; i++) 
            if (x.x == visited[i].x && x.y == visited[i].y)
                return false;
        return true;
    });
    if (adjacents.length === 0) {
        return [];
    }
    visited = [...visited, lowPoint];
    let next = [];
    adjacents.forEach(x => next = [...next, ...lowPointToBasin(x)]);
    return [lowPoint, ...adjacents, ...next];
}

const removeDups = (basin) => {
    return basin.filter((thing, index, self) =>
        index === self.findIndex((t) => (
            t.x === thing.x && t.y === thing.y
        ))
    );
};

let lowPoints = [];
for (let x = 0; x < w; x++) {
    for (let y = 0; y < h; y++) {
        const adjacents = getAdjacents(x, y);
        let isLowest = true;
        adjacents.forEach(adjacent => {
            if (input[adjacent.y][adjacent.x] <= input[y][x])
                isLowest = false;
        });

        if (isLowest)
            lowPoints = [...lowPoints, { x, y, val:  input[y][x]}];
    }
}
const basins = lowPoints.map(lowPoint => removeDups(lowPointToBasin(lowPoint, [])));
const lens = basins.map(x => x.length);
lens.sort((a, b) => b - a);

const [a, b, c] = lens;
console.log(lens);
console.log(a * b * c);