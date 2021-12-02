const commands = require('fs').readFileSync('input.txt', 'utf-8').split('\n')
    .map(line => line.split(' '))
    .map(mov => ({
        type: mov[0],
        distance: parseInt(mov[1])
    }));

const res = commands.reduce((acc, val) => ({
    h: acc.h + (val.type === 'forward' ? val.distance : 0),
    d: acc.d -
        (val.type === 'up' ? val.distance : 0) +
        (val.type === 'down' ? val.distance : 0)
}), {
    h: 0,
    d: 0,
});

console.log(commands);

const {h, d} = res;
console.log(h * d);