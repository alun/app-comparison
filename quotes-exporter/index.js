const futures = require('./src/krakenFutures');
const exchange = require('./src/kraken');
const { Sheet } = require('./src/sheet');
const fs = require('fs');
const { OP_BUY, OP_SELL } = require('./src/consts');
const config = JSON.parse(fs.readFileSync('./config/config.json'));

function promiseTimeout(timeout) {
    return new Promise(resolve => {
        setTimeout(resolve, timeout);
    });
}

const ops = {
    buy: OP_BUY,
    sell: OP_SELL,
}
const apis = {
    kraken: exchange,
    'kraken-futures': futures,
};

async function runConfig() {
    const results = {};
    const data = config.data;
    for (key in data) {
        const [api, asset, op] = key.split(':');
        results[key] = await apis[api].getPrice(asset, ops[op]);
    }

    const [id, sheet] = config.sheet.split(':');
    const s = new Sheet({ id, sheet });
    const cells = [];
    await Promise.all(
        Object.keys(data).reduce((acc, k) => acc.concat(
            data[k].map(cell => {
                cells.push(cell);
                return s.setCell(cell, results[k]);
            })
        ), [])
    );
    console.log(new Date().toISOString(), config.sheet, cells.join(' '), 'updated');
    await promiseTimeout(config.timeout || 5000);
    return await runConfig();
}

runConfig();
