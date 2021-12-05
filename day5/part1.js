const parsePos = pos => pos.split(',').map(x => parseInt(x, 10));

const lines = require('fs').readFileSync('input.txt', 'utf-8').split('\n').filter(x => x.length > 0).map(line => {
    const [start, end] = line.split('->').map(val => {
        const [x, y] = parsePos(val.trim());
        return {x, y};
    });
    if (start.x > end.x)
        [start.x, end.x] = [end.x, start.x];
    if (start.y > end.y)
            [start.y, end.y] = [end.y, start.y];
    return {start, end};
}).filter(l => (l.start.x === l.end.x || l.start.y === l.end.y));

const max = {x: 0, y: 0};
lines.forEach((line) => {
    if (line.start.x > max.x) max.x = line.start.x;
    if (line.start.y > max.y) max.y = line.start.y;
    if (line.end.x > max.x) max.x = line.end.x;
    if (line.end.y > max.y) max.y = line.end.y;
});

const map = new Array(max.y + 1).fill(0).map(() => new Array(max.x + 1).fill(0));
lines.forEach((line) => {
    for (let y = line.start.y; y <= line.end.y; y++) {
        for (let x = line.start.x; x <= line.end.x; x++) {
            map[y][x]++;
        }
    }
});

for (let y = 0; y <= max.y; y++) {
    console.log(map[y].map(x => x.toString()).join(''));
}

const twoOrMore = map.reduce((acc, val) => (
    acc + val.reduce((acc1, val1) => acc1 + (val1 >= 2 ? 1 : 0), 0)
), 0);
console.log(twoOrMore);