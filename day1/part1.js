const input = require('fs').readFileSync('input.txt', 'utf-8').split('\n').map(x => parseInt(x));

const increases = input.reduce((acc, val, index) => (acc + (val < input[index + 1])), 0);
console.log(increases);