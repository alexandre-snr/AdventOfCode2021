let input = require('fs').readFileSync('input.txt', 'utf-8').split(',').map(x => parseInt(x, 10));
let fishs = {};
for (let i = -1; i <= 8; i++)
    fishs[i] = 0;
input.forEach(fish => {
    fishs[fish]++;
});

const getNext = (input) => {
    const next = {};

    for (let i = 0; i <= 8; i++)
        next[i - 1] = input[i];
    next[8] = 0;
    next[6] += next[-1];
    next[8] += next[-1];
    next[-1] = undefined;

    return next;
}

for (let i = 0; i < 256; i++) {
    fishs = getNext(fishs);
}

let sum = 0;
for (let i = 0; i <= 8; i++) {
    sum += fishs[i];
}
console.log(sum);