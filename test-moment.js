const moment = require('moment')

console.log(moment().format('MMMM Do YYYY, h:mm:ss a'))
console.log(moment().format('MMMM Do YYYY, h:mm:ss a').length)
const date = moment().format('MMMM Do YYYY, h:mm:ss a')
console.log(`Date cached: ${date}`.length)