require('dotenv').config()
const cloudinary = require('cloudinary').v2


    cloudinary.uploader
      .upload('https://images.pexels.com/photos/635499/pexels-photo-635499.jpeg?auto=compress&cs=tinysrgb&dpr=2&w=500', {
        public_id: 'horse',
        overwrite: true,
        invalidate: true
      })
      .then(result => {
        const url = result.secure_url
        console.log(url)
      })
      .catch(error => console.error(error))

