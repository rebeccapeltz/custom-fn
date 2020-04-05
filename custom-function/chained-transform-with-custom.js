require('dotenv').config()
const cloudinary = require('cloudinary').v2
const open = require('open')

const url = cloudinary.url('tiger-lilly', {
  transformation: [
    {
      custom_function: {
        function_type: 'wasm',
        source: 'wasm_example_blur.wasm'
      }
    },
    { width: 300, height: 300, crop: 'crop' }
  ]
})

console.log(url)
open(url)
