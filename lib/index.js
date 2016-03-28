var nat = require('../native')



module.exports.leftPad = function(str, len, ch) {
  if (!ch && ch !== 0) ch = ' '

  return nat.leftPad(str, len , ch)
}
