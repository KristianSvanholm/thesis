"use strict";
/* The Computer Language Benchmarks Game
   http://benchmarksgame.alioth.debian.org/
   contributed by Isaac Gouy
*/
/// <reference path="../node_modules/@types/node/index.d.ts" />
function approximate(n) {
    let u = Array(n), v = Array(n);
    for (let i = 0; i < n; ++i) {
        u[i] = 1.0;
    }
    for (let i = 0; i < 10; ++i) {
        multiplyAtAv(n, u, v);
        multiplyAtAv(n, v, u);
    }
    let vBv = 0.0, vv = 0.0;
    for (let i = 0; i < 10; ++i) {
        vBv += u[i] * v[i];
        vv += v[i] * v[i];
    }
    return Math.sqrt(vBv / vv);
}
function a(i, j) {
    return 1.0 / ((i + j) * ((i + j) + 1) / 2 + i + 1);
}
function multiplyAv(n, v, av) {
    for (let i = 0; i < n - 1; ++i) {
        av[i] = 0.0;
        for (let j = 0; j < n - 1; ++j) {
            av[i] += a(i, j) * v[j];
        }
    }
}
function multiplyAtv(n, v, atv) {
    for (let i = 0; i < n - 1; ++i) {
        atv[i] = 0.0;
        for (let j = 0; j < n - 1; ++j) {
            atv[i] += a(j, i) * v[j];
        }
    }
}
function multiplyAtAv(n, v, atAv) {
    let u = new Array(n);
    multiplyAv(n, v, u);
    multiplyAtv(n, u, atAv);
}
console.log(approximate(+process.argv[2]).toFixed(9));
