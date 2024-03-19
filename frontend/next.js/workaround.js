const fs = require('node:fs');

function exitHandler(options) {
    console.log("Running exit handler")
    // if (fs.readdirSync('.').includes('node_modules')) {

    //     fs.rmSync('node_modules', {recursive: true});
    // }
}

process.on('beforeExit', exitHandler.bind(null,{cleanup:true}));
