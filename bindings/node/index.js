const { platform, arch } = process
module.exports = require(`./index.${platform}-${arch}.node`)
