const input = require('fs').readFileSync('input.txt', 'utf8').split('\n').map(e => 
    e.split(' ').map(i => i.split('').sort().join('')).join(' ').split(/ [|] /g)
);

let totalSum = 0;
let arr = new Array(10).fill('');
input.forEach(line => {
    let firstPart = line[0].split(" ");
    for (let i = 0; i < firstPart.length; i++) {
        let res = null;
        if (firstPart[i].length === 2) res = 1;
        else if (firstPart[i].length === 4) res = 4;
        else if (firstPart[i].length === 3) res = 7;
        else if (firstPart[i].length === 7) res = 8;
        if (res !== null) {
            arr[res] = firstPart[i];
            firstPart[i] = "";
        }
    }
    for (let i = 0; i < firstPart.length; i++) {
        let mistakes = 0;
        if (firstPart[i].length === 5)
            firstPart[i].split('').map(ch => !arr[7].includes(ch) && mistakes++);
        if (mistakes === 2) {
            arr[3] = firstPart[i];
            firstPart[i] = "";
            break;;
        }
    }
    for (let i = 0; i < firstPart.length; i++) {
        let mistakes = 0;
        if (firstPart[i].length === 6)
            firstPart[i].split('').map(ch => !arr[3].includes(ch) && mistakes++);
        if (mistakes === 1) {
            arr[9] = firstPart[i];
            firstPart[i] = "";
            break;
        }
    }
    for (let i = 0; i < firstPart.length; i++) {
        let mistakes = 0
        if (firstPart[i].length === 6)
            firstPart[i].split('').map(ch => !arr[7].includes(ch) && mistakes++)
        if (mistakes === 3) {
            arr[0] = firstPart[i];
            firstPart[i] = "";
        } else if (mistakes === 4) {
            arr[6] = firstPart[i];
            firstPart[i] = "";
        }
    }
    for (let i = 0; i < firstPart.length; i++) {
        let mistakes = 0;
        if (firstPart[i].length === 5)
            firstPart[i].split('').map(ch => !arr[9].includes(ch) && mistakes++);
        if (mistakes === 1) {
            arr[2] = firstPart[i];
            firstPart[i] = "";
            break;
        }
    }
    for (let i = 0; i < firstPart.length; i++) {
        if (firstPart[i].length === 5) {
            arr[5] = firstPart[i];
            firstPart[i] = "";
            break;
        }
    }
    result = "";
    let secondPart = line[1].split(" ")
    secondPart.forEach(seq =>  {
        seq = seq.split('').sort().join('');
        result += (arr.indexOf(seq));
    });
    totalSum += parseInt(result, 10);
});
console.log(totalSum);