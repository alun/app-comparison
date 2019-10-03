const axios = require('axios');
const { OP_BUY, OP_SELL } = require('./consts');

async function getPrice(asset, op) {
    const url = `https://www.cryptofacilities.com/derivatives/api/v3/orderbook?symbol=${asset}`;
    const { data: response } = await axios.get(url);
    try {
        switch (op) {
            case OP_BUY:
                return response.orderBook.asks[0][0];
            case OP_SELL:
                return response.orderBook.bids[0][0];
        }
    } catch (e) {
        console.error(e);
    }
}
module.exports = {
    getPrice,
};