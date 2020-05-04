require('dotenv').config()
const cloudinary = require('cloudinary').v2

cloudinary.uploader
  .destroy('shell', {
    invalidate: true,
    type: 'upload',
    resource_type: 'image'
  })
  .then(result => {
    console.log(result)
    cloudinary.uploader
      .upload('https://res.cloudinary.com/cloudinary-training/image/upload/dolphin.jpg', {
        public_id: 'dolphin',
        overwrite: true,
        invalidate: true
      })
      .then(result => {
        const url = result.secure_url
        console.log(url)
      })
      .catch(error => console.error(error))
  })
  .catch(error => console.error(error))
