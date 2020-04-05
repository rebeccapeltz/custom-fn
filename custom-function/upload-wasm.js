require('dotenv').config()
const cloudinary = require('cloudinary').v2

cloudinary.uploader.upload('./assets/wasm/wasm_example_blur.wasm',
  {
    use_filename: true,
    unique_filename: false,
    type: 'authenticated',
    resource_type: 'raw'
  })
  .then(result => {
    console.log('public_id', result.public_id)
  })
  .catch(error => {
    console.log(error)
  })

cloudinary.uploader.upload('./assets/wasm/wasm_example_quantize.wasm',
  {
    use_filename: true,
    unique_filename: false,
    type: 'authenticated',
    resource_type: 'raw'
  })
  .then(result => {
    console.log('public id:', result.public_id)
  })
  .catch(error => {
    console.log(error)
  })
