require("dotenv").config();
const cloudinary = require("cloudinary").v2;

cloudinary.uploader
  .destroy("horse", {
    invalidate: true,
    type: "upload",
    resource_type: "image",
  })
  .then((result) => {
    console.log(result);
  })
  .catch((error) => console.error(error));
