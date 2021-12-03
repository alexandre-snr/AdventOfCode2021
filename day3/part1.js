const input = require('fs').readFileSync('input.txt', 'utf-8').split('\n');
const bitsCount = input[0].length;

let gamma = '';
let epsilon = '';

for (let i = 0; i < bitsCount; i++) {
    const ones = input.filter(x => x[i] === '1').length;
    gamma += ones > input.length / 2 ? '1' : '0';
    epsilon += ones < input.length / 2 ? '1' : '0';
}
gamma = parseInt(gamma, 2);
epsilon = parseInt(epsilon, 2);

console.log(gamma, epsilon, gamma * epsilon);