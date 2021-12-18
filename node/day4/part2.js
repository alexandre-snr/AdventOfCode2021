const input = require('fs').readFileSync('input.txt', 'utf-8').split('\n').filter(line => line.length > 0);

const [numbersLine, ...boardsLines] = input;
const numbers = numbersLine.split(',').map(n => parseInt(n, 10));
let numberIndex = 0;

let boards = [...Array(boardsLines.length / 5)].map(e => Array(5));
boardsLines.forEach((line, index) => {
    boards[Math.floor(index / 5)][index % 5] = line.split(' ').filter(x => x.trim().length > 0).map(x => ({
        val: parseInt(x, 10),
        called: false,
    }));
})

const callNumber = () => {
    console.log('call', numbers[numberIndex]);
    boards.forEach(board => {
        board.forEach(line => {
            line.forEach(spot => {
                if (spot.val === numbers[numberIndex])
                    spot.called = true;
            })
        });
    });
    numberIndex++;
}

const checkLine = (line) => line.every(spot => spot.called);
const checkCol = (board, colIndex) => {
    for (let x = 0; x < 5; x++)
        if (!board[x][colIndex].called)
            return false;
    return true;
}

const getWinningBoard = () => {
    let winningBoard = null;
    boards.forEach(board => {
        for (let y = 0; y < 5; y++) {
            if (checkLine(board[y]) || checkCol(board, y))
                winningBoard = board;
        }
    });
    return winningBoard;
}

let lastWinning = null;
let lastCalled = 0;
while (numberIndex < numbers.length) {
    callNumber();
    while (true) {
        const winningBoard = getWinningBoard();
        if (winningBoard !== null) {
            boards = boards.filter(b => b !== winningBoard);
            lastWinning = JSON.parse(JSON.stringify(winningBoard));
            lastCalled = numbers[numberIndex - 1];
        } else
            break;
    }
}

const sumUncalled = (board) => board.reduce((acc, val) => (
    acc + val.reduce((acc1, val1) => acc1 + (val1.called ? 0 : val1.val), 0)
), 0);

const sum = sumUncalled(lastWinning);
console.log(sum, lastCalled, sum * lastCalled);