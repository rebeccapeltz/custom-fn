// upload a small video such as a customer might upload from an iphone
require('dotenv').config()
const cloudinary = require('cloudinary').v2
const open = require('open')

cloudinary.uploader.upload('./assets/images/tiger-lilly.jpg', {
  public_id: 'tiger-lilly',
  overwrite: true,
  invalidate: true,
})
  .then(result => {
    console.log(result)
    open(result.secure_url)
  })
  .catch(error => {
    console.log(error)
  })
