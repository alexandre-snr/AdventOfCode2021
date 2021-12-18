const input = require('fs').readFileSync('input.txt', 'utf-8').split('\n').map(x => parseInt(x));
const slidingSum = i => input.slice(i, i + 3).reduce((acc, val) => acc + val, 0)

const increases = input.reduce((acc, _, index) => (acc + (slidingSum(index) < slidingSum(index + 1))), 0);
console.log(increases);