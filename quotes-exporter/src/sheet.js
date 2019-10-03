const google = require('./google');

/**
 * Converts Promise resolve/reject functions to classic Node.js callback(err, res)
 * @param {*} resolve 
 * @param {*} reject 
 */
function toErrRes(resolve, reject) {
    return (err, res) => {
        if (err) reject(err);
        else resolve(res);
    }
}

class Sheet {
    constructor({id, sheet}) {
        this.id = id;
        this.sheet = sheet;
    }
    async setCell(cell, value) {
        const sheets = await google.sheets();
        return new Promise((resolve, reject) => {
            sheets.spreadsheets.values.update({
                spreadsheetId: this.id, 
                range: `${this.sheet}!${cell}:${cell}`, //
                valueInputOption: 'USER_ENTERED',
                resource: {
                    values: [[value]]
                }
            }, toErrRes(resolve, reject));
        });
    }

}

module.exports = {
    Sheet,
};