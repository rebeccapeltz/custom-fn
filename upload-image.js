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
      .upload('https://images.pexels.com/photos/1618606/pexels-photo-1618606.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=750&w=1260', {
        public_id: 'turtle',
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
