const parsePos = pos => pos.split(',').map(x => parseInt(x, 10));

const lines = require('fs').readFileSync('input.txt', 'utf-8').split('\n').filter(x => x.length > 0).map(line => {
    const [start, end] = line.split('->').map(val => {
        const [x, y] = parsePos(val.trim());
        return {x, y};
    });
    return {start, end};
});

const max = {x: 0, y: 0};
lines.forEach((line) => {
    if (line.start.x > max.x) max.x = line.start.x;
    if (line.start.y > max.y) max.y = line.start.y;
    if (line.end.x > max.x) max.x = line.end.x;
    if (line.end.y > max.y) max.y = line.end.y;
});

const map = new Array(max.y + 1).fill(0).map(() => new Array(max.x + 1).fill(0));
lines.forEach((line) => {
    const startPoint = line.start;
    const endPoint = line.end;
    const step = {
        x: endPoint.x - startPoint.x,
        y: endPoint.y - startPoint.y
    }
    if (step.x < 0) step.x = -1;
    if (step.x > 0) step.x = 1;
    if (step.y < 0) step.y = -1;
    if (step.y > 0) step.y = 1;

    for (let x = startPoint.x, y = startPoint.y; x != endPoint.x || y != endPoint.y; x += step.x, y += step.y)
        map[y][x]++;
    map[endPoint.y][endPoint.x]++;
});

for (let y = 0; y <= max.y; y++) {
    console.log(map[y].map(x => x.toString()).join(''));
}

const twoOrMore = map.reduce((acc, val) => (
    acc + val.reduce((acc1, val1) => acc1 + (val1 >= 2 ? 1 : 0), 0)
), 0);
console.log(twoOrMore);