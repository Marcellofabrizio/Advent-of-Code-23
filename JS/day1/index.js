const fs = require("fs");
const input = fs.readFileSync("data1.txt", "utf-8").split("\n");

const digits = {
    one: 1,
    two: 2,
    three: 3,
    four: 4,
    five: 5,
    six: 6,
    seven: 7,
    eight: 8,
    nine: 9,
    1: 1,
    2: 2,
    3: 3,
    4: 4,
    5: 5,
    6: 6,
    7: 7,
    8: 8,
    9: 9,
};

var sum = 0;

input.forEach((l) => {
    console.log(l);
    const firsts = Object.keys(digits)
        .map((d) => {
            return [digits[d], l.indexOf(d)];
        })
        .filter((f) => f[1] >= 0);
    const seconds = Object.keys(digits)
        .map((d) => {
            return [digits[d], l.lastIndexOf(d)];
        })
        .filter((s) => s[1] >= 0);
    const first = firsts.sort((a, b) => a[1] - b[1])[0][0];
    const second = seconds.sort((a, b) => b[1] - a[1])[0][0];

    sum += first * 10 + second;
});

console.log(sum);
