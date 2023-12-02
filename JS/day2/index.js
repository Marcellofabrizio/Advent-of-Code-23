const fs = require("fs");
const input = fs.readFileSync("data2.txt", "utf-8").split("\n");

const gameSets = input.map((l) => l.split(": ")).map((g) => g[1].split("; "));

const dices = {};

gameSets.forEach((g, i) => {
    dices[i + 1] = {
        red: 0,
        green: 0,
        blue: 0,
    };
    g.forEach((d) => {
        d.split(", ").forEach((f) => {
            const roll = f.split(" ");
            const qnt = parseInt(roll[0]);
            const color = roll[1];
            if (qnt > dices[i + 1][color]) {
                dices[i + 1][color] = qnt;
            }
        });
    });
});

const targetDices = {
    red: 12,
    green: 13,
    blue: 14,
};

console.log(dices);

const idxSum = Object.keys(dices).reduce((acc, d) => {
    if (
        dices[d].red <= targetDices.red &&
        dices[d].green <= targetDices.green &&
        dices[d].blue <= targetDices.blue
    ) {
        acc += parseInt(d);
    }

    return acc;
}, 0);

const idxSum2 = Object.keys(dices).reduce((acc, d) => {
    acc += Object.values(dices[d]).reduce((acc, d) => acc * d, 1);
    return acc;
}, 0);

console.log(idxSum, idxSum2);
