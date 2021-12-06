let input = require('fs').readFileSync('input.txt', 'utf-8').split(',').map(x => parseInt(x, 10));

for (let i = 0; i < 80; i++) {
    input = input.map(x => x - 1);

    let newBorns = 0;
    input = input.map(x => {
        if (x < 0) {
            newBorns++;
            return 6;
        }
        return x;
    });
    const newBornsArr = Array(newBorns).fill(8);
    input = [...input, ...newBornsArr];
    console.log(input.join(','));
}
console.log(input);
console.log(input.length);