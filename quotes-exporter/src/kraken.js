const axios = require('axios');
const { OP_BUY, OP_SELL } = require('./consts');

async function getPrice(asset, op) {
    const url = `https://api.kraken.com/0/public/Depth?pair=${asset}`;
    const { data } = await axios.get(url);
    const key = Object.keys(data.result)[0];
    try {
        switch (op) {
            case OP_BUY:
                return data.result[key].asks[0][0];
            case OP_SELL:
                return data.result[key].bids[0][0];
        }
    } catch (e) {
        console.error(e);
    }
}

module.exports = {
    getPrice,
};