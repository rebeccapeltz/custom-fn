require('dotenv').config()
const cloudinary = require('cloudinary').v2

const open = require('open')

const url = cloudinary.url('sample.jpg', {
  sign_url: true,
  custom_function: {
    function_type: 'remote',
    source: 'https://secure-caverns-90265.herokuapp.com/api/file'
  }
})
console.log(url)
open(url)
