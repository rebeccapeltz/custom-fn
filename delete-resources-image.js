require("dotenv").config();
const cloudinary = require("cloudinary").v2;

cloudinary.api
  .delete_resources(["tree"])
  .then((result) => {
    console.log(result);
  })
  .catch((error) => console.error(error));
