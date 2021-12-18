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

let lowPoints = [];
for (let x = 0; x < w; x++) {
    for (let y = 0; y < h; y++) {
        const adjacents = getAdjacents(x, y);
        let isLowest = true;
        adjacents.forEach(adjacent => {
            if (input[adjacent.y][adjacent.x] <= input[y][x])
                isLowest = false;
        });

        if (isLowest) {
            lowPoints = [...lowPoints, { x, y, val:  input[y][x]}];
        }
    }
}

console.log(lowPoints);
const sum = lowPoints.reduce((acc, val) => acc + parseInt(val.val, 10) + 1, 0);
console.log(sum);