const input = require('fs').readFileSync('input.txt', 'utf-8').split(',').map(x => parseInt(x, 10));

const cost = (data, pos) => data.map(x => Math.abs(pos - x)).reduce((acc, val) => acc + val, 0);

const min = Math.min(...input);
const max = Math.max(...input);

let lowestCost = cost(input, min);
for (let i = min; i < max; i++) {
    const c = cost(input, i);
    if (c < lowestCost)
        lowestCost =  c;
}

console.log(lowestCost);