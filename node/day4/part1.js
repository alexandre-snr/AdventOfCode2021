const input = require('fs').readFileSync('input.txt', 'utf-8').split('\n').filter(line => line.length > 0);

const [numbersLine, ...boardsLines] = input;
const numbers = numbersLine.split(',').map(n => parseInt(n, 10));
let numberIndex = 0;

const boards = [...Array(boardsLines.length / 5)].map(e => Array(5));
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

let winningBoard = null;
while (true) {
    callNumber();
    winningBoard = getWinningBoard();
    if (winningBoard !== null || numberIndex >= numbers.length)
        break;
}

const sumUncalled = (board) => board.reduce((acc, val) => (
    acc + val.reduce((acc1, val1) => acc1 + (val1.called ? 0 : val1.val), 0)
), 0);

const sum = sumUncalled(winningBoard);
const lastCalled = numbers[numberIndex - 1];
console.log(sum * lastCalled);