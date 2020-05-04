require('dotenv').config()
const cloudinary = require('cloudinary').v2


    cloudinary.uploader
      .upload('https://images.pexels.com/photos/1102909/pexels-photo-1102909.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=750&w=1260', {
        public_id: 'tree',
        overwrite: true,
        invalidate: true
      })
      .then(result => {
        const url = result.secure_url
        console.log(url)
      })
      .catch(error => console.error(error))

