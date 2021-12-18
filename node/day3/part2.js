const input = require('fs').readFileSync('input.txt', 'utf-8').split('\n');

const mostCommonBit = (data, position) => {
    const ones = data.filter(x => x[position] === '1').length;
    return ones >= data.length / 2 ? '1' : '0';
}

const oxygenFilter = (data, position) => {
    const mcb = mostCommonBit(data, position);
    return data.filter(x => x[position] === mcb);
}

const co2Filter = (data, position) => {
    const mcb = mostCommonBit(data, position);
    return data.filter(x => x[position] !== mcb);
}

const test = (filter) => {
    let data = [...input];
    let index = 0;
    while (data.length > 1) {
        data = filter(data, index);
        index++;
    }
    return parseInt(data[0], 2);
}

const oValue = test(oxygenFilter);
const cValue = test(co2Filter);

console.log(oValue, cValue, oValue * cValue);