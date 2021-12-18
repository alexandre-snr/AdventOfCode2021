const input = require('fs').readFileSync('input.txt', 'utf-8').split('\n').map(x => {
    const split = x.split('|').map(y => y.trim().split(' '));
    return {
        input: split[0],
        output: split[1],
    };
});

let sum = 0;
for (let i = 0; i < input.length; i++) {
    for (let j = 0; j < input[i].output.length; j++) {
        if (input[i].output[j].length == 2 || input[i].output[j].length == 4 ||
            input[i].output[j].length == 3 || input[i].output[j].length == 7)
            sum++;
    }
}

console.log(sum);